//! Lab 10 — observe competing branches and most-work convergence.

use crate::model::{ChainTip, ForkSnapshot, ReorgReport};
use crate::rpc::{RpcClient, parse_cli_value, required_string, required_u64};
use crate::LabResult;

/// Read height, best-block hash, and accumulated chainwork from one node.
pub fn get_chain_tip<C: RpcClient>(client: &C) -> LabResult<ChainTip> {
    // TODO: call getblockchaininfo and decode blocks, bestblockhash, and chainwork.
    // todo!("Lab 10: inspect one node's chain tip")
    let raw = client.call(None, "getblockchaininfo", &[])?;
    let response = parse_cli_value(&raw)?;

    Ok(ChainTip {
        height: required_u64(&response, "blocks")?,
        best_block_hash: required_string(&response, "bestblockhash")?,
        chainwork: required_string(&response, "chainwork")?,
    })
}

/// Disconnect a peer by its address.
pub fn disconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    // TODO: call disconnectnode with the peer address.
    // todo!("Lab 10: disconnect competing nodes")
    client.call(None, "disconnectnode", &[peer_address.to_owned()])?;
    Ok(())
}

/// Reconnect a peer for a one-time synchronization attempt.
pub fn reconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    // TODO: call addnode with the address and `onetry`.
    // todo!("Lab 10: reconnect competing nodes")
    client.call(
        None,
        "addnode",
        &[peer_address.to_owned(), "onetry".to_owned()],
    )?;
    Ok(())
}

/// Compare the private competing tips with the final synchronized tips.
pub fn build_reorg_report(
    common_tip_before_split: &str,
    competing_tips: ForkSnapshot,
    final_tips: ForkSnapshot,
) -> ReorgReport {
    // TODO: nodes converge when their final best hashes and heights match.
    // todo!("Lab 10: report most-work-chain convergence")
    
    /*
     * Convergence means both nodes now name the same block as their tip at the same
     * height, which is what the most-work rule forces the shorter branch to accept.
     */
    let converged = final_tips.node_a.best_block_hash == final_tips.node_b.best_block_hash
        && final_tips.node_a.height == final_tips.node_b.height;

    ReorgReport {
        common_tip_before_split: common_tip_before_split.to_owned(),
        competing_tips,
        final_tips,
        converged,
    }
}
