use std::path::Path;

use bdk_wallet::rusqlite::Connection;
use bdk_wallet::{AddressInfo, Balance, KeychainKind, LocalOutput, PersistedWallet, Wallet};
use bitcoin::Network;

use crate::error::WalletError;
use crate::keys::Descriptors;

pub type WalletDb = Connection;

// === Open / create

/// Opens the wallet's SQLite persistence, loading an existing wallet if the file already has
/// data, or creating a fresh one from `descriptors` otherwise.
pub fn open_or_create_wallet(
    db_path: &Path,
    descriptors: &Descriptors,
    network: Network,
) -> Result<(PersistedWallet<WalletDb>, WalletDb), WalletError> {
    let mut db = Connection::open(db_path).map_err(|e| WalletError::Database(e.to_string()))?;

    let loaded = Wallet::load()
        .descriptor(KeychainKind::External, Some(descriptors.external.clone()))
        .descriptor(KeychainKind::Internal, Some(descriptors.internal.clone()))
        .extract_keys()
        .check_network(network)
        .load_wallet(&mut db)
        .map_err(|e| WalletError::Load(e.to_string()))?;

    let wallet = match loaded {
        Some(wallet) => wallet,
        None => Wallet::create(descriptors.external.clone(), descriptors.internal.clone())
            .network(network)
            .create_wallet(&mut db)
            .map_err(|e| WalletError::Create(e.to_string()))?,
    };

    Ok((wallet, db))
}

// === Addresses

pub fn new_receive_address(wallet: &mut PersistedWallet<WalletDb>) -> AddressInfo {
    wallet.reveal_next_address(KeychainKind::External)
}

pub fn new_change_address(wallet: &mut PersistedWallet<WalletDb>) -> AddressInfo {
    wallet.reveal_next_address(KeychainKind::Internal)
}

// === Balance / UTXOs

pub fn get_balance(wallet: &PersistedWallet<WalletDb>) -> Balance {
    wallet.balance()
}

pub fn list_utxos(wallet: &PersistedWallet<WalletDb>) -> Vec<LocalOutput> {
    wallet.list_unspent().collect()
}
