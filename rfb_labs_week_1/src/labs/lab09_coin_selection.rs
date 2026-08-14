//! Lab 09 — force and audit multi-UTXO coin selection.

use crate::labs::lab03_maturity::attempt_payment;
use crate::labs::lab04_utxos::list_unspent;
use crate::labs::lab06_decode::{calculate_fee, decode_verbose_transaction, identify_payment_and_change};
use crate::model::{MultiUtxoAudit, Utxo};
use crate::rpc::RpcClient;
use crate::LabResult;

/// Value of each funding payment, small enough that one alone cannot cover 1 BTC.
const FUNDING_AMOUNT_BTC: f64 = 0.4;

/// Amount Alice spends, which no single funding output can cover on its own.
const COMBINED_PAYMENT_BTC: f64 = 1.0;

/// Send three separate 0.4 BTC funding transactions and return their TXIDs.
pub fn create_three_funding_transactions<C: RpcClient>(
    client: &C,
    miner_wallet: &str,
    alice_address: &str,
) -> LabResult<Vec<String>> {
    // TODO: call sendtoaddress three times, each for 0.4 BTC.
    // todo!("Lab 09: create three separate funding transactions")

    // Three separate sends, not one send with three outputs, so Alice ends up with three independent UTXOs.

    (0..3)
        .map(|_| attempt_payment(client, miner_wallet, alice_address, FUNDING_AMOUNT_BTC))
        .collect()

}

/// Return confirmed UTXOs belonging to the supplied address.
pub fn confirmed_utxos_for_address<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
) -> LabResult<Vec<Utxo>> {
    // TODO: call listunspent and retain confirmed outputs for this address.
    // todo!("Lab 09: locate Alice's confirmed UTXOs")
    let utxos = list_unspent(client, wallet_name)?;

    Ok(utxos
        .into_iter()
        .filter(|utxo| utxo.address.as_deref() == Some(address) && utxo.confirmations > 0)
        .collect())
}

/// Send 1 BTC from Alice and return the new TXID.
pub fn send_combined_payment<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
) -> LabResult<String> {
    // TODO: call sendtoaddress for 1 BTC.
    // todo!("Lab 09: create a spend requiring multiple inputs")
    attempt_payment(
        client,
        alice_wallet,
        receiver_address,
        COMBINED_PAYMENT_BTC,
    )
}

/// Decode Alice's spend and prove that multiple funding UTXOs were combined.
pub fn audit_multi_utxo_spend<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
    funding_utxos: &[Utxo],
) -> LabResult<MultiUtxoAudit> {
    // TODO:
    // 1. Send the 1 BTC payment.
    // 2. Reuse Lab 06 to decode it.
    // 3. Identify payment and change.
    // 4. Calculate fee and input count.
    // 5. Record the funding outpoints.
    // todo!("Lab 09: audit multi-UTXO coin selection")
    let spend_txid = send_combined_payment(client, alice_wallet, receiver_address)?;

    let transaction = decode_verbose_transaction(client, &spend_txid)?;
    let payment_and_change = identify_payment_and_change(&transaction, receiver_address)?;
    let fee = calculate_fee(&transaction)?;

    Ok(MultiUtxoAudit {
        funding_outpoints: funding_utxos.iter().map(Utxo::outpoint).collect(),
        spend_txid,
        spend_input_count: transaction.inputs.len(),
        payment_and_change,
        fee,
    })
}
