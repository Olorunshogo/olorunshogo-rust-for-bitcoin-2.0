use crate::{error::TransactionError, transaction::OutPoint};

#[derive(Debug, PartialEq, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: u64,
}

pub fn select_utxos(available_utxos: &[Utxo], target: u64) -> Result<Vec<&Utxo>, TransactionError> {
    // TODO(Part 9): select in slice order until the target is reached. Return
    // borrowed UTXOs and InsufficientFunds when their total is too small.
    // let _ = (available_utxos, target);
    // todo!("select UTXOs")

    let mut selected: Vec<&Utxo> = Vec::new();
    let mut accumulated = 0u64;

    for utxo in available_utxos {
        if accumulated >= target {
            break;
        }

        accumulated += utxo.value;
        selected.push(utxo);
    }

    if accumulated < target {
        return Err(TransactionError::InsufficientFunds {
            available: available_utxos.iter().map(|utxo| utxo.value).sum(),
            required: target,
        });
    }

    Ok(selected)
}
