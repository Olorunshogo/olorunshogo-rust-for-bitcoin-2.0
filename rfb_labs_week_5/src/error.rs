use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error type shared by all Week 5 labs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabError {
    InvalidAddress(String),
    WrongNetwork(String),
    InvalidKey(String),
    InvalidScript(String),
    InvalidMnemonic(String),
    InvalidPath(String),
    Derivation(String),
    InvalidSize(String),
}

impl Display for LabError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAddress(message) => {
                write!(formatter, "invalid Bitcoin address: {message}")
            }
            Self::WrongNetwork(message) => write!(formatter, "address network mismatch: {message}"),
            Self::InvalidKey(message) => {
                write!(formatter, "invalid public or extended key: {message}")
            }
            Self::InvalidScript(message) => write!(formatter, "invalid Bitcoin script: {message}"),
            Self::InvalidMnemonic(message) => {
                write!(formatter, "invalid BIP39 mnemonic: {message}")
            }
            Self::InvalidPath(message) => write!(formatter, "invalid derivation path: {message}"),
            Self::Derivation(message) => write!(formatter, "key derivation failed: {message}"),
            Self::InvalidSize(message) => write!(formatter, "invalid transaction size: {message}"),
        }
    }
}

impl Error for LabError {}

pub type LabResult<T> = Result<T, LabError>;
