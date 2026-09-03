use rfb_labs_week_5::labs::lab05_compatibility::{
    best_supported_format, can_send_to, compatibility_report, required_encoding,
};
use rfb_labs_week_5::model::{AddressFormat, CompatibilityReport, SenderCapabilities};

fn p2sh_era_wallet() -> SenderCapabilities {
    SenderCapabilities {
        base58_p2pkh: true,
        base58_p2sh: true,
        bech32: false,
        bech32m: false,
    }
}

#[test]
fn older_p2sh_wallet_accepts_wrapped_but_not_native() {
    let wallet = p2sh_era_wallet();
    assert!(can_send_to(wallet, AddressFormat::P2sh));
    assert!(!can_send_to(wallet, AddressFormat::P2wpkh));
}

#[test]
fn builds_the_four_format_map() {
    assert_eq!(
        compatibility_report(p2sh_era_wallet()),
        CompatibilityReport {
            p2pkh: true,
            p2sh_p2wpkh: true,
            p2wpkh: false,
            p2tr: false,
        }
    );
}

#[test]
fn selects_the_most_modern_supported_format() {
    let mut wallet = p2sh_era_wallet();
    assert_eq!(best_supported_format(wallet), Some(AddressFormat::P2sh));
    wallet.bech32 = true;
    assert_eq!(best_supported_format(wallet), Some(AddressFormat::P2wpkh));
    wallet.bech32m = true;
    assert_eq!(best_supported_format(wallet), Some(AddressFormat::P2tr));
}

#[test]
fn names_the_required_human_encoding() {
    assert_eq!(required_encoding(AddressFormat::P2pkh), "Base58Check");
    assert_eq!(required_encoding(AddressFormat::P2sh), "Base58Check");
    assert_eq!(required_encoding(AddressFormat::P2wpkh), "Bech32");
    assert_eq!(required_encoding(AddressFormat::P2tr), "Bech32m");
}
