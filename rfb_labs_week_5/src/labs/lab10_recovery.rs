//! Lab 10 — prove deterministic recovery across BIP44, BIP49, and BIP84.

use std::str::FromStr;

use bip39::Mnemonic;
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, CompressedPublicKey, Network};

use crate::error::LabError;
use crate::model::{AddressFormat, DerivedAddressSet};
use crate::LabResult;

fn derive_child_xpriv(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<Xpriv> {
    let parsed = mnemonic
        .parse::<Mnemonic>()
        .map_err(|error| LabError::InvalidMnemonic(error.to_string()))?;
    let seed = parsed.to_seed(passphrase);

    let secp = Secp256k1::new();
    let master = Xpriv::new_master(network, &seed)
        .map_err(|error| LabError::Derivation(error.to_string()))?;
    let derivation_path =
        DerivationPath::from_str(path).map_err(|error| LabError::InvalidPath(error.to_string()))?;

    master
        .derive_priv(&secp, &derivation_path)
        .map_err(|error| LabError::Derivation(error.to_string()))
}

/// Return the standard BIP44 coin type for a network (0 for mainnet, 1 for every test network).
fn coin_type(network: Network) -> u32 {
    if network == Network::Bitcoin {
        0
    } else {
        1
    }
}

/// Derive one address from an arbitrary full path and selected script family.
pub fn derive_address_for_path(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<String> {
    // todo!("Lab 10: derive one P2PKH, wrapped P2WPKH, or native P2WPKH address")
    let child = derive_child_xpriv(mnemonic, passphrase, path, network)?;
    let secp = Secp256k1::new();
    let public = child.to_priv().public_key(&secp);

    match format {
        AddressFormat::P2pkh => Ok(Address::p2pkh(public, network).to_string()),
        AddressFormat::P2sh => {
            let compressed = CompressedPublicKey::try_from(public)
                .map_err(|error| LabError::InvalidKey(error.to_string()))?;
            Ok(Address::p2shwpkh(&compressed, network).to_string())
        }
        AddressFormat::P2wpkh => {
            let compressed = CompressedPublicKey::try_from(public)
                .map_err(|error| LabError::InvalidKey(error.to_string()))?;
            Ok(Address::p2wpkh(&compressed, network).to_string())
        }
        AddressFormat::P2tr | AddressFormat::Unknown => Err(LabError::InvalidScript(
            "lab 10 only derives P2PKH, P2SH-wrapped P2WPKH, and native P2WPKH".to_owned(),
        )),
    }
}

/// Derive index `n` on the BIP44, BIP49, and BIP84 receive branches.
pub fn derive_address_set(
    mnemonic: &str,
    passphrase: &str,
    account: u32,
    index: u32,
    network: Network,
) -> LabResult<DerivedAddressSet> {
    // todo!("Lab 10: derive three address families from one recovery root")
    let coin = coin_type(network);

    let bip44_p2pkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &format!("m/44'/{coin}'/{account}'/0/{index}"),
        AddressFormat::P2pkh,
        network,
    )?;
    let bip49_p2sh_p2wpkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &format!("m/49'/{coin}'/{account}'/0/{index}"),
        AddressFormat::P2sh,
        network,
    )?;
    let bip84_p2wpkh = derive_address_for_path(
        mnemonic,
        passphrase,
        &format!("m/84'/{coin}'/{account}'/0/{index}"),
        AddressFormat::P2wpkh,
        network,
    )?;

    Ok(DerivedAddressSet {
        bip44_p2pkh,
        bip49_p2sh_p2wpkh,
        bip84_p2wpkh,
    })
}

/// Prove that identical mnemonic, passphrase, path, and network reproduce an address.
pub fn recovery_is_repeatable(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    // todo!("Lab 10: derive twice and compare the results")
    let first = derive_address_for_path(mnemonic, passphrase, path, format, network)?;
    let second = derive_address_for_path(mnemonic, passphrase, path, format, network)?;
    Ok(first == second)
}

/// Prove that changing only the final index selects a different address.
pub fn changing_index_changes_address(
    mnemonic: &str,
    passphrase: &str,
    first_path: &str,
    second_path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    // todo!("Lab 10: compare addresses selected by two child indexes")
    let first = derive_address_for_path(mnemonic, passphrase, first_path, format, network)?;
    let second = derive_address_for_path(mnemonic, passphrase, second_path, format, network)?;
    Ok(first != second)
}
