//! Starter library for the Week 2 Bitcoin transaction assignment.
//!
//! Work through the `TODO` markers in part order. Keep the public names and
//! function signatures unchanged so the test suite can exercise your code.

pub mod error;
pub mod state;
pub mod transaction;
pub mod utxo;

pub use error::TransactionError;
pub use state::{InvalidTransition, TxState};
pub use transaction::{
    find_outputs_for_recipient, highest_value_output, BitcoinValue, InputKind, OutPoint,
    OutputType, Transaction, TxOutput,
};
pub use utxo::{select_utxos, Utxo};
