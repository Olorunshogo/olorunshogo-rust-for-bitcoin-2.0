//! Lab 08 — inspect proof-linked headers and confirmation depth.

use serde_json::Value;

use crate::labs::lab03_maturity::mine_blocks;
use crate::labs::lab07_confirm::transaction_confirmations;
use crate::model::{BlockHeaderEvidence, SecurityReport};
use crate::rpc::{RpcClient, parse_cli_value, required_f64, required_string, required_u64};
use crate::{LabError, LabResult};

/// Number of extra blocks the lab mines to take the payment to six confirmations.
const ADDITIONAL_CONFIRMATIONS: u64 = 5;

/// Decode a block header into the fields used by the lab.
pub fn get_block_header<C: RpcClient>(
    client: &C,
    block_hash: &str,
) -> LabResult<BlockHeaderEvidence> {
    // TODO: call getblockheader with verbose output and decode all model fields.
    // todo!("Lab 08: inspect a block header")

    let raw = client.call(None, "getblockheader", &[block_hash.to_owned()])?;
    let response = parse_cli_value(&raw)?;

    Ok(BlockHeaderEvidence {
        hash: required_string(&response, "hash")?,
        height: required_u64(&response, "height")?,
        // The genesis block has no parent, so this link is genuinely optional.
        previous_block_hash: response
            .get("previousblockhash")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        merkle_root: required_string(&response, "merkleroot")?,
        nonce: required_u64(&response, "nonce")?,
        difficulty: required_f64(&response, "difficulty")?,
        bits: required_string(&response, "bits")?,
        confirmations: response
            .get("confirmations")
            .and_then(Value::as_i64)
            .ok_or(LabError::MissingField("confirmations"))?,
        chainwork: required_string(&response, "chainwork")?,
    })
}

/// Mine an exact number of additional blocks and return their hashes.
pub fn mine_additional_blocks<C: RpcClient>(
    client: &C,
    miner_address: &str,
    count: u64,
) -> LabResult<Vec<String>> {
    // TODO: call generatetoaddress.
    // todo!("Lab 08: mine additional confirmations")
    mine_blocks(client, miner_address, count)
}

/// Read a transaction's confirmation count.
pub fn get_confirmations<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<i64> {
    // TODO: call gettransaction and return confirmations.
    // todo!("Lab 08: read confirmation depth")
    transaction_confirmations(client, wallet_name, txid)
}

/// Record the block header and prove one confirmation becomes six after five blocks.
pub fn build_security_report<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
    block_hash: &str,
    miner_address: &str,
) -> LabResult<SecurityReport> {
    // TODO: read header and initial confirmations, mine five blocks, then read again.
    // todo!("Lab 08: build proof-of-work and confirmation evidence")

    let header = get_block_header(client, block_hash)?;
    let confirmations_before = get_confirmations(client, wallet_name, txid)?;

    mine_additional_blocks(client, miner_address, ADDITIONAL_CONFIRMATIONS)?;

    let confirmations_after = get_confirmations(client, wallet_name, txid)?;

    Ok(SecurityReport {
        header,
        confirmations_before,
        confirmations_after,
    })
    
}
