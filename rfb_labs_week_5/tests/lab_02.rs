use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};
use rfb_labs_week_5::labs::lab02_p2pkh::{
    build_p2pkh_script_pubkey, committed_pubkey_hash, derive_p2pkh_address, p2pkh_spend_template,
};

fn public_key() -> PublicKey {
    let secp = Secp256k1::new();
    let secret = SecretKey::from_slice(&[2_u8; 32]).unwrap();
    PublicKey::new(secret.public_key(&secp))
}

#[test]
fn derives_the_expected_p2pkh_address() {
    let public = public_key();
    let expected = Address::p2pkh(public, Network::Bitcoin).to_string();
    assert_eq!(
        derive_p2pkh_address(&public.to_string(), Network::Bitcoin).unwrap(),
        expected
    );
}

#[test]
fn builds_the_standard_p2pkh_lock() {
    let public = public_key();
    let expected = Address::p2pkh(public, Network::Bitcoin)
        .script_pubkey()
        .to_hex_string();
    assert_eq!(
        build_p2pkh_script_pubkey(&public.to_string()).unwrap(),
        expected
    );
}

#[test]
fn commits_to_hash160_of_the_public_key() {
    let public = public_key();
    assert_eq!(
        committed_pubkey_hash(&public.to_string()).unwrap(),
        public.pubkey_hash().to_string()
    );
}

#[test]
fn puts_unlocking_data_in_scriptsig() {
    let public = public_key();
    let spend = p2pkh_spend_template("30440220deadbeef01", &public.to_string()).unwrap();
    assert_eq!(
        spend.script_sig_items,
        vec!["30440220deadbeef01".to_owned(), public.to_string()]
    );
    assert!(spend.witness_items.is_empty());
}
