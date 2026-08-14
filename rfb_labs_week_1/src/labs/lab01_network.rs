//! Lab 01 — build and verify a regtest network.

use crate::model::NetworkSnapshot;
use crate::rpc::{parse_cli_value, required_string, RpcClient};
use crate::{LabError, LabResult};

/// Return the active chain reported by `getblockchaininfo`.
pub fn get_chain<C: RpcClient>(client: &C) -> LabResult<String> {
    // TODO: call getblockchaininfo and return its `chain` field.
    let raw = client.call(None, "getblockchaininfo", &[])?;

    let response = parse_cli_value(&raw)?;

    let chain = required_string(&response, "chain")?;
    Ok(chain)
}

/// Return the current height using the node-wide `getblockcount` RPC.
pub fn get_block_height<C: RpcClient>(client: &C) -> LabResult<u64> {
    // TODO: call getblockcount and parse the numeric response.
    let raw = client.call(None, "getblockcount", &[]).unwrap();

    let response = parse_cli_value(&raw)?;
    let val = response.as_u64();
    match val {
        Some(val) => Ok(val),
        _ => Err(LabError::Parse("expected u64".to_string())),
    }
}

/// Return the node's current best-block hash.
pub fn get_best_block_hash<C: RpcClient>(client: &C) -> LabResult<String> {
    let raw = client.call(None, "getbestblockhash", &[])?;
    let response = parse_cli_value(&raw)?;

    match response {
        serde_json::Value::String(hash) => Ok(hash),
        _ => Err(LabError::Parse("Expected String".to_string())),
    }
}

/// Collect the chain, height, and best-block hash into one snapshot.
// pub fn inspect_network<C: RpcClient>(client: &C) -> LabResult<NetworkSnapshot> {
//     // TODO: compose the three functions above and reject non-regtest networks.
//     todo!("Lab 01: build a verified regtest snapshot")
// }
pub fn inspect_network<C: RpcClient>(client: &C) -> LabResult<NetworkSnapshot> {
    let chain = get_chain(client)?;

    if chain != "regtest" {
        return Err(LabError::Parse(format!(
            "Expected regtest network, got {}",
            chain
        )));
    }

    let block_height = get_block_height(client)?;
    let best_block_hash = get_best_block_hash(client)?;
    Ok(NetworkSnapshot {
        chain,
        block_height,
        best_block_hash,
    })
}
