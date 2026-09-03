//! Lab 07 — validate BIP39 recovery words and derive seeds safely.

use crate::model::{MnemonicReport, PassphraseComparison};
use crate::LabResult;

/// Validate an English mnemonic and report its entropy/checksum structure.
pub fn inspect_mnemonic(mnemonic: &str) -> LabResult<MnemonicReport> {
    todo!("Lab 07: validate BIP39 words and calculate ENT/CS lengths")
}

/// Derive the 512-bit BIP39 seed from words plus an optional passphrase.
pub fn mnemonic_seed_hex(mnemonic: &str, passphrase: &str) -> LabResult<String> {
    todo!("Lab 07: apply the BIP39 mnemonic-to-seed function")
}

/// Demonstrate that the same words with a different passphrase make a different seed.
pub fn compare_passphrases(
    mnemonic: &str,
    protected_passphrase: &str,
) -> LabResult<PassphraseComparison> {
    todo!("Lab 07: compare the empty-passphrase and protected seeds")
}

/// Recognize the public BIP39 test mnemonic used in the class labs.
pub fn is_public_test_mnemonic(mnemonic: &str) -> bool {
    todo!("Lab 07: accept only abandon x11 + about after normalizing whitespace")
}
