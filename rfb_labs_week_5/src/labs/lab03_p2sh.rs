//! Lab 03 — wrap a 2-of-3 multisig rule in P2SH.

use bitcoin::Network;

use crate::model::P2shReport;
use crate::LabResult;

/// Build `2 <pub1> <pub2> <pub3> 3 OP_CHECKMULTISIG`.
pub fn build_2_of_3_redeem_script(public_keys: [&str; 3]) -> LabResult<String> {
    todo!("Lab 03: build a canonical 2-of-3 multisig redeemScript")
}

/// Derive the P2SH address that commits to a redeemScript.
pub fn derive_p2sh_address(redeem_script_hex: &str, network: Network) -> LabResult<String> {
    todo!("Lab 03: HASH160 the redeemScript and encode its P2SH address")
}

/// Return the outer `OP_HASH160 <scriptHash> OP_EQUAL` scriptPubKey.
pub fn build_p2sh_script_pubkey(redeem_script_hex: &str) -> LabResult<String> {
    todo!("Lab 03: build the P2SH outer locking script")
}

/// Collect the inner script, outer address, and scriptPubKey in one report.
pub fn inspect_p2sh_multisig(public_keys: [&str; 3], network: Network) -> LabResult<P2shReport> {
    todo!("Lab 03: connect the redeemScript to its P2SH commitment")
}
