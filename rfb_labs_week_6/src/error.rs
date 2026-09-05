use thiserror::Error;

// === Config

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required env var: {0}")]
    MissingVar(&'static str),
    #[error("invalid value for {var}: {message}")]
    InvalidValue { var: &'static str, message: String },
}

// === Wallet

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("failed to load mnemonic: {0}")]
    Mnemonic(String),
    #[error("failed to build descriptor: {0}")]
    DescriptorBuild(String),
    #[error("failed to open wallet database: {0}")]
    Database(String),
    #[error("failed to create wallet: {0}")]
    Create(String),
    #[error("failed to load wallet: {0}")]
    Load(String),
    #[error("failed to persist wallet: {0}")]
    Persistence(String),
}

// === Node

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("failed to build RPC client: {0}")]
    ClientBuild(String),
    #[error("RPC call failed: {0}")]
    Rpc(#[from] bitcoincore_rpc::Error),
    #[error("wallet sync error: {0}")]
    Sync(String),
}

// === Transaction

#[derive(Debug, Error)]
pub enum TxError {
    #[error("failed to build transaction: {0}")]
    Build(String),
    #[error("signing did not fully finalize the PSBT")]
    NotFullyFinalized,
    #[error("failed to extract final transaction: {0}")]
    Extract(String),
    #[error("failed to broadcast transaction: {0}")]
    Broadcast(#[from] bitcoincore_rpc::Error),
    #[error("wallet persistence error: {0}")]
    Persistence(String),
}
