//! Lab 09 — decode BIP44 paths and derive the selected address.

use std::str::FromStr;

use bip39::Mnemonic;
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};

use crate::error::LabError;
use crate::model::Bip44PathInfo;
use crate::LabResult;

const ORDINAL_WORDS: [&str; 10] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
];

fn ordinal_word(zero_based: u32) -> String {
    match ORDINAL_WORDS.get(zero_based as usize) {
        Some(word) => (*word).to_owned(),
        None => format!("{}th", zero_based + 1),
    }
}

fn hardened_index(child: &ChildNumber) -> LabResult<u32> {
    match child {
        ChildNumber::Hardened { index } => Ok(*index),
        ChildNumber::Normal { .. } => Err(LabError::InvalidPath(
            "expected a hardened path level".to_owned(),
        )),
    }
}

fn normal_index(child: &ChildNumber) -> LabResult<u32> {
    match child {
        ChildNumber::Normal { index } => Ok(*index),
        ChildNumber::Hardened { .. } => Err(LabError::InvalidPath(
            "expected a non-hardened path level".to_owned(),
        )),
    }
}

/// Parse `m / purpose' / coin' / account' / change / index`.
pub fn decode_bip44_path(path: &str) -> LabResult<Bip44PathInfo> {
    // todo!("Lab 09: validate and decode all five BIP44 path levels")
    let derivation_path =
        DerivationPath::from_str(path).map_err(|error| LabError::InvalidPath(error.to_string()))?;
    let levels = derivation_path.as_ref();

    if levels.len() != 5 {
        return Err(LabError::InvalidPath(format!(
            "expected 5 BIP44 path levels, found {}",
            levels.len()
        )));
    }

    Ok(Bip44PathInfo {
        purpose: hardened_index(&levels[0])?,
        coin_type: hardened_index(&levels[1])?,
        account: hardened_index(&levels[2])?,
        change: normal_index(&levels[3])?,
        index: normal_index(&levels[4])?,
    })
}

/// Translate a decoded path into a concise English explanation.
pub fn describe_bip44_path(info: &Bip44PathInfo) -> String {
    // todo!("Lab 09: explain purpose, coin, account, chain, and index")
    let account_word = ordinal_word(info.account);
    let index_word = ordinal_word(info.index);
    let branch_word = if info.change == 0 {
        "receiving"
    } else {
        "change"
    };

    format!(
        "This path uses purpose {purpose}' for BIP44, coin type {coin}' for the network, \
the {account_word} account (zero-based account index {account}), the {branch_word} chain \
(change = {change}), and the {index_word} address (zero-based address index {index}).",
        purpose = info.purpose,
        coin = info.coin_type,
        account_word = account_word,
        account = info.account,
        branch_word = branch_word,
        change = info.change,
        index_word = index_word,
        index = info.index,
    )
}

/// Return the same BIP44 path with only its final address index changed.
pub fn with_address_index(path: &str, new_index: u32) -> LabResult<String> {
    // todo!("Lab 09: preserve the branch and replace only the final child")
    let info = decode_bip44_path(path)?;
    Ok(format!(
        "m/{purpose}'/{coin}'/{account}'/{change}/{index}",
        purpose = info.purpose,
        coin = info.coin_type,
        account = info.account,
        change = info.change,
        index = new_index,
    ))
}

/// Derive the P2PKH address selected by a BIP44 path.
pub fn derive_bip44_address(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<String> {
    // todo!("Lab 09: derive the child key and encode its P2PKH address")
    let parsed = mnemonic
        .parse::<Mnemonic>()
        .map_err(|error| LabError::InvalidMnemonic(error.to_string()))?;
    let seed = parsed.to_seed(passphrase);

    let secp = Secp256k1::new();
    let master = Xpriv::new_master(network, &seed)
        .map_err(|error| LabError::Derivation(error.to_string()))?;
    let derivation_path =
        DerivationPath::from_str(path).map_err(|error| LabError::InvalidPath(error.to_string()))?;
    let child = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|error| LabError::Derivation(error.to_string()))?;

    let public = child.to_priv().public_key(&secp);
    Ok(Address::p2pkh(public, network).to_string())
}
