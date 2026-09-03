//! Lab 09 — decode BIP44 paths and derive the selected address.

use bitcoin::Network;

use crate::model::Bip44PathInfo;
use crate::LabResult;

/// Parse `m / purpose' / coin' / account' / change / index`.
pub fn decode_bip44_path(path: &str) -> LabResult<Bip44PathInfo> {
    todo!("Lab 09: validate and decode all five BIP44 path levels")
}

/// Translate a decoded path into a concise English explanation.
pub fn describe_bip44_path(info: &Bip44PathInfo) -> String {
    todo!("Lab 09: explain purpose, coin, account, chain, and index")
}

/// Return the same BIP44 path with only its final address index changed.
pub fn with_address_index(path: &str, new_index: u32) -> LabResult<String> {
    todo!("Lab 09: preserve the branch and replace only the final child")
}

/// Derive the P2PKH address selected by a BIP44 path.
pub fn derive_bip44_address(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<String> {
    todo!("Lab 09: derive the child key and encode its P2PKH address")
}
