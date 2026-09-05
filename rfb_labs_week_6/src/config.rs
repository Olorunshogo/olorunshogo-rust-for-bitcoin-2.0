use std::env;
use std::path::PathBuf;

use bitcoin::Network;

use crate::error::ConfigError;

// === Types

#[derive(Debug, Clone)]
pub enum RpcAuthConfig {
    CookieFile(PathBuf),
    UserPass(String, String),
    None,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub network: Network,
    pub rpc_url: String,
    pub rpc_auth: RpcAuthConfig,
    pub db_path: PathBuf,
    pub mnemonic: Option<String>,
}

// === Loading

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // Missing .env is fine — real env vars (CI, shell exports) still work.
        let _ = dotenvy::dotenv();

        let network = parse_network(&require_var("BITCOIN_NETWORK")?)?;
        let rpc_url = require_var("RPC_URL")?;
        let rpc_auth = load_rpc_auth()?;
        let db_path = env::var("WALLET_DB_PATH")
            .unwrap_or_else(|_| "wallet.sqlite".to_string())
            .into();
        let mnemonic = env::var("MNEMONIC").ok().filter(|value| !value.is_empty());

        Ok(Self {
            network,
            rpc_url,
            rpc_auth,
            db_path,
            mnemonic,
        })
    }
}

fn require_var(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingVar(name))
}

fn parse_network(value: &str) -> Result<Network, ConfigError> {
    match value.to_lowercase().as_str() {
        "regtest" => Ok(Network::Regtest),
        "testnet" => Ok(Network::Testnet),
        "signet" => Ok(Network::Signet),
        "bitcoin" | "mainnet" => Ok(Network::Bitcoin),
        other => Err(ConfigError::InvalidValue {
            var: "BITCOIN_NETWORK",
            message: format!("unknown network '{other}'"),
        }),
    }
}

fn load_rpc_auth() -> Result<RpcAuthConfig, ConfigError> {
    let cookie_file = env::var("RPC_COOKIE_FILE")
        .ok()
        .filter(|value| !value.is_empty());
    let user = env::var("RPC_USER").ok().filter(|value| !value.is_empty());
    let pass = env::var("RPC_PASS").ok().filter(|value| !value.is_empty());

    match (cookie_file, user, pass) {
        (Some(path), _, _) => Ok(RpcAuthConfig::CookieFile(PathBuf::from(path))),
        (None, Some(user), Some(pass)) => Ok(RpcAuthConfig::UserPass(user, pass)),
        (None, None, None) => Ok(RpcAuthConfig::None),
        (None, _, _) => Err(ConfigError::InvalidValue {
            var: "RPC_USER",
            message: "RPC_USER and RPC_PASS must both be set when not using RPC_COOKIE_FILE"
                .to_string(),
        }),
    }
}
