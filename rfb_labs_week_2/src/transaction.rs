use std::fmt;

use crate::error::TransactionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    P2pkh,
    P2wpkh,
    P2tr,
    OpReturn,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TxOutput {
    pub value: u64,
    pub recipient: String,
    pub output_type: OutputType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputKind {
    Regular {
        previous_output: OutPoint,
        value: u64,
        sequence: u32,
    },
    Coinbase {
        block_height: u32,
        reward: u64,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<InputKind>,
    pub outputs: Vec<TxOutput>,
    pub locktime: u32,
}

pub trait BitcoinValue {
    fn value(&self) -> u64;

    fn value_in_btc(&self) -> f64 {
        self.value() as f64 / 100_000_000.0
    }
}

impl Transaction {
    pub fn new(version: i32, locktime: u32) -> Self {
        Self {
            version,
            inputs: Vec::new(),
            outputs: Vec::new(),
            locktime,
        }
    }

    pub fn add_input(&mut self, input: InputKind) {
        // TODO(Part 3): move `input` into the transaction.
        // let _ = input;
        // todo!("add an input")

        // `input` is taken by value, so the caller's binding is moved here and the vector becomes the sole owner.
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: TxOutput) {
        // TODO(Part 3): move `output` into the transaction.
        // let _ = output;
        // todo!("add an output")

        self.outputs.push(output);
    }

    pub fn total_input_value(&self) -> u64 {
        // TODO(Part 3): match both InputKind variants and sum their values.
        // todo!("calculate the total input value")

        self.inputs.iter().map(|input| input.value()).sum()
    }

    pub fn total_output_value(&self) -> u64 {
        // TODO(Part 3): sum the value of every output.
        // todo!("calculate the total output value")

        self.outputs.iter().map(|output| output.value()).sum()
    }

    pub fn fee(&self) -> Result<u64, TransactionError> {
        // TODO(Part 3): checked subtraction must return OutputsExceedInputs.
        // todo!("calculate the fee")

        let total_inputs = self.total_input_value();
        let total_outputs = self.total_output_value();

        total_inputs
            .checked_sub(total_outputs)
            .ok_or(TransactionError::OutputsExceedInputs {
                total_inputs,
                total_outputs,
            })
    }

    pub fn validate(&self) -> Result<(), TransactionError> {
        // TODO(Part 5): apply every validation rule in ASSIGNMENT.md.
        // todo!("validate the transaction")

        if self.inputs.is_empty() {
            return Err(TransactionError::NoInputs);
        }

        if self.outputs.is_empty() {
            return Err(TransactionError::NoOutputs);
        }

        let mut coinbase_count = 0usize;
        let mut regular_count = 0usize;

        for input in &self.inputs {
            match input {
                InputKind::Regular {
                    previous_output, ..
                } => {
                    regular_count += 1;

                    if previous_output.txid.trim().is_empty() {
                        return Err(TransactionError::InvalidTxid);
                    }
                }
                InputKind::Coinbase { .. } => {
                    coinbase_count += 1;
                }
            }
        }

        if coinbase_count > 1 {
            return Err(TransactionError::MultipleCoinbaseInputs);
        }

        if coinbase_count > 0 && regular_count > 0 {
            return Err(TransactionError::CoinbaseMixedWithRegularInputs);
        }

        for output in &self.outputs {
            // OP_RETURN outputs are provably unspendable data carriers, so they
            // are the one case where a zero value is legitimate.
            if output.value == 0 && output.output_type != OutputType::OpReturn {
                return Err(TransactionError::ZeroValueOutput);
            }
        }

        // `?` surfaces OutputsExceedInputs without repeating the comparison.
        self.fee()?;

        Ok(())
    }
}

impl BitcoinValue for TxOutput {
    fn value(&self) -> u64 {
        // TODO(Part 6)
        // todo!("return the output value")

        self.value
    }
}

impl BitcoinValue for InputKind {
    fn value(&self) -> u64 {
        // TODO(Part 6): both variants carry a value under different names.
        // todo!("return the input value")

        match self {
            InputKind::Regular { value, .. } => *value,
            InputKind::Coinbase { reward, .. } => *reward,
        }
    }
}

pub fn highest_value_output(transaction: &Transaction) -> Option<&TxOutput> {
    // TODO(Part 7): borrow from `transaction`; do not clone.
    // let _ = transaction;
    // todo!("find the highest-value output")

    transaction
        .outputs
        .iter()
        .max_by_key(|output| output.value())
}

pub fn find_outputs_for_recipient<'a>(
    transaction: &'a Transaction,
    recipient: &str,
) -> Vec<&'a TxOutput> {
    // TODO(Part 7): return references to all matching outputs.
    // let _ = (transaction, recipient);
    // todo!("find outputs for a recipient")

    // The returned Vec owns only the references, so the outputs themselves stay
    // inside `transaction` and the lifetime `'a` ties them to it.
    transaction
        .outputs
        .iter()
        .filter(|output| output.recipient == recipient)
        .collect()
}

impl fmt::Display for OutPoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(Part 6): format as `<txid>:<vout>`.
        // todo!("display an outpoint")

        write!(formatter, "{}:{}", self.txid, self.vout)
    }
}

impl fmt::Display for TxOutput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(Part 6)
        // todo!("display an output")

        write!(
            formatter,
            "{} sats to {} ({:?})",
            self.value, self.recipient, self.output_type
        )
    }
}

impl fmt::Display for InputKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(Part 6)
        // todo!("display an input")

        match self {
            InputKind::Regular {
                previous_output,
                value,
                sequence,
            } => write!(
                formatter,
                "regular input {previous_output} worth {value} sats (sequence {sequence})"
            ),
            InputKind::Coinbase {
                block_height,
                reward,
            } => write!(
                formatter,
                "coinbase input at height {block_height} worth {reward} sats"
            ),
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(Part 6): print the readable summary described in the assignment.
        // todo!("display a transaction summary")

        writeln!(formatter, "Transaction")?;
        writeln!(formatter, "  version:      {}", self.version)?;
        writeln!(formatter, "  locktime:     {}", self.locktime)?;
        writeln!(formatter, "  inputs:       {}", self.inputs.len())?;
        writeln!(formatter, "  outputs:      {}", self.outputs.len())?;
        writeln!(
            formatter,
            "  total input:  {} sats",
            self.total_input_value()
        )?;
        writeln!(
            formatter,
            "  total output: {} sats",
            self.total_output_value()
        )?;

        // An invalid fee is reported, never unwrapped, so Display cannot panic.
        match self.fee() {
            Ok(fee) => write!(formatter, "  fee:          {fee} sats"),
            Err(error) => write!(formatter, "  fee:          invalid ({error})"),
        }
    }
}
