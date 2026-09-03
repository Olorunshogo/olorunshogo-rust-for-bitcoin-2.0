use bitcoin::Network;
use rfb_labs_week_5::labs::lab08_bip32::{
    derive_extended_keys, derive_normal_child_xpub, master_xpriv, path_contains_hardened_step,
};

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn creates_a_test_family_master_xpriv() {
    let xpriv = master_xpriv(MNEMONIC, "", Network::Regtest).unwrap();
    assert!(xpriv.starts_with("tprv"));
    assert_eq!(xpriv, master_xpriv(MNEMONIC, "", Network::Regtest).unwrap());
}

#[test]
fn derives_matching_extended_keys() {
    let report = derive_extended_keys(MNEMONIC, "", "m/84'/1'/0'", Network::Regtest).unwrap();
    assert_eq!(report.derivation_path, "m/84'/1'/0'");
    assert!(report.xpriv.starts_with("tprv"));
    assert!(report.xpub.starts_with("tpub"));
}

#[test]
fn xpub_derives_a_normal_public_child() {
    let parent = derive_extended_keys(MNEMONIC, "", "m/84'/1'/0'/0", Network::Regtest).unwrap();
    let child = derive_normal_child_xpub(&parent.xpub, 7).unwrap();
    assert!(child.starts_with("tpub"));
    assert_ne!(child, parent.xpub);
}

#[test]
fn distinguishes_hardened_and_normal_paths() {
    assert!(path_contains_hardened_step("m/44'/0'/0'/0/0").unwrap());
    assert!(!path_contains_hardened_step("m/0/1/2").unwrap());
    assert!(path_contains_hardened_step("not/a/path").is_err());
}
