use std::sync::Arc;

mod config;
mod error;
mod keys;
mod node;
mod tx;
mod wallet;

use config::Config;
use error::WalletError;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    let mnemonic = keys::load_or_generate_mnemonic(config.mnemonic.as_deref())?;
    let descriptors = keys::descriptors_from_mnemonic(&mnemonic, config.network.into())?;
    println!(
        "external descriptor (public): {}",
        descriptors.external_public
    );
    println!(
        "internal descriptor (public): {}",
        descriptors.internal_public
    );

    let (mut w, mut db) =
        wallet::open_or_create_wallet(&config.db_path, &descriptors, config.network)?;

    let receive = wallet::new_receive_address(&mut w);
    let change = wallet::new_change_address(&mut w);
    println!("receive address (external): {}", receive.address);
    println!("change address (internal):  {}", change.address);
    w.persist(&mut db)
        .map_err(|e| WalletError::Persistence(e.to_string()))?;

    let rpc_client = Arc::new(node::build_rpc_client(&config.rpc_url, &config.rpc_auth)?);
    let (chain, blocks) = node::chain_info(&rpc_client)?;
    println!("connected to node: chain={chain} blocks={blocks}");

    if chain == "regtest" {
        let mined =
            node::fund_wallet_regtest(&rpc_client, &receive.address, node::COINBASE_MATURITY + 1)?;
        println!("mined {} blocks to {}", mined.len(), receive.address);
    }

    node::sync_wallet(&mut w, Arc::clone(&rpc_client), &mut db)?;
    let balance = wallet::get_balance(&w);
    let utxos = wallet::list_utxos(&w);
    println!(
        "balance: total={} confirmed={} trusted_pending={}",
        balance.total(),
        balance.confirmed,
        balance.trusted_pending
    );
    println!("utxos: {}", utxos.len());

    if chain == "regtest" && balance.confirmed > bitcoin::Amount::ZERO {
        let send_to = wallet::new_receive_address(&mut w);
        w.persist(&mut db)
            .map_err(|e| WalletError::Persistence(e.to_string()))?;

        // Hardcoded demo values, not user input — `expect` here can't fail (1.0 and 2 are
        // always in range), unlike everything else in this file that touches the network, disk,
        // or wallet state.
        let demo_amount = bitcoin::Amount::from_btc(1.0).expect("1.0 BTC is always representable");
        let demo_fee_rate =
            bitcoin::FeeRate::from_sat_per_vb(2).expect("2 sat/vB is always representable");
        let txid = tx::send(
            &mut w,
            &rpc_client,
            &mut db,
            &send_to.address,
            demo_amount,
            demo_fee_rate,
        )?;
        println!("broadcast txid: {txid}");

        node::fund_wallet_regtest(&rpc_client, &receive.address, 1)?;
        node::sync_wallet(&mut w, Arc::clone(&rpc_client), &mut db)?;
        let balance = wallet::get_balance(&w);
        println!(
            "balance after 1 confirmation: total={} confirmed={}",
            balance.total(),
            balance.confirmed
        );
    } else {
        println!("no confirmed balance yet — skipping send demo");
    }

    Ok(())
}
