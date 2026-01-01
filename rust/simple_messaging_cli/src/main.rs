use clap::Parser;
use num_bigint::BigUint;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(
    name = "simple_messaging_cli",
    version,
    about = "Rust CLI wrapper for the 'simple_messaging.aleo' send_message transition"
)]
struct Args {
    #[arg(long)]
    sender: String,

    #[arg(long)]
    recipient: String,

    #[arg(long, default_value = "1field")]
    msg_id: String,

    #[arg(long, default_value = "0field")]
    data0: String,

    #[arg(long, default_value = "0field")]
    data1: String,

    #[arg(long, default_value = "0field")]
    data2: String,

    /// Optional plain-text message; will be packed into 3 field literals (31 bytes each)
    #[arg(long)]
    message: Option<String>,

    #[arg(long)]
    private_key: String,

    /// Priority fee passed to Leo as --priority-fees.
    /// Keep as string to support formats like "1000" or "50000|100000|200000".
    #[arg(long, default_value = "1000")]
    priority_fee: String,

    /// Leo network identifier (e.g., "testnet")
    #[arg(long, default_value = "testnet")]
    network: String,

    /// Explorer endpoint base URL (recommended base: https://api.explorer.provable.com/v1).
    /// NOTE: Do NOT append /testnet here if you already pass --network testnet.
    #[arg(long)]
    endpoint: Option<String>,

    /// WSL path to the Leo project root (required), e.g. /home/<user>/simple_messaging
    #[arg(long)]
    leo_dir: String,

    /// If set, actually runs the command (otherwise dry-run)
    #[arg(long)]
    run: bool,

    /// If set, adds --broadcast --yes
    #[arg(long)]
    broadcast: bool,
}

fn bytes_to_field_decimal_literal(src: &[u8]) -> String {
    if src.is_empty() {
        return "0field".to_string();
    }
    let truncated = &src[..src.len().min(31)];
    let n = BigUint::from_bytes_le(truncated);
    format!("{}field", n)
}

fn string_to_3_field_literals(msg: &str) -> (String, String, String) {
    let bytes = msg.as_bytes();
    let truncated = &bytes[..bytes.len().min(93)];

    let f0 = bytes_to_field_decimal_literal(&truncated[0..truncated.len().min(31)]);
    let f1 = if truncated.len() > 31 {
        bytes_to_field_decimal_literal(&truncated[31..truncated.len().min(62)])
    } else {
        "0field".to_string()
    };
    let f2 = if truncated.len() > 62 {
        bytes_to_field_decimal_literal(&truncated[62..truncated.len().min(93)])
    } else {
        "0field".to_string()
    };

    (f0, f1, f2)
}

/// Wrap a value in single-quotes for bash.
/// Note: Aleo keys/URLs normally do not contain single quotes.
fn bash_sq(s: &str) -> String {
    format!("'{}'", s)
}

fn main() {
    let args = Args::parse();

    let (data0, data1, data2) = if let Some(ref msg) = args.message {
        string_to_3_field_literals(msg)
    } else {
        (args.data0.clone(), args.data1.clone(), args.data2.clone())
    };

    // Build the Leo command as a single bash command string.
    // Important: quote values that may contain shell-special characters (notably '|' in priority-fees).
    let mut leo_cmd = format!(
        "cd {leo_dir} && leo execute send_message {sender} {recipient} {msg_id} {data0} {data1} {data2} \
--network {network} --private-key {private_key} --priority-fees {priority_fees}",
        leo_dir = bash_sq(&args.leo_dir),
        sender = args.sender,
        recipient = args.recipient,
        msg_id = args.msg_id,
        data0 = data0,
        data1 = data1,
        data2 = data2,
        network = args.network,
        private_key = bash_sq(&args.private_key),
        priority_fees = bash_sq(&args.priority_fee),
    );

    if let Some(endpoint) = &args.endpoint {
        leo_cmd.push_str(&format!(" --endpoint {}", bash_sq(endpoint)));
    }

    if args.broadcast {
        leo_cmd.push_str(" --broadcast --yes");
    }

    println!("👋 simple_messaging_cli – Leo wrapper");
    println!("------------------------------------");
    println!("Leo dir (WSL):    {}", args.leo_dir);
    println!("Sender:           {}", args.sender);
    println!("Recipient:        {}", args.recipient);
    println!("msg_id:           {}", args.msg_id);
    println!("data0:            {}", data0);
    println!("data1:            {}", data1);
    println!("data2:            {}", data2);
    println!("Network:          {}", args.network);
    println!("Endpoint:         {}", args.endpoint.as_deref().unwrap_or("(none)"));
    println!("Priority fee:     {}", args.priority_fee);
    println!("Broadcast:        {}", args.broadcast);
    println!();
    println!("💡 Leo command:");
    println!("  {}", leo_cmd);
    println!();

    if !args.run {
        println!("ℹ Dry-run mode (use --run to execute).");
        return;
    }

    println!("🚀 Running Leo CLI in WSL...");

    let status = Command::new("wsl")
        .args(["bash", "-lc", &leo_cmd])
        .status()
        .expect("Failed to run WSL / Leo");

    println!("✅ Leo finished with status: {}", status);
}
