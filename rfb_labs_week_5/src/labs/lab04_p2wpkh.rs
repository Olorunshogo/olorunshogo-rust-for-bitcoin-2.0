//! Lab 04 — construct and explain native SegWit P2WPKH.

use bitcoin::Network;

use crate::model::{NativeSegwitSpend, WitnessProgramReport};
use crate::LabResult;

/// Derive a native P2WPKH address from a compressed public key.
pub fn derive_p2wpkh_address(public_key_hex: &str, network: Network) -> LabResult<String> {
    todo!("Lab 04: encode a version-0, 20-byte witness program")
}

/// Build the P2WPKH `0 <20-byte-pubkey-hash>` scriptPubKey.
pub fn build_p2wpkh_script_pubkey(public_key_hex: &str) -> LabResult<String> {
    todo!("Lab 04: construct the native SegWit scriptPubKey")
}

/// Report the witness version and program committed by P2WPKH.
pub fn witness_program(public_key_hex: &str) -> LabResult<WitnessProgramReport> {
    todo!("Lab 04: inspect the P2WPKH witness program")
}

/// Put the signature and public key in witness while leaving ScriptSig empty.
pub fn native_spend_template(
    signature_hex: &str,
    public_key_hex: &str,
) -> LabResult<NativeSegwitSpend> {
    todo!("Lab 04: model native P2WPKH unlocking data")
}
