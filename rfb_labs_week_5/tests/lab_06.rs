use rfb_labs_week_5::labs::lab06_weight_fees::{
    compare_fees, fee_sats, transaction_weight, virtual_size,
};

#[test]
fn calculates_bip141_weight() {
    assert_eq!(transaction_weight(100, 200).unwrap(), 500);
    assert!(transaction_weight(201, 200).is_err());
}

#[test]
fn rounds_weight_up_to_virtual_bytes() {
    assert_eq!(virtual_size(564), 141);
    assert_eq!(virtual_size(565), 142);
}

#[test]
fn calculates_fee_from_feerate() {
    assert_eq!(fee_sats(141, 50).unwrap(), 7_050);
    assert!(fee_sats(u64::MAX, 2).is_err());
}

#[test]
fn reproduces_the_class_fee_comparison() {
    let report = compare_fees(226, 141, 50).unwrap();
    assert_eq!(report.legacy_fee_sats, 11_300);
    assert_eq!(report.segwit_fee_sats, 7_050);
    assert_eq!(report.savings_sats, 4_250);
}
