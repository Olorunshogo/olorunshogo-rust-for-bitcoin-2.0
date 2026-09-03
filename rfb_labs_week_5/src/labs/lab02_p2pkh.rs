//! Lab 02 — construct and explain legacy P2PKH.

use bitcoin::Network;

use crate::model::P2pkhSpendTemplate;
use crate::LabResult;

/// Derive a P2PKH address from a serialized public key.
pub fn derive_p2pkh_address(public_key_hex: &str, network: Network) -> LabResult<String> {
    todo!("Lab 02: hash the public key and encode a P2PKH address")
}

/// Build the P2PKH scriptPubKey for the serialized public key.
pub fn build_p2pkh_script_pubkey(public_key_hex: &str) -> LabResult<String> {
    todo!("Lab 02: build OP_DUP OP_HASH160 <hash> OP_EQUALVERIFY OP_CHECKSIG")
}

/// Return the HASH160 commitment made to the public key.
pub fn committed_pubkey_hash(public_key_hex: &str) -> LabResult<String> {
    todo!("Lab 02: calculate the public-key hash committed by P2PKH")
}

/// Place a signature and public key in the legacy unlocking location.
pub fn p2pkh_spend_template(
    signature_hex: &str,
    public_key_hex: &str,
) -> LabResult<P2pkhSpendTemplate> {
    todo!("Lab 02: model P2PKH ScriptSig items and its empty witness")
}
