use std::fmt;

/// Expected failures produced by transaction validation and coin selection.
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionError {
    NoInputs,
    NoOutputs,
    ZeroValueOutput,
    OutputsExceedInputs {
        total_inputs: u64,
        total_outputs: u64,
    },
    CoinbaseMixedWithRegularInputs,
    MultipleCoinbaseInputs,
    InvalidTxid,
    InsufficientFunds {
        available: u64,
        required: u64,
    },
}

impl fmt::Display for TransactionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(Part 4): return a useful message for every error variant.
        // todo!("implement Display for TransactionError")

        match self {
            TransactionError::NoInputs => {
                write!(formatter, "transaction has no inputs")
            }
            TransactionError::NoOutputs => {
                write!(formatter, "transaction has no outputs")
            }
            TransactionError::ZeroValueOutput => {
                write!(
                    formatter,
                    "a non-OP_RETURN output cannot have a value of zero satoshis"
                )
            }
            TransactionError::OutputsExceedInputs {
                total_inputs,
                total_outputs,
            } => {
                write!(
                    formatter,
                    "outputs exceed inputs: {total_outputs} sats spent against {total_inputs} sats available"
                )
            }
            TransactionError::CoinbaseMixedWithRegularInputs => {
                write!(
                    formatter,
                    "a coinbase input cannot be mixed with regular inputs"
                )
            }
            TransactionError::MultipleCoinbaseInputs => {
                write!(
                    formatter,
                    "a transaction can hold at most one coinbase input"
                )
            }
            TransactionError::InvalidTxid => {
                write!(
                    formatter,
                    "a regular input must reference a non-empty previous transaction id"
                )
            }
            TransactionError::InsufficientFunds {
                available,
                required,
            } => {
                write!(
                    formatter,
                    "insufficient funds: {available} sats available but {required} sats required"
                )
            }
        }
    }
}

impl std::error::Error for TransactionError {}
