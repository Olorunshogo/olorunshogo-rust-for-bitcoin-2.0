# Lab 09 — BIP44 path decoding

## Commands used

```bash
cargo test --test lab_09 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_09 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running tests/lab_09.rs (target/debug/deps/lab_09-2e5d193eb57f896c)

running 4 tests
test changes_only_the_final_index ... ok
test decodes_every_bip44_level ... ok
test explains_zero_based_account_and_chain ... ok
test derives_the_selected_bip44_address ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
```

`decode_bip44_path("m/44'/0'/2'/1/5")` reports `purpose 44, coin_type 0, account 2, change 1, index 5`, and `describe_bip44_path` reads that back as the third account on the change chain, sixth address.

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_09/`:

- `submissions/screenshots/lab_09/09-cargo-test.png`

## Explanation

Account and address indexes in BIP44 are zero-based, so `m/44'/0'/2'/1/5` means the third account, not the second, and the sixth address on that chain, not the fifth. `decode_bip44_path` stores the raw numeric indexes (`account: 2`, `index: 5`), and `describe_bip44_path` is what translates the offset, index 2 is worded as "the third account" and index 5 as "the sixth address" because counting starts at 0. Getting this wrong is a common source of address-derivation bugs when restoring a wallet by hand.

The apostrophe marks a hardened derivation step, purpose, coin type, and account are always hardened (`44'`, `0'`, `2'`) so that a leaked account-level xpub can never expose the master key even in combination with a leaked child private key, the private-key-mixing hardened derivation described in Lab 08 stops that attack at each hardened boundary. Change and address index stay non-hardened (`1`, `5`) specifically so that xpubs at the account level can still generate watch-only receiving addresses. The `change` level is the receive/change branch: `0` is the external chain handed out to third parties as receiving addresses, and `1` is the internal chain a wallet uses privately for its own transaction change outputs, keeping the two address pools separate so external observers cannot trivially link a payment address to a change address from the same account.
