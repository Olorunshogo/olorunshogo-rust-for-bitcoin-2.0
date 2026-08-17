mod cli;
mod hex;
mod serialize;
mod transaction;

use clap::Parser;
use cli::Cli;
use hex::bytes_to_hex;
use serialize::serialize_transaction;
use std::process::ExitCode;

fn main() -> ExitCode {
    let trx = match Cli::parse().into_transaction() {
        Ok(trx) => trx,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let serialized = serialize_transaction(&trx);

    println!("Serialized transaction:");
    println!("{:?}", serialized);
    println!("Serialized Hex transaction:");
    println!("{}", bytes_to_hex(&serialized));

    println!("\nTransaction size: {} bytes", serialized.len());

    ExitCode::SUCCESS
}
