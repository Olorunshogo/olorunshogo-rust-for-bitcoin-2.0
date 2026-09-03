//! Lab 01 — identify Bitcoin address formats and enforce network safety.

use bitcoin::address::{AddressType, NetworkUnchecked};
use bitcoin::{Address, Network};

use crate::error::LabError;
use crate::model::{AddressFormat, AddressReport};
use crate::LabResult;

/// Identify an address family from its human-readable prefix.
pub fn identify_prefix(address: &str) -> AddressFormat {
    // todo!("Lab 01: identify P2PKH, P2SH, P2WPKH, and P2TR prefixes")
    if let Some(rest) = address
        .strip_prefix("bc1")
        .or_else(|| address.strip_prefix("tb1"))
        .or_else(|| address.strip_prefix("bcrt1"))
    {
        match rest.chars().next() {
            Some('q') => AddressFormat::P2wpkh,
            Some('p') => AddressFormat::P2tr,
            _ => AddressFormat::Unknown,
        }
    } else if address.starts_with('1') {
        AddressFormat::P2pkh
    } else if address.starts_with('3') || address.starts_with('2') {
        AddressFormat::P2sh
    } else {
        AddressFormat::Unknown
    }
}

/// Return the expected human-readable prefix for a format on a selected network.
pub fn expected_prefix(format: AddressFormat, network: Network) -> Option<&'static str> {
    // todo!("Lab 01: map address formats and networks to prefixes")
    match (format, network) {
        (AddressFormat::P2pkh, Network::Bitcoin) => Some("1"),
        (AddressFormat::P2pkh, Network::Testnet | Network::Testnet4 | Network::Signet) => {
            Some("m/n")
        }
        (AddressFormat::P2pkh, Network::Regtest) => Some("m/n"),
        (AddressFormat::P2sh, Network::Bitcoin) => Some("3"),
        (AddressFormat::P2sh, Network::Testnet | Network::Testnet4 | Network::Signet) => Some("2"),
        (AddressFormat::P2sh, Network::Regtest) => Some("2"),
        (AddressFormat::P2wpkh, Network::Bitcoin) => Some("bc1q"),
        (AddressFormat::P2wpkh, Network::Testnet | Network::Testnet4 | Network::Signet) => {
            Some("tb1q")
        }
        (AddressFormat::P2wpkh, Network::Regtest) => Some("bcrt1q"),
        (AddressFormat::P2tr, Network::Bitcoin) => Some("bc1p"),
        (AddressFormat::P2tr, Network::Testnet | Network::Testnet4 | Network::Signet) => {
            Some("tb1p")
        }
        (AddressFormat::P2tr, Network::Regtest) => Some("bcrt1p"),
        (AddressFormat::Unknown, _) => None,
    }
}

/// Parse an address, reject the wrong network, and return its full report.
pub fn inspect_address(address: &str, network: Network) -> LabResult<AddressReport> {
    // todo!("Lab 01: validate the address and inspect its script type")
    let unchecked = address
        .parse::<Address<NetworkUnchecked>>()
        .map_err(|error| LabError::InvalidAddress(error.to_string()))?;
    let checked = unchecked
        .require_network(network)
        .map_err(|error| LabError::WrongNetwork(error.to_string()))?;

    let format = match checked.address_type() {
        Some(AddressType::P2pkh) => AddressFormat::P2pkh,
        Some(AddressType::P2sh) => AddressFormat::P2sh,
        Some(AddressType::P2wpkh) => AddressFormat::P2wpkh,
        Some(AddressType::P2tr) => AddressFormat::P2tr,
        _ => AddressFormat::Unknown,
    };

    Ok(AddressReport {
        address: checked.to_string(),
        network: network.to_string(),
        format,
        script_pubkey_hex: checked.script_pubkey().to_hex_string(),
    })
}

/// Return the scriptPubKey encoded by a network-checked address.
pub fn script_pubkey_hex(address: &str, network: Network) -> LabResult<String> {
    // todo!("Lab 01: translate a checked address into scriptPubKey bytes")
    Ok(inspect_address(address, network)?.script_pubkey_hex)
}
