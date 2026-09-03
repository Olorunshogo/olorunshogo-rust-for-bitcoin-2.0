use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, CompressedPublicKey, Network, PublicKey};
use rfb_labs_week_5::labs::lab04_p2wpkh::{
    build_p2wpkh_script_pubkey, derive_p2wpkh_address, native_spend_template, witness_program,
};

fn compressed_public_key() -> (PublicKey, CompressedPublicKey) {
    let secp = Secp256k1::new();
    let secret = SecretKey::from_slice(&[4_u8; 32]).unwrap();
    let public = PublicKey::new(secret.public_key(&secp));
    let compressed = CompressedPublicKey::try_from(public).unwrap();
    (public, compressed)
}

#[test]
fn derives_a_native_regtest_address() {
    let (public, compressed) = compressed_public_key();
    let expected = Address::p2wpkh(&compressed, Network::Regtest).to_string();
    let actual = derive_p2wpkh_address(&public.to_string(), Network::Regtest).unwrap();
    assert_eq!(actual, expected);
    assert!(actual.starts_with("bcrt1q"));
}

#[test]
fn builds_a_version_zero_witness_lock() {
    let (public, compressed) = compressed_public_key();
    let expected = Address::p2wpkh(&compressed, Network::Regtest)
        .script_pubkey()
        .to_hex_string();
    assert_eq!(
        build_p2wpkh_script_pubkey(&public.to_string()).unwrap(),
        expected
    );
    assert!(expected.starts_with("0014"));
}

#[test]
fn reports_a_twenty_byte_program() {
    let (public, _) = compressed_public_key();
    let report = witness_program(&public.to_string()).unwrap();
    assert_eq!(report.version, 0);
    assert_eq!(report.program_length, 20);
    assert_eq!(report.program_hex.len(), 40);
}

#[test]
fn leaves_scriptsig_empty_and_uses_witness() {
    let (public, _) = compressed_public_key();
    let spend = native_spend_template("30440220cafebabe01", &public.to_string()).unwrap();
    assert!(spend.script_sig_hex.is_empty());
    assert_eq!(
        spend.witness_items,
        vec!["30440220cafebabe01".to_owned(), public.to_string()]
    );
}
