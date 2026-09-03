# Lab 10 — Deterministic recovery across BIP44, BIP49, and BIP84

## Commands used

```bash
cargo test --test lab_10 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_10 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running tests/lab_10.rs (target/debug/deps/lab_10-a39b1ec0ed495d43)

running 4 tests
test format_selection_changes_the_lock_target ... ok
test identical_recovery_inputs_repeat ... ok
test changing_only_the_index_changes_the_address ... ok
test derives_three_regtest_address_families ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

`derive_address_set` for account 0, index 0 on regtest produces a `m/n...` BIP44 P2PKH address, a `2...` BIP49 P2SH-wrapped P2WPKH address, and a `bcrt1q...` BIP84 native P2WPKH address, matching the expected regtest prefix for each script family.

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_10/`:

- `submissions/screenshots/lab_10/10-cargo-test.png`

## Explanation

Identical recovery inputs reproduce the same address because every step in the chain, BIP39 seed derivation, BIP32 master key creation, and child key derivation, is a deterministic pure function of its inputs. `recovery_is_repeatable` calls `derive_address_for_path` twice with the same mnemonic, passphrase, path, format, and network and gets the same address back both times, because `to_seed` always produces the same 512-bit seed for the same words and passphrase, `Xpriv::new_master` always produces the same master key and chain code from that seed, and `derive_priv` always walks the same path to the same child key. There is no randomness anywhere in this pipeline, which is the entire point of a hierarchical deterministic wallet, the words are the only thing that has to be backed up.

`changing_index_changes_address` shows the other half of that determinism: holding the mnemonic, passphrase, purpose, coin type, account, and change level fixed and only changing the final address index produces a different address, because that index feeds into a different HMAC-SHA512 derivation step at the last level of the tree. This is also why restoring a wallet takes more than the mnemonic and passphrase alone. `derive_address_set` fixes specific purposes (44, 49, 84) and coin types per network to reconstruct the exact BIP44, BIP49, and BIP84 branches, but a different derivation path or a different script family (P2PKH versus P2SH-wrapped versus native P2WPKH, shown directly in `format_selection_changes_the_lock_target`) at the same index produces a completely different address from the same key material. Recovering funds correctly means reproducing not just the mnemonic and passphrase, but the exact path and script convention the original wallet used to generate them.
