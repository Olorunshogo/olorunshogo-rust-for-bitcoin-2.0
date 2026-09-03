use bitcoin::opcodes::all::OP_CHECKMULTISIG;
use bitcoin::script::Builder;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey, ScriptBuf};
use rfb_labs_week_5::labs::lab03_p2sh::{
    build_2_of_3_redeem_script, build_p2sh_script_pubkey, derive_p2sh_address,
    inspect_p2sh_multisig,
};

fn public_keys() -> [PublicKey; 3] {
    let secp = Secp256k1::new();
    [1_u8, 2, 3].map(|byte| {
        let secret = SecretKey::from_slice(&[byte; 32]).unwrap();
        PublicKey::new(secret.public_key(&secp))
    })
}

fn expected_script(keys: &[PublicKey; 3]) -> ScriptBuf {
    Builder::new()
        .push_int(2)
        .push_key(&keys[0])
        .push_key(&keys[1])
        .push_key(&keys[2])
        .push_int(3)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script()
}

#[test]
fn builds_a_two_of_three_redeem_script() {
    let keys = public_keys();
    let inputs = [
        &keys[0].to_string()[..],
        &keys[1].to_string()[..],
        &keys[2].to_string()[..],
    ];
    assert_eq!(
        build_2_of_3_redeem_script(inputs).unwrap(),
        expected_script(&keys).to_hex_string()
    );
}

#[test]
fn derives_the_committed_p2sh_address() {
    let keys = public_keys();
    let script = expected_script(&keys);
    let expected = Address::p2sh(&script, Network::Regtest)
        .unwrap()
        .to_string();
    assert_eq!(
        derive_p2sh_address(&script.to_hex_string(), Network::Regtest).unwrap(),
        expected
    );
}

#[test]
fn builds_the_outer_p2sh_lock() {
    let keys = public_keys();
    let script = expected_script(&keys);
    let expected = Address::p2sh(&script, Network::Regtest)
        .unwrap()
        .script_pubkey()
        .to_hex_string();
    assert_eq!(
        build_p2sh_script_pubkey(&script.to_hex_string()).unwrap(),
        expected
    );
}

#[test]
fn reports_both_validation_layers() {
    let keys = public_keys();
    let inputs = [
        &keys[0].to_string()[..],
        &keys[1].to_string()[..],
        &keys[2].to_string()[..],
    ];
    let report = inspect_p2sh_multisig(inputs, Network::Regtest).unwrap();
    assert!(report.address.starts_with('2'));
    assert_eq!(
        report.redeem_script_hex,
        expected_script(&keys).to_hex_string()
    );
    assert!(report.script_pubkey_hex.starts_with("a914"));
}
