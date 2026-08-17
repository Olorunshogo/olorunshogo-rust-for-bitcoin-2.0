use clap::Parser;
use decodetrx::decode_transaction;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "Transaction decoder")]
#[command(version = "1.0")]
#[command(about = "Bitcoin Transaction decoder", long_about = None)]
struct Cli {
    #[arg(required = true, help = "(string, required) Raw transaction hex")]
    transaction_hex: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match decode_transaction(cli.transaction_hex) {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

// // https://mempool.space/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2
