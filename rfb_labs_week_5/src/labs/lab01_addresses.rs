//! Lab 01 — identify Bitcoin address formats and enforce network safety.

use bitcoin::Network;

use crate::model::{AddressFormat, AddressReport};
use crate::LabResult;

/// Identify an address family from its human-readable prefix.
pub fn identify_prefix(address: &str) -> AddressFormat {
    todo!("Lab 01: identify P2PKH, P2SH, P2WPKH, and P2TR prefixes")
}

/// Return the expected human-readable prefix for a format on a selected network.
pub fn expected_prefix(format: AddressFormat, network: Network) -> Option<&'static str> {
    todo!("Lab 01: map address formats and networks to prefixes")
}

/// Parse an address, reject the wrong network, and return its full report.
pub fn inspect_address(address: &str, network: Network) -> LabResult<AddressReport> {
    todo!("Lab 01: validate the address and inspect its script type")
}

/// Return the scriptPubKey encoded by a network-checked address.
pub fn script_pubkey_hex(address: &str, network: Network) -> LabResult<String> {
    todo!("Lab 01: translate a checked address into scriptPubKey bytes")
}
