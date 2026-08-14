//! Lab 05 — broadcast a transaction and observe the mempool.

use serde_json::Value;

use crate::labs::lab03_maturity::{attempt_payment, get_balances};
use crate::model::{MempoolObservation, WalletBalances, WalletTransactionStatus};
use crate::rpc::{parse_cli_value, required_f64, required_string, RpcClient};
use crate::{LabError, LabResult};

/// Send bitcoin from one wallet and return the TXID.
pub fn send_btc<C: RpcClient>(
    client: &C,
    from_wallet: &str,
    destination: &str,
    amount_btc: f64,
) -> LabResult<String> {
    // TODO: call sendtoaddress in the sender's wallet context.
    // todo!("Lab 05: send bitcoin")
    attempt_payment(client, from_wallet, destination, amount_btc)
}

/// Return the node's local mempool as a list of TXIDs.
pub fn get_raw_mempool<C: RpcClient>(client: &C) -> LabResult<Vec<String>> {
    // TODO: call getrawmempool and decode its array.
    // todo!("Lab 05: inspect the local mempool")
    let raw = client.call(None, "getrawmempool", &[])?;
    Ok(serde_json::from_str(&raw)?)
}

/// Return the selected wallet's view of one transaction.
pub fn get_transaction_status<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<WalletTransactionStatus> {
    // TODO: call gettransaction and decode txid, amount, fee, confirmations, and blockhash.
    // todo!("Lab 05: inspect wallet transaction status")
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let response = parse_cli_value(&raw)?;

    Ok(WalletTransactionStatus {
        txid: required_string(&response, "txid")?,
        confirmations: response
            .get("confirmations")
            .and_then(Value::as_i64)
            .ok_or(LabError::MissingField("confirmations"))?,
        amount: required_f64(&response, "amount")?,
        fee: response.get("fee").and_then(Value::as_f64),
        block_hash: response
            .get("blockhash")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
    })
}

/// Send a payment without mining and capture its mempool and receiver-wallet state.
pub fn observe_unconfirmed_payment<C: RpcClient>(
    client: &C,
    sender_wallet: &str,
    receiver_wallet: &str,
    receiver_address: &str,
    amount_btc: f64,
) -> LabResult<MempoolObservation> {
    // TODO: send, inspect getrawmempool, inspect sender status, and read receiver balances.
    // todo!("Lab 05: prove a payment is broadcast but unconfirmed")
    let txid = send_btc(client, sender_wallet, receiver_address, amount_btc)?;

    let mempool = get_raw_mempool(client)?;
    let mempool_contains_tx = mempool.iter().any(|entry| entry == &txid);

    let sender_status = get_transaction_status(client, sender_wallet, &txid)?;
    let receiver_balance = get_balances(client, receiver_wallet)?;

    Ok(MempoolObservation {
        txid,
        mempool_contains_tx,
        sender_status,
        receiver_balance,
    })
}
