//! Lab 07 — confirm a transaction and prove block membership.

use serde_json::Value;

use crate::labs::lab03_maturity::mine_blocks;
use crate::labs::lab05_mempool::get_raw_mempool;
use crate::model::ConfirmationReport;
use crate::rpc::{RpcClient, parse_cli_value, required_string};
use crate::{LabError, LabResult};

/// Mine exactly one block and return its hash.
pub fn mine_one_block<C: RpcClient>(client: &C, miner_address: &str) -> LabResult<String> {
    // TODO: call generatetoaddress with a count of one.
    // todo!("Lab 07: mine one block")

    let hashes = mine_blocks(client, miner_address, 1)?;

    hashes
        .into_iter()
        .next()
        .ok_or_else(|| LabError::Parse("generatetoaddress returned no block hash".to_owned()))
}

/// Return true only when this node's mempool contains no transactions.
pub fn mempool_is_empty<C: RpcClient>(client: &C) -> LabResult<bool> {
    // TODO: inspect getrawmempool.
    // todo!("Lab 07: check whether the mempool is empty")
    Ok(get_raw_mempool(client)?.is_empty())
}

/// Return a transaction's confirmation count in the selected wallet.
pub fn transaction_confirmations<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<i64> {
    // TODO: call gettransaction and return confirmations.
    // todo!("Lab 07: read transaction confirmations")
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let response = parse_cli_value(&raw)?;

    response
        .get("confirmations")
        .and_then(Value::as_i64)
        .ok_or(LabError::MissingField("confirmations"))
}

/// Mine, locate the transaction's block, and prove that the block contains the TXID.
pub fn confirm_and_locate_transaction<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
    miner_address: &str,
) -> LabResult<ConfirmationReport> {
    // TODO:
    // 1. Mine one block.
    // 2. Check the mempool.
    // 3. Read gettransaction for blockhash and confirmations.
    // 4. Read getblock and verify that its `tx` array contains txid.
    // todo!("Lab 07: prove confirmation and block membership")
    mine_one_block(client, miner_address)?;

    let mempool_empty = mempool_is_empty(client)?;

    
    // One wallet lookup supplies both the depth and the containing block.
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let wallet_view = parse_cli_value(&raw)?;

    let confirmations = wallet_view
        .get("confirmations")
        .and_then(Value::as_i64)
        .ok_or(LabError::MissingField("confirmations"))?;
    let block_hash = required_string(&wallet_view, "blockhash")?;

    let raw_block = client.call(None, "getblock", &[block_hash.clone(), "1".to_owned()])?;
    let block = parse_cli_value(&raw_block)?;

    let transaction_is_in_block = block
        .get("tx")
        .and_then(Value::as_array)
        .ok_or(LabError::MissingField("tx"))?
        .iter()
        .any(|entry| entry.as_str() == Some(txid));

    Ok(ConfirmationReport {
        txid: txid.to_owned(),
        block_hash,
        confirmations,
        mempool_is_empty: mempool_empty,
        transaction_is_in_block,
    })

}
