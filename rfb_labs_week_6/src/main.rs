use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use bitcoin::{Address, Amount, FeeRate};
use bitcoincore_rpc::Client;
use clap::Parser;

mod cli;
mod config;
mod error;
mod keys;
mod node;
mod tx;
mod wallet;

use cli::{Cli, Command};
use config::Config;
use error::WalletError;
use wallet::WalletDb;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    let mnemonic = keys::load_or_generate_mnemonic(config.mnemonic.as_deref())?;
    let descriptors = keys::descriptors_from_mnemonic(&mnemonic, config.network.into())?;
    let (mut w, mut db) =
        wallet::open_or_create_wallet(&config.db_path, &descriptors, config.network)?;
    let rpc_client = Arc::new(node::build_rpc_client(&config.rpc_url, &config.rpc_auth)?);
    let (chain, _) = node::chain_info(&rpc_client)?;

    match cli.command {
        Command::Demo => run_demo(&mut w, &mut db, &rpc_client, &chain, &descriptors)?,
        Command::Balance => {
            node::sync_wallet(&mut w, Arc::clone(&rpc_client), &mut db)?;
            print_balance(&w);
        }
        Command::Address { internal } => {
            let address = if internal {
                wallet::new_change_address(&mut w)
            } else {
                wallet::new_receive_address(&mut w)
            };
            persist(&mut w, &mut db)?;
            println!("{}", address.address);
        }
        Command::Sync => {
            node::sync_wallet(&mut w, Arc::clone(&rpc_client), &mut db)?;
            println!("synced");
        }
        Command::Send {
            to,
            amount,
            fee_rate,
        } => {
            let recipient = Address::from_str(&to)
                .with_context(|| format!("'{to}' is not a valid address"))?
                .require_network(config.network)
                .with_context(|| format!("'{to}' is not valid on {}", config.network))?;
            let amount = Amount::from_btc(amount).context("invalid amount")?;
            let fee_rate = FeeRate::from_sat_per_vb(fee_rate).context("invalid fee rate")?;

            node::sync_wallet(&mut w, Arc::clone(&rpc_client), &mut db)?;
            let txid = tx::send(&mut w, &rpc_client, &mut db, &recipient, amount, fee_rate)?;
            println!("broadcast txid: {txid}");
        }
        Command::Fund { blocks } => {
            anyhow::ensure!(chain == "regtest", "fund is only available on regtest");
            let address = wallet::new_receive_address(&mut w);
            persist(&mut w, &mut db)?;
            let mined = node::fund_wallet_regtest(&rpc_client, &address.address, blocks)?;
            println!("mined {} blocks to {}", mined.len(), address.address);
        }
    }

    Ok(())
}

fn persist(
    w: &mut bdk_wallet::PersistedWallet<WalletDb>,
    db: &mut WalletDb,
) -> Result<(), WalletError> {
    w.persist(db)
        .map_err(|e| WalletError::Persistence(e.to_string()))?;
    Ok(())
}

fn print_balance(w: &bdk_wallet::PersistedWallet<WalletDb>) {
    let balance = wallet::get_balance(w);
    let utxos = wallet::list_utxos(w);
    println!(
        "balance: total={} confirmed={} trusted_pending={}",
        balance.total(),
        balance.confirmed,
        balance.trusted_pending
    );
    println!("utxos: {}", utxos.len());
}

/// Full create -> fund -> sync -> send -> confirm walkthrough, regtest only.
fn run_demo(
    w: &mut bdk_wallet::PersistedWallet<WalletDb>,
    db: &mut WalletDb,
    rpc_client: &Arc<Client>,
    chain: &str,
    descriptors: &keys::Descriptors,
) -> anyhow::Result<()> {
    anyhow::ensure!(chain == "regtest", "demo is only available on regtest");

    println!(
        "external descriptor (public): {}",
        descriptors.external_public
    );
    println!(
        "internal descriptor (public): {}",
        descriptors.internal_public
    );

    let receive = wallet::new_receive_address(w);
    let change = wallet::new_change_address(w);
    println!("receive address (external): {}", receive.address);
    println!("change address (internal):  {}", change.address);
    persist(w, db)?;

    let mined =
        node::fund_wallet_regtest(rpc_client, &receive.address, node::COINBASE_MATURITY + 1)?;
    println!("mined {} blocks to {}", mined.len(), receive.address);

    node::sync_wallet(w, Arc::clone(rpc_client), db)?;
    print_balance(w);

    let balance = wallet::get_balance(w);
    if balance.confirmed == Amount::ZERO {
        println!("no confirmed balance yet, skipping send demo");
        return Ok(());
    }

    let send_to = wallet::new_receive_address(w);
    persist(w, db)?;

    // Hardcoded demo values, not user input: `expect` here can't fail (1.0 and 2 are always in
    // range), unlike everything else in this function that touches the network, disk, or wallet
    // state.
    let demo_amount = Amount::from_btc(1.0).expect("1.0 BTC is always representable");
    let demo_fee_rate = FeeRate::from_sat_per_vb(2).expect("2 sat/vB is always representable");
    let txid = tx::send(
        w,
        rpc_client,
        db,
        &send_to.address,
        demo_amount,
        demo_fee_rate,
    )?;
    println!("broadcast txid: {txid}");

    node::fund_wallet_regtest(rpc_client, &receive.address, 1)?;
    node::sync_wallet(w, Arc::clone(rpc_client), db)?;
    let balance = wallet::get_balance(w);
    println!(
        "balance after 1 confirmation: total={} confirmed={}",
        balance.total(),
        balance.confirmed
    );

    Ok(())
}
