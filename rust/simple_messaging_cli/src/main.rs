use clap::Parser;
use num_bigint::BigUint;
use std::process::Command;

/// Egyszerű CLI a `simple_messaging.aleo` Leo program `send_message` transitionjéhez.
///
/// Nem csinál kriptót, nem hív közvetlen snarkVM-et,
/// csak egy biztonságos "wrapper":
///   - összegyúrja a paramétereket
///   - megmutatja, milyen `leo execute` parancsot futtatna
///   - opcionálisan tényleg meghívja a Leo CLI-t (`--run`).
#[derive(Parser, Debug)]
#[command(
    name = "simple_messaging_cli",
    version,
    about = "Rust CLI wrapper a 'simple_messaging.aleo' send_message transitionjéhez",
    long_about = None
)]
struct Args {
    /// Feladó Aleo címe (sender)
    #[arg(long)]
    sender: String,

    /// Címzett Aleo címe (recipient / owner)
    #[arg(long)]
    recipient: String,

    /// Üzenet azonosító (Leo field literal, pl. 1field)
    #[arg(long, default_value = "1field")]
    msg_id: String,

    /// Üzenet adat 0. (Leo field literal, ha nincs message, akkor ezt használjuk)
    #[arg(long, default_value = "0field")]
    data0: String,

    /// Üzenet adat 1. (Leo field literal)
    #[arg(long, default_value = "0field")]
    data1: String,

    /// Üzenet adat 2. (Leo field literal)
    #[arg(long, default_value = "0field")]
    data2: String,

    /// Teljes üzenet szövegben. Ha megadod, ezt automatán feldaraboljuk 3 fieldre.
    ///
    /// Max. ~93 byte-ot használunk fel (3 × 31 byte). Ha hosszabb, levágjuk.
    #[arg(long)]
    message: Option<String>,

    /// Aleo private key, amivel a tranzakciót aláírod
    #[arg(long)]
    private_key: String,

    /// Hálózat (testnet, devnet, stb. – a Leo CLI-vel legyen konzisztens)
    #[arg(long, default_value = "testnet")]
    network: String,

    /// Endpoint az Aleo node / explorer API-hoz.
    ///
    /// Ha nincs megadva, a Leo CLI az ENDPOINT env változóból dolgozik
    /// (vagy a saját defaultjaiból).
    #[arg(long)]
    endpoint: Option<String>,

    /// Ha megadod, a program NEM csak kiírja a parancsot, hanem tényleg futtatja is.
    ///
    /// Alapértelmezetten csak "dry-run" történik, hogy biztonságos legyen.
    #[arg(long)]
    run: bool,
}

/// Szövegből (≤31 byte) decimális field literált csinál:
///  - bytes → BigUint (little-endian),
///  - BigUint → decimális string,
///  - "{dec}field"
fn bytes_to_field_decimal_literal(src: &[u8]) -> String {
    if src.is_empty() {
        return "0field".to_string();
    }
    // max 31 byte-ot használunk egy fieldre
    let max = 31;
    let truncated = if src.len() > max { &src[..max] } else { src };
    let n = BigUint::from_bytes_le(truncated);
    format!("{}field", n.to_string())
}

/// Szöveget (UTF-8) 3 darab decimális field literálra bont.
/// Összesen max. 93 byte-ot használ (3 × 31).
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
            "⚠ Figyelem: az üzenet {} byte, levágva {} byte-ra (3 field).",
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

    // Döntés: nyers data0/1/2 legyen, vagy message-ből generált field-ek?
    let (data0, data1, data2, used_message) = if let Some(ref msg) = args.message {
        let (f0, f1, f2) = string_to_3_field_literals(msg);
        (f0, f1, f2, true)
    } else {
        (args.data0.clone(), args.data1.clone(), args.data2.clone(), false)
    };

    // Összerakjuk a `leo execute` parancsot.
    // Szintaxis Leo 3.x alatt:
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

    // Logolható, "emberi" parancssor (a privát kulcsot nem írjuk ki teljesen).
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
        parts.push("APrivateKey…".to_string()); // itt nem logoljuk ki fullban

        if let Some(endpoint) = &args.endpoint {
            parts.push("--endpoint".to_string());
            parts.push(endpoint.clone());
        }

        parts.join(" ")
    };

    println!("👋 simple_messaging_cli – Leo wrapper");
    println!("------------------------------------");
    println!("Feladó (sender):   {}", args.sender);
    println!("Címzett (owner):   {}", args.recipient);
    println!("msg_id:            {}", args.msg_id);
    if used_message {
        println!("Üzenet (message):  {}", args.message.as_deref().unwrap_or(""));
    }
    println!("data0:             {}", data0);
    println!("data1:             {}", data1);
    println!("data2:             {}", data2);
    println!("Network:           {}", args.network);
    if let Some(endpoint) = &args.endpoint {
        println!("Endpoint:          {}", endpoint);
    } else {
        println!("Endpoint:          (Leo CLI default / ENDPOINT env)");
    }
    println!();
    println!("💡 Leo parancs (private key rövidítve a logban):");
    println!("  {}", printable);
    println!();

    if !args.run {
        println!("ℹ Dry-run mód: a parancs NINCS lefuttatva. Adj hozzá `--run` flaget, ha tényleg futtatni szeretnéd.");
        return;
    }

    println!("🚀 Futtatjuk a Leo CLI-t...");

    match cmd.output() {
        Ok(output) => {
            println!("✅ A parancs lefutott. Exit code: {}", output.status);

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
            eprintln!("❌ Hiba a Leo CLI futtatása közben: {err}");
            eprintln!("Ellenőrizd, hogy a `leo` parancs elérhető-e (PATH), és a working directory a Leo program gyökere-e.");
        }
    }
}
