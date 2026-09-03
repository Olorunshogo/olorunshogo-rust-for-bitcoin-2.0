# Lab 02 — Legacy P2PKH construction

## Commands used

```bash
cargo test --test lab_02 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_02 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running tests/lab_02.rs (target/debug/deps/lab_02-1d64efbce3d4f4b7)

running 4 tests
test builds_the_standard_p2pkh_lock ... ok
test puts_unlocking_data_in_scriptsig ... ok
test commits_to_hash160_of_the_public_key ... ok
test derives_the_expected_p2pkh_address ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo fmt --check
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$
```

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_02/`:

- `submissions/screenshots/lab_02/02-cargo-test.png`

## Explanation

A P2PKH lock proves control of a key, not authorization to spend by itself. `derive_p2pkh_address` and `build_p2pkh_script_pubkey` both start from the same commitment, `committed_pubkey_hash`, which is HASH160 (RIPEMD160 of SHA256) of the compressed public key. That hash is what the scriptPubKey `OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG` actually commits to on chain, so anyone can see who is allowed to spend the output, but seeing that identity is not the same as being able to spend it.

Spend authorization only happens in the ScriptSig, which `p2pkh_spend_template` models as `[signature, public_key]`. `OP_DUP OP_HASH160 ... OP_EQUALVERIFY` only checks that the supplied public key hashes to the committed value, that is key identity. The actual authorization comes from `OP_CHECKSIG`, which verifies the ECDSA signature against that public key and the transaction data being spent. Presenting the correct public key alone satisfies identity but fails at `OP_CHECKSIG` without a valid signature from the matching private key, which is why P2PKH is described as key identity plus a separate signature check, not one combined step.
