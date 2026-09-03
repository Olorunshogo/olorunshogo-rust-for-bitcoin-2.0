# Lab 03 — P2SH 2-of-3 multisig

## Commands used

```bash
cargo test --test lab_03 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_03 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running tests/lab_03.rs (target/debug/deps/lab_03-3df1e926531d5f77)

running 4 tests
test builds_a_two_of_three_redeem_script ... ok
test builds_the_outer_p2sh_lock ... ok
test derives_the_committed_p2sh_address ... ok
test reports_both_validation_layers ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_03/`:

- `submissions/screenshots/lab_03/03-cargo-test.png`

## Explanation

Matching the outer scriptPubKey is only the first gate, not proof of spending rights. `build_p2sh_script_pubkey` locks funds to `OP_HASH160 <scriptHash> OP_EQUAL`, where `scriptHash` is HASH160 of the redeemScript built in `build_2_of_3_redeem_script`. Anyone who supplies a script that hashes to the same value satisfies `OP_EQUAL`, but that only proves they know the redeemScript bytes, it does not authorize spending on its own.

The inner rule, `2 <pub1> <pub2> <pub3> 3 OP_CHECKMULTISIG`, is executed only after the hash check passes, and it demands two valid signatures from the listed public keys before the output can actually be spent. `inspect_p2sh_multisig` reports both layers together (`redeem_script_hex`, `address`, `script_pubkey_hex`) to make that separation explicit: the outer P2SH commitment hides the policy on chain until spend time, while the inner CHECKMULTISIG is what enforces the 2-of-3 rule. Presenting a matching script without the required signatures fails at `OP_CHECKMULTISIG`, which is why matching the script hash is necessary but not sufficient.
