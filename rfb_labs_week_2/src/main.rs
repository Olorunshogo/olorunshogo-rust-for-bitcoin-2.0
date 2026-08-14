//! Small executable for Part 8 of the assignment.

use rfb_labs_week_2::{
    find_outputs_for_recipient, highest_value_output, select_utxos, BitcoinValue, InputKind,
    OutPoint, OutputType, Transaction, TxOutput, TxState, Utxo,
};

const PAYMENT: u64 = 90_000;
const FEE: u64 = 2_000;

fn main() {
    // TODO(Part 8): add the two supplied UTXOs as inputs, then add the
    // payment and correctly calculated change outputs.

    let wallet = vec![
        Utxo {
            outpoint: OutPoint {
                txid: "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90".into(),
                vout: 0,
            },
            value: 70_000,
        },
        Utxo {
            outpoint: OutPoint {
                txid: "f0e1d2c3b4a5968778695a4b3c2d1e0ff0e1d2c3b4a5968778695a4b3c2d1e0f".into(),
                vout: 1,
            },
            value: 50_000,
        },
    ];

    let selected = match select_utxos(&wallet, PAYMENT + FEE) {
        Ok(selected) => selected,
        Err(error) => {
            eprintln!("cannot fund the payment: {error}");
            return;
        }
    };

    println!("Selected {} UTXO(s):", selected.len());
    for utxo in &selected {
        println!("  {} -> {} sats", utxo.outpoint, utxo.value);
    }
    println!();

    let mut transaction = Transaction::new(2, 0);

    for utxo in &selected {
        transaction.add_input(InputKind::Regular {
            previous_output: OutPoint {
                txid: utxo.outpoint.txid.clone(),
                vout: utxo.outpoint.vout,
            },
            value: utxo.value,
            sequence: u32::MAX,
        });
    }

    // The change is whatever is left once the payment and the target fee are
    // taken out of the selected inputs.
    let change = transaction.total_input_value() - PAYMENT - FEE;

    transaction.add_output(TxOutput {
        value: PAYMENT,
        recipient: "bc1qreceiver".into(),
        output_type: OutputType::P2wpkh,
    });

    transaction.add_output(TxOutput {
        value: change,
        recipient: "bc1qsender".into(),
        output_type: OutputType::P2wpkh,
    });

    println!("Inputs:");
    for input in &transaction.inputs {
        println!("  {input}");
    }

    println!("Outputs:");
    for output in &transaction.outputs {
        println!("  {output}");
    }
    println!();

    println!("{transaction}");
    println!();

    match transaction.validate() {
        Ok(()) => println!("Validation: ok"),
        Err(error) => println!("Validation: {error}"),
    }

    if let Some(output) = highest_value_output(&transaction) {
        println!("Largest output: {output}");
    }

    let change_outputs = find_outputs_for_recipient(&transaction, "bc1qsender");
    println!(
        "Change back to bc1qsender: {} output(s), {} sats ({:.8} BTC)",
        change_outputs.len(),
        change_outputs
            .iter()
            .map(|output| output.value())
            .sum::<u64>(),
        change_outputs
            .iter()
            .map(|output| output.value_in_btc())
            .sum::<f64>()
    );

    // Part 10: walk the lifecycle far enough to show the guard working.
    let state = TxState::Created
        .transition(TxState::Validated)
        .and_then(|state| state.transition(TxState::Signed))
        .and_then(|state| state.transition(TxState::Broadcast));

    match state {
        Ok(state) => println!("Lifecycle state: {state}"),
        Err(error) => println!("Lifecycle error: {error}"),
    }

    if let Err(error) = TxState::Created.transition(TxState::Broadcast) {
        println!("Rejected transition: {error}");
    }
}
