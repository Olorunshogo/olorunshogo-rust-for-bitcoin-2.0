//! Lab 04 — inspect UTXOs and outpoints.

use serde_json::Value;

use crate::model::{OutPoint, Utxo};
use crate::rpc::{parse_cli_value, required_f64, required_string, required_u64, RpcClient};
use crate::{LabError, LabResult};

/// Return all UTXOs tracked by the selected wallet.
pub fn list_unspent<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<Vec<Utxo>> {
    // Call the Bitcoin Core RPC in the wallet context.
    let raw = client.call(Some(wallet_name), "listunspent", &[])?;

    // Parse the JSON returned by the CLI.
    let response = parse_cli_value(&raw)?;

    // The response should be an array of UTXOs.
    let entries = response
        .as_array()
        .ok_or_else(|| LabError::Parse("expected a JSON array of unspent outputs".to_owned()))?;

    // Decode each JSON object into a Utxo and collect them into a Vec<Utxo>.
    entries.iter().map(decode_utxo).collect()
}

pub fn decode_utxo(entry: &Value) -> LabResult<Utxo> {
    Ok(Utxo {
        txid: required_string(entry, "txid")?,
        vout: required_u64(entry, "vout")? as u32,
        address: entry
            .get("address")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        script_pub_key: required_string(entry, "scriptPubKey")?,
        amount: required_f64(entry, "amount")?,
        confirmations: required_u64(entry, "confirmations")?,
        spendable: entry
            .get("spendable")
            .and_then(Value::as_bool)
            .ok_or(LabError::MissingField("spendable"))?,
    })
}

/// Select one spendable UTXO, preferring the one with the most confirmations.selec
pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    // pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    // TODO: filter by spendable and select deterministically.
    // todo!("Lab 04: select a spendable UTXO")
    utxos
        .iter()
        .filter(|utxo| utxo.spendable)
        .max_by_key(|utxo| (utxo.confirmations, utxo.txid.clone(), utxo.vout))
        .cloned()
}

/// Convert a UTXO into its unique `txid:vout` coordinate.
pub fn outpoint(utxo: &Utxo) -> OutPoint {
    // TODO: return the matching outpoint.
    // todo!("Lab 04: construct an outpoint")
    utxo.outpoint()
}

/// Sum only the spendable UTXOs.
pub fn sum_spendable_utxos(utxos: &[Utxo]) -> f64 {
    // TODO: ignore non-spendable entries and sum BTC amounts.
    // todo!("Lab 04: calculate spendable wallet balance")
    utxos
        .iter()
        .filter(|utxo| utxo.spendable)
        .map(|utxo| utxo.amount)
        .sum()
}
