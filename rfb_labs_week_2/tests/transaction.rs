use rfb_labs_week_2::{
    find_outputs_for_recipient, highest_value_output, BitcoinValue, InputKind, OutPoint,
    OutputType, Transaction, TransactionError, TxOutput, TxState,
};

fn regular_input(value: u64) -> InputKind {
    InputKind::Regular {
        previous_output: OutPoint {
            txid: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            vout: 0,
        },
        value,
        sequence: u32::MAX,
    }
}

fn coinbase_input(reward: u64) -> InputKind {
    InputKind::Coinbase {
        block_height: 840_000,
        reward,
    }
}

fn output(value: u64, recipient: &str) -> TxOutput {
    TxOutput {
        value,
        recipient: recipient.into(),
        output_type: OutputType::P2wpkh,
    }
}

#[test]
fn valid_regular_transaction_passes_validation() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(120_000));
    transaction.add_output(output(90_000, "bc1qreceiver"));
    transaction.add_output(output(28_000, "bc1qsender"));

    assert_eq!(transaction.validate(), Ok(()));
    assert_eq!(transaction.total_input_value(), 120_000);
    assert_eq!(transaction.total_output_value(), 118_000);
    assert_eq!(transaction.fee(), Ok(2_000));
}

#[test]
fn outputs_cannot_exceed_inputs() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(50_000));
    transaction.add_output(output(60_000, "bc1qreceiver"));

    assert_eq!(
        transaction.validate(),
        Err(rfb_labs_week_2::TransactionError::OutputsExceedInputs {
            total_inputs: 50_000,
            total_outputs: 60_000,
        })
    );
}

#[test]
fn totals_sum_across_multiple_inputs_and_outputs() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(70_000));
    transaction.add_input(regular_input(50_000));
    transaction.add_output(output(90_000, "bc1qreceiver"));
    transaction.add_output(output(28_000, "bc1qsender"));

    assert_eq!(transaction.total_input_value(), 120_000);
    assert_eq!(transaction.total_output_value(), 118_000);
    assert_eq!(transaction.fee(), Ok(2_000));
}

#[test]
fn fee_reports_the_error_instead_of_underflowing() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(10_000));
    transaction.add_output(output(25_000, "bc1qreceiver"));

    assert_eq!(
        transaction.fee(),
        Err(TransactionError::OutputsExceedInputs {
            total_inputs: 10_000,
            total_outputs: 25_000,
        })
    );
}

#[test]
fn highest_value_output_borrows_the_largest_output() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(200_000));
    transaction.add_output(output(30_000, "bc1qsender"));
    transaction.add_output(output(150_000, "bc1qreceiver"));

    let largest = highest_value_output(&transaction).expect("transaction has outputs");

    assert_eq!(largest.value, 150_000);
    assert_eq!(largest.recipient, "bc1qreceiver");
    // The transaction still owns its outputs after the borrow.
    assert_eq!(transaction.outputs.len(), 2);
}

#[test]
fn highest_value_output_is_none_without_outputs() {
    let transaction = Transaction::new(2, 0);

    assert!(highest_value_output(&transaction).is_none());
}

#[test]
fn outputs_can_be_filtered_by_recipient() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(200_000));
    transaction.add_output(output(90_000, "bc1qreceiver"));
    transaction.add_output(output(28_000, "bc1qsender"));
    transaction.add_output(output(10_000, "bc1qsender"));

    let change = find_outputs_for_recipient(&transaction, "bc1qsender");

    assert_eq!(change.len(), 2);
    assert_eq!(
        change.iter().map(|output| output.value()).sum::<u64>(),
        38_000
    );
    assert!(find_outputs_for_recipient(&transaction, "bc1qunknown").is_empty());
}

#[test]
fn valid_coinbase_transaction_passes_validation() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(coinbase_input(312_500_000));
    transaction.add_output(output(312_500_000, "bc1qminer"));

    assert_eq!(transaction.validate(), Ok(()));
    assert_eq!(transaction.total_input_value(), 312_500_000);
    assert_eq!(transaction.fee(), Ok(0));
}

