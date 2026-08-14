//! Lab 03 — demonstrate coinbase maturity.

use crate::labs::lab01_network::get_block_height;
use crate::model::{CoinbaseMaturityReport, WalletBalances};
use crate::rpc::{parse_cli_value, required_f64, RpcClient};
use crate::{LabError, LabResult};

/// Mine `count` blocks to an address and return the generated block hashes.
pub fn mine_blocks<C: RpcClient>(client: &C, address: &str, count: u64) -> LabResult<Vec<String>> {
    // TODO: call generatetoaddress with count and address.
    // todo!("Lab 03: mine blocks")
    let raw = client.call(
        None,
        "generatetoaddress",
        &[count.to_string(), address.to_owned()],
    )?;
    Ok(serde_json::from_str(&raw)?)
}

/// Read the wallet's trusted, untrusted-pending, and immature balances.
pub fn get_balances<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<WalletBalances> {
    // TODO: call getbalances in wallet context and decode the nested `mine` object.
    // todo!("Lab 03: inspect wallet balances")
    let raw = client.call(Some(wallet_name), "getbalances", &[])?;
    let response = parse_cli_value(&raw)?;

    let mine = response.get("mine").ok_or(LabError::MissingField("mine"))?;

    Ok(WalletBalances {
        trusted: required_f64(mine, "trusted")?,
        untrusted_pending: required_f64(mine, "untrusted_pending")?,
        immature: required_f64(mine, "immature")?,
    })
}

/// Attempt a wallet payment and return either its TXID or the Bitcoin Core error.
pub fn attempt_payment<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
    amount_btc: f64,
) -> LabResult<String> {
    // TODO: call sendtoaddress. Do not hide an insufficient-funds RPC error.
    // todo!("Lab 03: attempt a payment")
    let raw = client.call(
        Some(wallet_name),
        "sendtoaddress",
        &[address.to_owned(), amount_btc.to_string()],
    )?;

    let response = parse_cli_value(&raw)?;
    response
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| LabError::Parse("Expected a txid string".to_owned()))
}

/// Mine one block, prove the reward is immature, then mine 100 more blocks.
pub fn demonstrate_coinbase_maturity<C: RpcClient>(
    client: &C,
    miner_wallet: &str,
    miner_address: &str,
    receiver_address: &str,
) -> LabResult<CoinbaseMaturityReport> {
    // TODO:
    // 1. Mine one block.
    // 2. Record height and balances.
    // 3. Attempt a 1 BTC payment and capture its error text.
    // 4. Mine 100 more blocks.
    // 5. Record final height and balances.
    // todo!("Lab 03: produce coinbase-maturity evidence")
    mine_blocks(client, miner_address, 1)?;

    let height_after_first_block = get_block_height(client)?;
    let balance_after_first_block = get_balances(client, miner_wallet)?;

    /*
    This payment must fail as the single coinbase reward is still immature.
    The rejection text is the evidence the lab wants to see at this stage
    */
    let premature_spend_error = match attempt_payment(client, miner_wallet, receiver_address, 1.0) {
        Err(LabError::Rpc(message)) => message,
        Err(other) => return Err(other),
        Ok(txid) => {
            return Err(LabError::Parse(format!(
                "Premature spend succeeded unexpectedly as {txid}"
            )))
        }
    };

    mine_blocks(client, miner_address, 100)?;

    let final_height = get_block_height(client)?;
    let final_balance = get_balances(client, miner_wallet)?;

    Ok(CoinbaseMaturityReport {
        height_after_first_block,
        balance_after_first_block,
        premature_spend_error,
        final_height,
        final_balance,
    })
}
