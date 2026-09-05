use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rfb-wallet", about = "Minimal regtest Bitcoin wallet")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Full create -> fund -> sync -> send -> confirm walkthrough (regtest only)
    Demo,
    /// Sync wallet state, then print balance and UTXO count
    Balance,
    /// Reveal and print the next address
    Address {
        /// Reveal the next internal (change) address instead of external (receive)
        #[arg(long)]
        internal: bool,
    },
    /// Sync the wallet's view of the chain
    Sync,
    /// Build, sign, and broadcast a transaction
    Send {
        /// Recipient address
        #[arg(long)]
        to: String,
        /// Amount to send, in BTC
        #[arg(long)]
        amount: f64,
        /// Fee rate, in sat/vB
        #[arg(long, default_value_t = 2)]
        fee_rate: u64,
    },
    /// Regtest only: mine blocks to a fresh address
    Fund {
        /// Number of blocks to mine
        #[arg(long)]
        blocks: u64,
    },
}
