# Lab 06 — Weight, virtual size, and fees

## Commands used

```bash
cargo test --test lab_06 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_06 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running tests/lab_06.rs (target/debug/deps/lab_06-5143ef20effedfbe)

running 4 tests
test calculates_bip141_weight ... ok
test calculates_fee_from_feerate ... ok
test reproduces_the_class_fee_comparison ... ok
test rounds_weight_up_to_virtual_bytes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The class comparison is reproduced directly by `compare_fees(226, 141, 50)`, which returns a legacy fee of 11,300 sats against a SegWit fee of 7,050 sats at 50 sat/vB, a saving of 4,250 sats.

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_06/`:

- `submissions/screenshots/lab_06/06-cargo-test.png`

## Explanation

BIP141 weight is not witness data simply deleted, and it is not one flat discount applied to the whole transaction. Weight is computed as `stripped_size * 3 + total_size`, where `stripped_size` is the transaction serialized without any witness data and `total_size` includes it. Non-witness bytes (version, inputs, outputs, locktime) are counted three extra times through the `stripped_size * 3` term, landing at a full weight of 4 per byte, while witness bytes only ever appear once, in `total_size`, giving them an effective weight of 1 per byte. That is a per-byte-category discount baked into the formula, not a percentage knocked off the transaction total.

`virtual_size` then divides that weight by 4 and rounds up (`(weight + 3) / 4`) to get a byte-equivalent figure that fee estimation tools use directly, which is why `virtual_size(564)` gives 141 vB and `virtual_size(565)` gives 142 vB, one extra weight unit is enough to push a fee estimate up. The 226 vB legacy versus 141 vB SegWit figures from the class come out of this mechanism: a P2WPKH input still pays full weight for its non-witness bytes (outpoint, sequence, scriptSig length), but its actual unlocking data (signature and public key) sits in the witness at quarter weight, which is what shrinks the virtual size and, at a fixed feerate, the fee.
