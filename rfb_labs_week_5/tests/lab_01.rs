use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};
use rfb_labs_week_5::labs::lab01_addresses::{
    expected_prefix, identify_prefix, inspect_address, script_pubkey_hex,
};
use rfb_labs_week_5::model::AddressFormat;

fn regtest_p2pkh() -> Address {
    let secp = Secp256k1::new();
    let secret = SecretKey::from_slice(&[1_u8; 32]).unwrap();
    let public = PublicKey::new(secret.public_key(&secp));
    Address::p2pkh(public, Network::Regtest)
}

#[test]
fn identifies_human_readable_prefixes() {
    assert_eq!(
        identify_prefix("1BoatSLRHtKNngkdXEeobR76b53LETtpyT"),
        AddressFormat::P2pkh
    );
    assert_eq!(
        identify_prefix("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        AddressFormat::P2sh
    );
    assert_eq!(
        identify_prefix("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080"),
        AddressFormat::P2wpkh
    );
    assert_eq!(identify_prefix("bc1pexample"), AddressFormat::P2tr);
}

#[test]
fn maps_regtest_prefixes() {
    assert_eq!(
        expected_prefix(AddressFormat::P2pkh, Network::Regtest),
        Some("m/n")
    );
    assert_eq!(
        expected_prefix(AddressFormat::P2sh, Network::Regtest),
        Some("2")
    );
    assert_eq!(
        expected_prefix(AddressFormat::P2wpkh, Network::Regtest),
        Some("bcrt1q")
    );
    assert_eq!(
        expected_prefix(AddressFormat::P2tr, Network::Regtest),
        Some("bcrt1p")
    );
}

#[test]
fn inspects_a_network_checked_address() {
    let address = regtest_p2pkh();
    let report = inspect_address(&address.to_string(), Network::Regtest).unwrap();

    assert_eq!(report.address, address.to_string());
    assert_eq!(report.network, "regtest");
    assert_eq!(report.format, AddressFormat::P2pkh);
    assert_eq!(
        report.script_pubkey_hex,
        address.script_pubkey().to_hex_string()
    );
}

#[test]
fn rejects_an_address_for_the_wrong_network() {
    let address = regtest_p2pkh();
    assert!(inspect_address(&address.to_string(), Network::Bitcoin).is_err());
    assert!(script_pubkey_hex(&address.to_string(), Network::Bitcoin).is_err());
}
