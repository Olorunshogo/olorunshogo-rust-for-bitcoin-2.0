# Lab 04 — Native P2WPKH

## Commands used

```bash
cargo test --test lab_04 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_04 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running tests/lab_04.rs (target/debug/deps/lab_04-d3a7b49e00fd133b)

running 4 tests
test leaves_scriptsig_empty_and_uses_witness ... ok
test derives_a_native_regtest_address ... ok
test builds_a_version_zero_witness_lock ... ok
test reports_a_twenty_byte_program ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_04/`:

- `submissions/screenshots/lab_04/04-cargo-test.png`

## Explanation

Native P2WPKH moves the unlocking data out of ScriptSig entirely and into the witness, which changes both the scriptPubKey shape and how the input is validated. `build_p2wpkh_script_pubkey` produces `0 <20-byte-pubKeyHash>`, a version-0 witness program with no opcodes at all, unlike P2PKH's `OP_DUP OP_HASH160 ... OP_EQUALVERIFY OP_CHECKSIG` or P2SH's `OP_HASH160 <scriptHash> OP_EQUAL`. There is nothing to execute in the locking script itself; the witness program is only a commitment that tells the node which rule set (version 0, 20 bytes means P2WPKH) to apply.

`native_spend_template` reflects the consensus rule directly: ScriptSig stays empty and the signature plus public key move into the witness stack instead. This is different from P2PKH, where both live in ScriptSig, and different from P2SH-wrapped SegWit, where a minimal ScriptSig still carries the redeemScript push while the real signature data sits in the witness. `witness_program` confirms the structural piece that makes this possible: a fixed 20-byte HASH160 commitment at witness version 0, which is what lets a node route the input to P2WPKH validation instead of legacy script execution, and what makes witness data eligible for the discounted weight used in Lab 06.
