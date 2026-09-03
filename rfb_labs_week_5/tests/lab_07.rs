use rfb_labs_week_5::labs::lab07_bip39::{
    compare_passphrases, inspect_mnemonic, is_public_test_mnemonic, mnemonic_seed_hex,
};

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn validates_entropy_and_checksum_structure() {
    let report = inspect_mnemonic(MNEMONIC).unwrap();
    assert_eq!(report.word_count, 12);
    assert_eq!(report.entropy_bits, 128);
    assert_eq!(report.checksum_bits, 4);
}

#[test]
fn rejects_an_invalid_checksum() {
    let invalid =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon";
    assert!(inspect_mnemonic(invalid).is_err());
}

#[test]
fn matches_the_published_bip39_seed_vector() {
    let expected = concat!(
        "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e534955",
        "31f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04"
    );
    assert_eq!(mnemonic_seed_hex(MNEMONIC, "TREZOR").unwrap(), expected);
}

#[test]
fn passphrase_selects_a_different_wallet() {
    assert!(is_public_test_mnemonic(MNEMONIC));
    let report = compare_passphrases(MNEMONIC, "class").unwrap();
    assert!(report.seeds_differ);
    assert_ne!(report.empty_passphrase_seed_hex, report.protected_seed_hex);
}
