use bitcoin::Network;
use rfb_labs_week_5::labs::lab10_recovery::{
    changing_index_changes_address, derive_address_for_path, derive_address_set,
    recovery_is_repeatable,
};
use rfb_labs_week_5::model::AddressFormat;

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn derives_three_regtest_address_families() {
    let set = derive_address_set(MNEMONIC, "", 0, 0, Network::Regtest).unwrap();
    assert!(set.bip44_p2pkh.starts_with('m') || set.bip44_p2pkh.starts_with('n'));
    assert!(set.bip49_p2sh_p2wpkh.starts_with('2'));
    assert!(set.bip84_p2wpkh.starts_with("bcrt1q"));
}

#[test]
fn identical_recovery_inputs_repeat() {
    assert!(recovery_is_repeatable(
        MNEMONIC,
        "class",
        "m/84'/1'/0'/0/0",
        AddressFormat::P2wpkh,
        Network::Regtest,
    )
    .unwrap());
}

#[test]
fn changing_only_the_index_changes_the_address() {
    assert!(changing_index_changes_address(
        MNEMONIC,
        "",
        "m/84'/1'/0'/0/0",
        "m/84'/1'/0'/0/1",
        AddressFormat::P2wpkh,
        Network::Regtest,
    )
    .unwrap());
}

#[test]
fn format_selection_changes_the_lock_target() {
    let p2pkh = derive_address_for_path(
        MNEMONIC,
        "",
        "m/44'/1'/0'/0/0",
        AddressFormat::P2pkh,
        Network::Regtest,
    )
    .unwrap();
    let p2wpkh = derive_address_for_path(
        MNEMONIC,
        "",
        "m/44'/1'/0'/0/0",
        AddressFormat::P2wpkh,
        Network::Regtest,
    )
    .unwrap();
    assert_ne!(p2pkh, p2wpkh);
}
