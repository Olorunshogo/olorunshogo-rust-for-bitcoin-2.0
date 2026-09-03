//! Lab 08 — derive BIP32 extended private and public keys.

use std::str::FromStr;

use bip39::Mnemonic;
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;

use crate::error::LabError;
use crate::model::ExtendedKeyReport;
use crate::LabResult;

fn seed(mnemonic: &str, passphrase: &str) -> LabResult<[u8; 64]> {
    let parsed = mnemonic
        .parse::<Mnemonic>()
        .map_err(|error| LabError::InvalidMnemonic(error.to_string()))?;
    Ok(parsed.to_seed(passphrase))
}

fn parse_derivation_path(path: &str) -> LabResult<DerivationPath> {
    DerivationPath::from_str(path).map_err(|error| LabError::InvalidPath(error.to_string()))
}

/// Create the master extended private key from a BIP39 recovery setup.
pub fn master_xpriv(mnemonic: &str, passphrase: &str, network: Network) -> LabResult<String> {
    // todo!("Lab 08: derive the seed and create the BIP32 master xpriv")
    let seed = seed(mnemonic, passphrase)?;
    let master = Xpriv::new_master(network, &seed)
        .map_err(|error| LabError::Derivation(error.to_string()))?;
    Ok(master.to_string())
}

/// Derive an extended private/public key pair at a complete path.
pub fn derive_extended_keys(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<ExtendedKeyReport> {
    // todo!("Lab 08: derive an xpriv and neuter it to an xpub")
    let seed = seed(mnemonic, passphrase)?;
    let secp = Secp256k1::new();
    let master = Xpriv::new_master(network, &seed)
        .map_err(|error| LabError::Derivation(error.to_string()))?;
    let derivation_path = parse_derivation_path(path)?;
    let xpriv = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|error| LabError::Derivation(error.to_string()))?;
    let xpub = Xpub::from_priv(&secp, &xpriv);

    Ok(ExtendedKeyReport {
        derivation_path: path.to_owned(),
        xpriv: xpriv.to_string(),
        xpub: xpub.to_string(),
    })
}

/// Derive a normal public child from an xpub without private key material.
pub fn derive_normal_child_xpub(parent_xpub: &str, index: u32) -> LabResult<String> {
    // todo!("Lab 08: derive one non-hardened public child")
    let parent =
        Xpub::from_str(parent_xpub).map_err(|error| LabError::InvalidKey(error.to_string()))?;
    let child_number = ChildNumber::from_normal_idx(index)
        .map_err(|error| LabError::InvalidPath(error.to_string()))?;
    let secp = Secp256k1::new();
    let child = parent
        .derive_pub(&secp, &[child_number])
        .map_err(|error| LabError::Derivation(error.to_string()))?;
    Ok(child.to_string())
}

/// Return whether a textual path contains at least one hardened step.
pub fn path_contains_hardened_step(path: &str) -> LabResult<bool> {
    // todo!("Lab 08: parse the path and inspect its child numbers")
    let derivation_path = parse_derivation_path(path)?;
    Ok(derivation_path
        .as_ref()
        .iter()
        .any(ChildNumber::is_hardened))
}
