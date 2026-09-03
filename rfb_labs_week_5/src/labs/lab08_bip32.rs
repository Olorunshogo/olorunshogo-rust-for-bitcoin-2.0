//! Lab 08 — derive BIP32 extended private and public keys.

use bitcoin::Network;

use crate::model::ExtendedKeyReport;
use crate::LabResult;

/// Create the master extended private key from a BIP39 recovery setup.
pub fn master_xpriv(mnemonic: &str, passphrase: &str, network: Network) -> LabResult<String> {
    todo!("Lab 08: derive the seed and create the BIP32 master xpriv")
}

/// Derive an extended private/public key pair at a complete path.
pub fn derive_extended_keys(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<ExtendedKeyReport> {
    todo!("Lab 08: derive an xpriv and neuter it to an xpub")
}

/// Derive a normal public child from an xpub without private key material.
pub fn derive_normal_child_xpub(parent_xpub: &str, index: u32) -> LabResult<String> {
    todo!("Lab 08: derive one non-hardened public child")
}

/// Return whether a textual path contains at least one hardened step.
pub fn path_contains_hardened_step(path: &str) -> LabResult<bool> {
    todo!("Lab 08: parse the path and inspect its child numbers")
}
