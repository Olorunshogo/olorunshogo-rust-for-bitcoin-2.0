//! Lab 05 — reason about sender support across address generations.

use crate::model::{AddressFormat, CompatibilityReport, SenderCapabilities};

/// Return whether the sender can decode and pay the selected address format.
pub fn can_send_to(capabilities: SenderCapabilities, format: AddressFormat) -> bool {
    // todo!("Lab 05: map each format to the sender capability it requires")
    match format {
        AddressFormat::P2pkh => capabilities.base58_p2pkh,
        AddressFormat::P2sh => capabilities.base58_p2sh,
        AddressFormat::P2wpkh => capabilities.bech32,
        AddressFormat::P2tr => capabilities.bech32m,
        AddressFormat::Unknown => false,
    }
}

/// Summarize support for legacy, wrapped SegWit, native SegWit, and Taproot.
pub fn compatibility_report(capabilities: SenderCapabilities) -> CompatibilityReport {
    // todo!("Lab 05: build the complete compatibility map")
    CompatibilityReport {
        p2pkh: capabilities.base58_p2pkh,
        p2sh_p2wpkh: capabilities.base58_p2sh,
        p2wpkh: capabilities.bech32,
        p2tr: capabilities.bech32m,
    }
}

/// Choose the best supported single-key receiving format.
pub fn best_supported_format(capabilities: SenderCapabilities) -> Option<AddressFormat> {
    // todo!("Lab 05: prefer Taproot, then P2WPKH, wrapped SegWit, then P2PKH")
    if capabilities.bech32m {
        Some(AddressFormat::P2tr)
    } else if capabilities.bech32 {
        Some(AddressFormat::P2wpkh)
    } else if capabilities.base58_p2sh {
        Some(AddressFormat::P2sh)
    } else if capabilities.base58_p2pkh {
        Some(AddressFormat::P2pkh)
    } else {
        None
    }
}

/// Explain the encoding requirement that controls sender compatibility.
pub fn required_encoding(format: AddressFormat) -> &'static str {
    // todo!("Lab 05: distinguish Base58Check, Bech32, and Bech32m")
    match format {
        AddressFormat::P2pkh | AddressFormat::P2sh => "Base58Check",
        AddressFormat::P2wpkh => "Bech32",
        AddressFormat::P2tr => "Bech32m",
        AddressFormat::Unknown => "Unknown",
    }
}
