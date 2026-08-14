//! Lab 06 — decode a transaction and prove value conservation.

use crate::model::{DecodedInput, DecodedOutput, DecodedTransaction, OutPoint, PaymentAndChange};
use crate::rpc::{parse_cli_value, required_f64, required_string, required_u64, RpcClient};
use crate::{LabError, LabResult};
use serde_json::Value;

/// Number of satoshis in one bitcoin, used to round away binary floating-point drift.
const SATOSHIS_PER_BTC: f64 = 100_000_000.0;

/// Round a BTC amount to the nearest satoshi.
fn round_to_satoshi(amount_btc: f64) -> f64 {
    (amount_btc * SATOSHIS_PER_BTC).round() / SATOSHIS_PER_BTC
}

/// Decode a transaction with enough verbosity to include every spent output's value.
pub fn decode_verbose_transaction<C: RpcClient>(
    client: &C,
    txid: &str,
) -> LabResult<DecodedTransaction> {
    // TODO: call getrawtransaction with verbosity 2 and decode:
    // - txid and vsize
    // - each vin's txid, vout, and prevout.value
    // - each vout's n, value, scriptPubKey.hex, and optional address
    // todo!("Lab 06: decode a verbose raw transaction")
    let raw = client.call(
        None,
        "getrawtransaction",
        &[txid.to_owned(), "2".to_owned()],
    )?;

    let response = parse_cli_value(&raw)?;

    let vin = response
        .get("vin")
        .and_then(Value::as_array)
        .ok_or(LabError::MissingField("vin"))?;

    let inputs = vin
        .iter()
        .map(|entry| {
            let previous_value = entry
                .get("prevout")
                .map(|prevout| required_f64(prevout, "value"))
                .transpose()?
                .ok_or(LabError::MissingField("prevout"))?;

            Ok(DecodedInput {
                previous_output: OutPoint {
                    txid: required_string(entry, "txid")?,
                    vout: required_u64(entry, "vout")? as u32,
                },
                previous_value,
            })
        })
        .collect::<LabResult<Vec<DecodedInput>>>()?;

    let vout = response
        .get("vout")
        .and_then(Value::as_array)
        .ok_or(LabError::MissingField("vout"))?;

    let outputs = vout
        .iter()
        .map(|entry| {
            let script_pub_key = entry
                .get("scriptPubKey")
                .ok_or(LabError::MissingField("scriptPubKey"))?;

            Ok(DecodedOutput {
                vout: required_u64(entry, "n")? as u32,
                value: required_f64(entry, "value")?,
                address: script_pub_key
                    .get("address")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned),
                script_pub_key_hex: required_string(script_pub_key, "hex")?,
            })
        })
        .collect::<LabResult<Vec<DecodedOutput>>>()?;

    Ok(DecodedTransaction {
        txid: required_string(&response, "txid")?,
        inputs,
        outputs,
        vsize: required_u64(&response, "vsize")?,
    })
}

/// Return every previous output consumed by the transaction.
pub fn input_outpoints(transaction: &DecodedTransaction) -> Vec<OutPoint> {
    // TODO: map decoded inputs to their outpoints.
    // todo!("Lab 06: list consumed outpoints")
    transaction
        .inputs
        .iter()
        .map(|input| input.previous_output.clone())
        .collect()
}

/// Identify the receiver payment and optional change output.
pub fn identify_payment_and_change(
    transaction: &DecodedTransaction,
    receiver_address: &str,
) -> LabResult<PaymentAndChange> {
    // TODO: match the receiver address; treat the remaining non-OP_RETURN output as change.
    // todo!("Lab 06: identify payment and change")
    let payment = transaction
        .outputs
        .iter()
        .find(|output| output.address.as_deref() == Some(receiver_address))
        .cloned()
        .ok_or_else(|| {
            LabError::Parse(format!(
                "no output pays the receiver address {receiver_address}"
            ))
        })?;

    let change = transaction
        .outputs
        .iter()
        // OP_RETURN outputs (script prefix 0x6a) carry data, never returned value.
        .find(|output| output.vout != payment.vout && !output.script_pub_key_hex.starts_with("6a"))
        .cloned();

    Ok(PaymentAndChange { payment, change })
}

/// Calculate `sum(inputs) - sum(outputs)`.
pub fn calculate_fee(transaction: &DecodedTransaction) -> LabResult<f64> {
    // TODO: reject impossible negative fees and return the BTC fee.
    // todo!("Lab 06: calculate the miner fee")
    let inputs: f64 = transaction
        .inputs
        .iter()
        .map(|input| input.previous_value)
        .sum();
    let outputs: f64 = transaction.outputs.iter().map(|output| output.value).sum();

    let fee = round_to_satoshi(inputs - outputs);

    if fee < 0.0 {
        return Err(LabError::Parse(format!(
            "outputs exceed inputs by {} BTC",
            -fee
        )));
    }

    Ok(fee)
}
