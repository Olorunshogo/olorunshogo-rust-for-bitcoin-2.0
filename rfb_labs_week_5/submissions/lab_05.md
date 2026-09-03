# Lab 05 — Compatibility map

## Commands used

```bash
cargo test --test lab_05 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_05 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running tests/lab_05.rs (target/debug/deps/lab_05-eb8d8686bbdebb30)

running 4 tests
test builds_the_four_format_map ... ok
test selects_the_most_modern_supported_format ... ok
test older_p2sh_wallet_accepts_wrapped_but_not_native ... ok
test names_the_required_human_encoding ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_05/`:

- `submissions/screenshots/lab_05/05-cargo-test.png`

## Explanation

An older wallet accepts `3...` addresses and rejects `bc1q...` ones because the two live on separate encodings that a wallet has to implement support for independently. `3...` is a Base58Check P2SH address, the same encoding family the wallet already understands from `1...` P2PKH addresses, so a wallet that shipped before native SegWit existed can still pay it. `bc1q...` is Bech32, a completely different checksum and character set introduced with BIP173 alongside SegWit, so an old wallet's address parser simply does not recognize the prefix and cannot build a valid output for it, regardless of whether it understands the underlying script type.

`can_send_to` and `compatibility_report` model this as four independent capability flags (`base58_p2pkh`, `base58_p2sh`, `bech32`, `bech32m`) rather than one flag per script type, which is what separates sending support from spending support. `base58_p2sh` lets a wallet send to a P2SH-wrapped SegWit address without knowing anything about SegWit internally, it only needs to Base58Check-encode a script hash. Spending support is a different question entirely: actually consuming a SegWit output requires witness-aware transaction construction, which is a signing-side capability, not an address-encoding one. `best_supported_format` prefers Taproot, then native SegWit, then wrapped SegWit, then legacy P2PKH, matching the order in which sending capability was rolled out across the ecosystem.
