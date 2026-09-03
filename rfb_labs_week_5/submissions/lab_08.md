# Lab 08 — BIP32 extended keys

## Commands used

```bash
cargo test --test lab_08 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_08 -- --nocapture
warning: corrupt incremental compilation artifact found at `/home/olorunshogo/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5/target/debug/incremental/lab_08-1g577ox1xyex3/s-hlybmvtatk-07tycih-working/dep-graph.bin`. This file will automatically be ignored and deleted. If you see this message repeatedly or can provoke it without manually manipulating the compiler's artifacts, please file an issue. The incremental compilation system relies on hardlinks and filesystem locks behaving correctly, and may not deal well with OS crashes, so whatever information you can provide about your filesystem or other state may be very relevant

warning: `rfb-labs-week-5` (test "lab_08") generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running tests/lab_08.rs (target/debug/deps/lab_08-d5b9a76bd06010a2)

running 4 tests
test distinguishes_hardened_and_normal_paths ... ok
test derives_matching_extended_keys ... ok
test xpub_derives_a_normal_public_child ... ok
test creates_a_test_family_master_xpriv ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s
```

The `corrupt incremental compilation artifact` warning is a local Cargo cache message unrelated to the lab code. Cargo detects and discards the stale artifact and rebuilds automatically, it does not affect the test result.

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_08/`:

- `submissions/screenshots/lab_08/08-cargo-test.png`

## Explanation

The chain code is the 32-byte extra secret that HD derivation mixes in alongside a key so that child keys cannot be predicted from the parent public key and an index alone. `master_xpriv` and `derive_extended_keys` both come from `Xpriv::new_master`, which splits the BIP39 seed's HMAC-SHA512 output into a private key and a chain code. Every subsequent derivation step feeds the parent's chain code, not just its key, into HMAC-SHA512 to produce the child's key and its own chain code, which is what makes the whole tree deterministic from the same seed while keeping siblings cryptographically unlinkable without that shared chain code.

Xpubs exist so a system can generate and monitor receiving addresses without ever holding spending authority. `derive_normal_child_xpub` calls `derive_pub` on a parent xpub to walk down to child public keys, which is exactly the operation a watch-only wallet or a payment processor uses to hand out fresh addresses while the corresponding xpriv stays offline. This only works for normal (non-hardened) children, because a normal child key is computed from the parent public key, index, and chain code, none of which requires the parent private key. Hardened children flip that: BIP32 defines hardened derivation to hash the parent's private key itself into the child, specifically so that leaking a normal child's private key plus the parent xpub cannot expose the whole subtree, a known weakness of pure normal derivation. Because hardened derivation needs the private key as input, `path_contains_hardened_step` matters here too: an xpub-only wallet cannot derive past a hardened level (`m/44'`, `m/49'`, `m/84'`, account, purpose, coin type) at all, which is why those levels are always derived from the xpriv and only the branches below them are exposed as xpubs.
