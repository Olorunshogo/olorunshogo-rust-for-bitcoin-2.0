use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddressFormat {
    P2pkh,
    P2sh,
    P2wpkh,
    P2tr,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressReport {
    pub address: String,
    pub network: String,
    pub format: AddressFormat,
    pub script_pubkey_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct P2pkhSpendTemplate {
    pub script_sig_items: Vec<String>,
    pub witness_items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct P2shReport {
    pub redeem_script_hex: String,
    pub address: String,
    pub script_pubkey_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeSegwitSpend {
    pub script_sig_hex: String,
    pub witness_items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessProgramReport {
    pub version: u8,
    pub program_hex: String,
    pub program_length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SenderCapabilities {
    pub base58_p2pkh: bool,
    pub base58_p2sh: bool,
    pub bech32: bool,
    pub bech32m: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub p2pkh: bool,
    pub p2sh_p2wpkh: bool,
    pub p2wpkh: bool,
    pub p2tr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeeComparison {
    pub legacy_vbytes: u64,
    pub segwit_vbytes: u64,
    pub legacy_fee_sats: u64,
    pub segwit_fee_sats: u64,
    pub savings_sats: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MnemonicReport {
    pub word_count: usize,
    pub entropy_bits: usize,
    pub checksum_bits: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassphraseComparison {
    pub empty_passphrase_seed_hex: String,
    pub protected_seed_hex: String,
    pub seeds_differ: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedKeyReport {
    pub derivation_path: String,
    pub xpriv: String,
    pub xpub: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bip44PathInfo {
    pub purpose: u32,
    pub coin_type: u32,
    pub account: u32,
    pub change: u32,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedAddressSet {
    pub bip44_p2pkh: String,
    pub bip49_p2sh_p2wpkh: String,
    pub bip84_p2wpkh: String,
}
