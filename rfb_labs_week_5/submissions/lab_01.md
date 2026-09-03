# Lab 01 — Address and network identification

## Commands used

```bash
cargo test --test lab_01 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_01 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running tests/lab_01.rs (target/debug/deps/lab_01-4fd37e4269b310ea)

running 4 tests
test identifies_human_readable_prefixes ... ok
test maps_regtest_prefixes ... ok
test inspects_a_network_checked_address ... ok
test rejects_an_address_for_the_wrong_network ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo fmt --check
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$
```

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_01/`:

- `submissions/screenshots/lab_01/01-cargo-test.png`

## Explanation

The human-readable prefix is only a first-glance hint, not proof. `1`, `3`, `bc1q`, and `bc1p` tell you which encoding a wallet chose, but the string could still be malformed, meant for a different network, or simply mistyped.

Base58Check addresses (`1...`, `3...`) carry a version byte plus a 4-byte checksum derived from a double SHA-256 hash. The version byte encodes both the script type and the network, and the checksum is what actually proves the payload was not corrupted or altered. Bech32 and Bech32m addresses (`bc1...`) use a different checksum scheme (BCH-based) tied to a human-readable part such as `bc`, `tb`, or `bcrt`, which is what enforces the network at the encoding level.

`identify_prefix` in this lab only inspects the string, so it can be fooled by something like `bc1pexample`, which is not valid bech32m data at all. `inspect_address` is the function that does the real work: it parses the address into an `Address<NetworkUnchecked>`, then calls `require_network`, which validates both the checksum and the network before the address is trusted. That is why the wrong-network test expects an error even though the prefix logic alone would happily label the address. Prefix inspection is useful for quick UI hints, but checksum and network validation through the library are what must gate any address before it is used to receive or spend funds.