#[test]
fn transaction_without_inputs_is_rejected() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_output(output(10_000, "bc1qreceiver"));

    assert_eq!(transaction.validate(), Err(TransactionError::NoInputs));
}

#[test]
fn transaction_without_outputs_is_rejected() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(10_000));

    assert_eq!(transaction.validate(), Err(TransactionError::NoOutputs));
}

#[test]
fn zero_value_output_is_rejected_unless_op_return() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(10_000));
    transaction.add_output(output(0, "bc1qreceiver"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::ZeroValueOutput)
    );

    let mut with_data = Transaction::new(2, 0);
    with_data.add_input(regular_input(10_000));
    with_data.add_output(output(9_000, "bc1qreceiver"));
    with_data.add_output(TxOutput {
        value: 0,
        recipient: "op_return".into(),
        output_type: OutputType::OpReturn,
    });

    assert_eq!(with_data.validate(), Ok(()));
}

#[test]
fn coinbase_cannot_be_mixed_with_regular_inputs() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(coinbase_input(100_000));
    transaction.add_input(regular_input(50_000));
    transaction.add_output(output(120_000, "bc1qminer"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::CoinbaseMixedWithRegularInputs)
    );
}

#[test]
fn multiple_coinbase_inputs_are_rejected() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(coinbase_input(100_000));
    transaction.add_input(coinbase_input(100_000));
    transaction.add_output(output(150_000, "bc1qminer"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::MultipleCoinbaseInputs)
    );
}

#[test]
fn regular_input_with_empty_txid_is_rejected() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(InputKind::Regular {
        previous_output: OutPoint {
            txid: String::new(),
            vout: 0,
        },
        value: 50_000,
        sequence: u32::MAX,
    });
    transaction.add_output(output(40_000, "bc1qreceiver"));

    assert_eq!(transaction.validate(), Err(TransactionError::InvalidTxid));
}

#[test]
fn display_shows_the_required_summary_fields() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(120_000));
    transaction.add_output(output(118_000, "bc1qreceiver"));

    let rendered = transaction.to_string();

    assert!(rendered.contains("version:      2"));
    assert!(rendered.contains("locktime:     0"));
    assert!(rendered.contains("inputs:       1"));
    assert!(rendered.contains("outputs:      1"));
    assert!(rendered.contains("total input:  120000 sats"));
    assert!(rendered.contains("total output: 118000 sats"));
    assert!(rendered.contains("fee:          2000 sats"));
}

#[test]
fn display_reports_an_invalid_fee_without_panicking() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(10_000));
    transaction.add_output(output(25_000, "bc1qreceiver"));

    let rendered = transaction.to_string();

    assert!(rendered.contains("fee:          invalid"));
    assert!(rendered.contains("outputs exceed inputs"));
}

#[test]
fn bitcoin_value_is_shared_by_inputs_and_outputs() {
    assert_eq!(regular_input(70_000).value(), 70_000);
    assert_eq!(coinbase_input(312_500_000).value(), 312_500_000);
    assert_eq!(output(90_000, "bc1qreceiver").value(), 90_000);
    assert_eq!(coinbase_input(100_000_000).value_in_btc(), 1.0);
}

#[test]
fn outpoint_displays_as_txid_and_vout() {
    let outpoint = OutPoint {
        txid: "abcd".into(),
        vout: 3,
    };

    assert_eq!(outpoint.to_string(), "abcd:3");
}

#[test]
fn lifecycle_rejects_invalid_transitions() {
    let signed = TxState::Created
        .transition(TxState::Validated)
        .and_then(|state| state.transition(TxState::Signed))
        .expect("created -> validated -> signed is a valid path");

    assert_eq!(signed, TxState::Signed);
    assert!(TxState::Created.transition(TxState::Broadcast).is_err());
    assert!(TxState::Confirmed.transition(TxState::Signed).is_err());
    assert!(TxState::Confirmed.is_terminal());
}
