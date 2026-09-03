//! Lab 05 — reason about sender support across address generations.

use crate::model::{AddressFormat, CompatibilityReport, SenderCapabilities};

/// Return whether the sender can decode and pay the selected address format.
pub fn can_send_to(capabilities: SenderCapabilities, format: AddressFormat) -> bool {
    todo!("Lab 05: map each format to the sender capability it requires")
}

/// Summarize support for legacy, wrapped SegWit, native SegWit, and Taproot.
pub fn compatibility_report(capabilities: SenderCapabilities) -> CompatibilityReport {
    todo!("Lab 05: build the complete compatibility map")
}

/// Choose the best supported single-key receiving format.
pub fn best_supported_format(capabilities: SenderCapabilities) -> Option<AddressFormat> {
    todo!("Lab 05: prefer Taproot, then P2WPKH, wrapped SegWit, then P2PKH")
}

/// Explain the encoding requirement that controls sender compatibility.
pub fn required_encoding(format: AddressFormat) -> &'static str {
    todo!("Lab 05: distinguish Base58Check, Bech32, and Bech32m")
}
