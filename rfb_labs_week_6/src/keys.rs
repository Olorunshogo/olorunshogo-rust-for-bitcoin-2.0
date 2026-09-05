use std::str::FromStr;

use bdk_wallet::descriptor::IntoWalletDescriptor;
use bdk_wallet::keys::bip39::{Language, Mnemonic, WordCount};
use bdk_wallet::keys::{GeneratableKey, GeneratedKey};
use bdk_wallet::miniscript::Segwitv0;
use bitcoin::NetworkKind;
use bitcoin::bip32::DerivationPath;
use bitcoin::secp256k1::Secp256k1;

use crate::error::WalletError;

// BIP84 (native segwit) account 0, external/internal keychains.
const EXTERNAL_PATH: &str = "m/84h/1h/0h/0";
const INTERNAL_PATH: &str = "m/84h/1h/0h/1";

pub struct Descriptors {
    // Private-key-embedded descriptors (tprv-based) — used to build a signing wallet. Never print these.
    pub external: String,
    pub internal: String,
    // Public-only equivalents (xpub-based), safe to print or log.
    pub external_public: String,
    pub internal_public: String,
}

// === Mnemonic

/// Parses `existing` as a mnemonic if present, otherwise generates a fresh 12-word one.
pub fn load_or_generate_mnemonic(existing: Option<&str>) -> Result<Mnemonic, WalletError> {
    match existing {
        Some(phrase) => Mnemonic::parse(phrase).map_err(|e| WalletError::Mnemonic(e.to_string())),
        None => {
            let generated: GeneratedKey<Mnemonic, Segwitv0> =
                Mnemonic::generate((WordCount::Words12, Language::English))
                    .map_err(|e| WalletError::Mnemonic(format!("{e:?}")))?;
            let mnemonic = generated.into_key();
            println!("generated a new mnemonic — save it to MNEMONIC in .env, never commit it:");
            println!("{mnemonic}");
            Ok(mnemonic)
        }
    }
}

// === Descriptors

/// Derives the BIP84 external/internal descriptors for `mnemonic` on `network`.
pub fn descriptors_from_mnemonic(
    mnemonic: &Mnemonic,
    network: NetworkKind,
) -> Result<Descriptors, WalletError> {
    let secp = Secp256k1::new();
    // EXTERNAL_PATH/INTERNAL_PATH are compile-time constants, not user input — `expect` is safe
    // here because there's no runtime value that could make this parse fail.
    let external_path = DerivationPath::from_str(EXTERNAL_PATH).expect("EXTERNAL_PATH is valid");
    let internal_path = DerivationPath::from_str(INTERNAL_PATH).expect("INTERNAL_PATH is valid");
    let mnemonic_with_passphrase = (mnemonic.clone(), None::<String>);

    let (external, ext_keymap) =
        bdk_wallet::descriptor!(wpkh((mnemonic_with_passphrase.clone(), external_path)))
            .map_err(|e| WalletError::DescriptorBuild(e.to_string()))?
            .into_wallet_descriptor(&secp, network)
            .map_err(|e| WalletError::DescriptorBuild(e.to_string()))?;
    let (internal, int_keymap) =
        bdk_wallet::descriptor!(wpkh((mnemonic_with_passphrase, internal_path)))
            .map_err(|e| WalletError::DescriptorBuild(e.to_string()))?
            .into_wallet_descriptor(&secp, network)
            .map_err(|e| WalletError::DescriptorBuild(e.to_string()))?;

    // `Descriptor::to_string()` only serializes the public half — the private key material
    // lives in the separate KeyMap, so a wallet built from public-only strings would be
    // watch-only. Embed the private keys with `to_string_with_secret` for the wallet-building
    // descriptors, and keep the plain (xpub-only) form for anything that prints or logs one.
    Ok(Descriptors {
        external: external.to_string_with_secret(&ext_keymap),
        internal: internal.to_string_with_secret(&int_keymap),
        external_public: external.to_string(),
        internal_public: internal.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_a_valid_mnemonic_when_none_is_configured() {
        let mnemonic = load_or_generate_mnemonic(None).unwrap();
        assert_eq!(mnemonic.word_count(), 12);
    }

    #[test]
    fn parses_a_configured_mnemonic() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = load_or_generate_mnemonic(Some(phrase)).unwrap();
        assert_eq!(mnemonic.to_string(), phrase);
    }

    #[test]
    fn external_and_internal_descriptors_are_distinct() {
        let mnemonic = load_or_generate_mnemonic(Some(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        ))
        .unwrap();
        let descriptors = descriptors_from_mnemonic(&mnemonic, NetworkKind::Test).unwrap();
        assert_ne!(descriptors.external, descriptors.internal);
        assert_ne!(descriptors.external_public, descriptors.internal_public);
        assert!(!descriptors.external_public.contains("tprv"));
        assert!(!descriptors.internal_public.contains("tprv"));
    }
}
