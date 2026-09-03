use bitcoin::Network;
use rfb_labs_week_5::labs::lab09_bip44::{
    decode_bip44_path, derive_bip44_address, describe_bip44_path, with_address_index,
};
use rfb_labs_week_5::model::Bip44PathInfo;

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn decodes_every_bip44_level() {
    assert_eq!(
        decode_bip44_path("m/44'/0'/2'/1/5").unwrap(),
        Bip44PathInfo {
            purpose: 44,
            coin_type: 0,
            account: 2,
            change: 1,
            index: 5,
        }
    );
}

#[test]
fn explains_zero_based_account_and_chain() {
    let info = decode_bip44_path("m/44'/0'/2'/1/5").unwrap();
    let description = describe_bip44_path(&info);
    assert!(description.contains("third account"));
    assert!(description.contains("change"));
    assert!(description.contains("sixth address"));
}

#[test]
fn changes_only_the_final_index() {
    assert_eq!(
        with_address_index("m/44'/0'/2'/1/5", 6).unwrap(),
        "m/44'/0'/2'/1/6"
    );
}

#[test]
fn derives_the_selected_bip44_address() {
    let address = derive_bip44_address(MNEMONIC, "", "m/44'/1'/0'/0/0", Network::Regtest).unwrap();
    assert!(address.starts_with('m') || address.starts_with('n'));
    assert_eq!(
        address,
        derive_bip44_address(MNEMONIC, "", "m/44'/1'/0'/0/0", Network::Regtest).unwrap()
    );
}
