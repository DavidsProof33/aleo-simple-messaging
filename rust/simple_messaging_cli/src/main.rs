use clap::Parser;
use num_bigint::BigUint;
use std::process::Command;

/// Simple CLI for the `send_message` transition of the `simple_messaging.aleo` Leo program.
///
/// It does *not* perform any cryptography and does *not* call snarkVM directly.
/// It is only a safe wrapper that:
///   - collects the parameters,
///   - shows which `leo execute` command it would run,
///   - and optionally actually executes the Leo CLI (`--run`).
#[derive(Parser, Debug)]
#[command(
    name = "simple_messaging_cli",
    version,
    about = "Rust CLI wrapper for the 'simple_messaging.aleo' send_message transition",
    long_about = None
)]
struct Args {
    /// Sender Aleo address
    #[arg(long)]
    sender: String,

    /// Recipient Aleo address (record owner)
    #[arg(long)]
    recipient: String,

    /// Message identifier (Leo field literal, e.g. 1field)
    #[arg(long, default_value = "1field")]
    msg_id: String,

    /// Message data 0 (Leo field literal; if no `message` is given, this is used)
    #[arg(long, default_value = "0field")]
    data0: String,

    /// Message data 1 (Leo field literal)
    #[arg(long, default_value = "0field")]
    data1: String,

    /// Message data 2 (Leo field literal)
    #[arg(long, default_value = "0field")]
    data2: String,

    /// Full message as UTF-8 text.  
    /// If provided, it is automatically split into 3 fields.
    ///
    /// We use up to ~93 bytes (3 × 31 bytes). Longer input will be truncated.
    #[arg(long)]
    message: Option<String>,

    /// Aleo private key used to sign the transaction
    #[arg(long)]
    private_key: String,

    /// Network (testnet, devnet, etc. – must match the Leo CLI configuration)
    #[arg(long, default_value = "testnet")]
    network: String,

    /// Endpoint for the Aleo node / explorer API.
    ///
    /// If not provided, the Leo CLI uses the ENDPOINT env variable
    /// (or its own defaults).
    #[arg(long)]
    endpoint: Option<String>,

    /// If set, the program WILL run the Leo command instead of just printing it.
    ///
    /// By default we only perform a dry-run for safety.
    #[arg(long)]
    run: bool,
}

/// Convert a byte slice (≤31 bytes) into a decimal field literal:
///  - bytes → BigUint (little-endian),
///  - BigUint → decimal string,
///  - "{dec}field"
fn bytes_to_field_decimal_literal(src: &[u8]) -> String {
    if src.is_empty() {
        return "0field".to_string();
    }
    // we use at most 31 bytes for a single field
    let max = 31;
    let truncated = if src.len() > max { &src[..max] } else { src };
    let n = BigUint::from_bytes_le(truncated);
    format!("{}field", n.to_string())
}

/// Split a UTF-8 string into 3 decimal field literals.
/// Uses at most 93 bytes total (3 × 31).
fn string_to_3_field_literals(msg: &str) -> (String, String, String) {
    let bytes = msg.as_bytes();
    let max_total = 93;
    let truncated = if bytes.len() > max_total {
        &bytes[..max_total]
    } else {
        bytes
    };

    if bytes.len() > max_total {
        eprintln!(
            "⚠ Warning: message is {} bytes, truncated to {} bytes (3 fields).",
            bytes.len(),
            max_total
        );
    }

    let chunk_size = 31;

    let chunk0 = &truncated[0..truncated.len().min(chunk_size)];
    let chunk1 = if truncated.len() > chunk_size {
        &truncated[chunk_size..truncated.len().min(2 * chunk_size)]
    } else {
        &[][..]
    };
    let chunk2 = if truncated.len() > 2 * chunk_size {
        &truncated[2 * chunk_size..truncated.len().min(3 * chunk_size)]
    } else {
        &[][..]
    };

    let f0 = bytes_to_field_decimal_literal(chunk0);
    let f1 = bytes_to_field_decimal_literal(chunk1);
    let f2 = bytes_to_field_decimal_literal(chunk2);

    (f0, f1, f2)
}

fn main() {
    let args = Args::parse();

    // Decide: use raw data0/1/2, or generate them from `message`?
    let (data0, data1, data2, used_message) = if let Some(ref msg) = args.message {
        let (f0, f1, f2) = string_to_3_field_literals(msg);
        (f0, f1, f2, true)
    } else {
        (args.data0.clone(), args.data1.clone(), args.data2.clone(), false)
    };

    // Build the `leo execute` command.
    // Leo 3.x syntax:
    //   leo execute send_message <sender> <recipient> <msg_id> <data0> <data1> <data2> ...
    let mut cmd = Command::new("leo");

    cmd.arg("execute")
        .arg("send_message")
        .arg(&args.sender)
        .arg(&args.recipient)
        .arg(&args.msg_id)
        .arg(&data0)
        .arg(&data1)
        .arg(&data2)
        .arg("--network")
        .arg(&args.network)
        .arg("--private-key")
        .arg(&args.private_key);

    if let Some(endpoint) = &args.endpoint {
        cmd.arg("--endpoint").arg(endpoint);
    }

    // Human-readable command for logging (private key is not printed in full).
    let printable = {
        let mut parts: Vec<String> = Vec::new();
        parts.push("leo".to_string());
        parts.push("execute".to_string());
        parts.push("send_message".to_string());
        parts.push(args.sender.clone());
        parts.push(args.recipient.clone());
        parts.push(args.msg_id.clone());
        parts.push(data0.clone());
        parts.push(data1.clone());
        parts.push(data2.clone());
        parts.push("--network".to_string());
        parts.push(args.network.clone());
        parts.push("--private-key".to_string());
        parts.push("APrivateKey…".to_string()); // do not log the full private key

        if let Some(endpoint) = &args.endpoint {
            parts.push("--endpoint".to_string());
            parts.push(endpoint.clone());
        }

        parts.join(" ")
    };

    println!("👋 simple_messaging_cli – Leo wrapper");
    println!("------------------------------------");
    println!("Sender:           {}", args.sender);
    println!("Recipient (owner):{}", args.recipient);
    println!("msg_id:           {}", args.msg_id);
    if used_message {
        println!("Message (text):   {}", args.message.as_deref().unwrap_or(""));
    }
    println!("data0:            {}", data0);
    println!("data1:            {}", data1);
    println!("data2:            {}", data2);
    println!("Network:          {}", args.network);
    if let Some(endpoint) = &args.endpoint {
        println!("Endpoint:         {}", endpoint);
    } else {
        println!("Endpoint:         (Leo CLI default / ENDPOINT env)");
    }
    println!();
    println!("💡 Leo command (private key shortened in log):");
    println!("  {}", printable);
    println!();

    if !args.run {
        println!(
            "ℹ Dry-run mode: the command is NOT executed. Add the `--run` flag if you really want to run it."
        );
        return;
    }

    println!("🚀 Running Leo CLI...");

    match cmd.output() {
        Ok(output) => {
            println!("✅ Command finished. Exit code: {}", output.status);

            if !output.stdout.is_empty() {
                println!("--- STDOUT ---");
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }

            if !output.stderr.is_empty() {
                println!("--- STDERR ---");
                print!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(err) => {
            eprintln!("❌ Error while running Leo CLI: {err}");
            eprintln!(
                "Please check that the `leo` command is on PATH and the working directory is the Leo project root."
            );
        }
    }
}
