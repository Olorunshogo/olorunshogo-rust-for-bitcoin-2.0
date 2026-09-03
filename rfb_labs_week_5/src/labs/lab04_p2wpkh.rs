//! Lab 04 — construct and explain native SegWit P2WPKH.

use std::str::FromStr;

use bitcoin::hashes::Hash;
use bitcoin::{Address, CompressedPublicKey, Network, PublicKey};

use crate::error::LabError;
use crate::model::{NativeSegwitSpend, WitnessProgramReport};
use crate::LabResult;

fn parse_compressed_public_key(public_key_hex: &str) -> LabResult<CompressedPublicKey> {
    let public = PublicKey::from_str(public_key_hex)
        .map_err(|error| LabError::InvalidKey(error.to_string()))?;
    CompressedPublicKey::try_from(public).map_err(|error| LabError::InvalidKey(error.to_string()))
}

/// Derive a native P2WPKH address from a compressed public key.
pub fn derive_p2wpkh_address(public_key_hex: &str, network: Network) -> LabResult<String> {
    // todo!("Lab 04: encode a version-0, 20-byte witness program")
    let compressed = parse_compressed_public_key(public_key_hex)?;
    Ok(Address::p2wpkh(&compressed, network).to_string())
}

/// Build the P2WPKH `0 <20-byte-pubkey-hash>` scriptPubKey.
pub fn build_p2wpkh_script_pubkey(public_key_hex: &str) -> LabResult<String> {
    // todo!("Lab 04: construct the native SegWit scriptPubKey")
    let compressed = parse_compressed_public_key(public_key_hex)?;
    let script = Address::p2wpkh(&compressed, Network::Bitcoin).script_pubkey();
    Ok(script.to_hex_string())
}

/// Report the witness version and program committed by P2WPKH.
pub fn witness_program(public_key_hex: &str) -> LabResult<WitnessProgramReport> {
    // todo!("Lab 04: inspect the P2WPKH witness program")
    let compressed = parse_compressed_public_key(public_key_hex)?;
    let hash = compressed.wpubkey_hash();
    let program_hex = hex::encode(hash.to_byte_array());

    Ok(WitnessProgramReport {
        version: 0,
        program_length: program_hex.len() / 2,
        program_hex,
    })
}

/// Put the signature and public key in witness while leaving ScriptSig empty.
pub fn native_spend_template(
    signature_hex: &str,
    public_key_hex: &str,
) -> LabResult<NativeSegwitSpend> {
    // todo!("Lab 04: model native P2WPKH unlocking data")
    hex::decode(signature_hex)
        .map_err(|error| LabError::InvalidScript(format!("invalid signature hex: {error}")))?;
    hex::decode(public_key_hex)
        .map_err(|error| LabError::InvalidKey(format!("invalid public key hex: {error}")))?;

    Ok(NativeSegwitSpend {
        script_sig_hex: String::new(),
        witness_items: vec![signature_hex.to_owned(), public_key_hex.to_owned()],
    })
}
