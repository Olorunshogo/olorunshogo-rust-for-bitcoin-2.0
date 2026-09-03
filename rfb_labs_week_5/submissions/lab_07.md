# Lab 07 — BIP39 mnemonics, seeds, and passphrases

## Commands used

```bash
cargo test --test lab_07 -- --nocapture
cargo fmt --check
```

## Terminal output

```bash
olorunshogo@olorunshogo:~/Projects/Rust/Rust/olorunshogo-rust-for-bitcoin-2.0/rfb_labs_week_5$ cargo test --test lab_07 -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running tests/lab_07.rs (target/debug/deps/lab_07-4f13bf30e901edac)

running 4 tests
test rejects_an_invalid_checksum ... ok
test validates_entropy_and_checksum_structure ... ok
test matches_the_published_bip39_seed_vector ... ok
test passphrase_selects_a_different_wallet ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

`mnemonic_seed_hex` against the public test mnemonic with passphrase `TREZOR` reproduces the published BIP39 test vector seed, `c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04`.

## Evidence references

Screenshots are stored under `submissions/screenshots/lab_07/`:

- `submissions/screenshots/lab_07/07-cargo-test.png`

## Explanation

The BIP39 checksum is error detection, not encryption. `inspect_mnemonic` derives `checksum_bits` as `entropy_bits / 32`, four bits for a 12-word, 128-bit-entropy mnemonic. Those bits are the first few bits of `SHA256(entropy)` appended to the entropy before it is split into 11-bit word indices. Flipping a word, or mistyping one, almost always breaks that relationship, which is exactly what `rejects_an_invalid_checksum` demonstrates: twelve `abandon` words fail because the final word no longer encodes the checksum of the preceding entropy. The checksum only lets a wallet detect a corrupted or mistyped mnemonic before deriving keys from it, it does nothing to hide or protect the entropy itself, anyone with the words can read the entropy directly.

The optional BIP39 passphrase is a completely separate secret that is never stored in or derivable from the mnemonic. `compare_passphrases` shows this directly: the same twelve words with an empty passphrase and with `class` produce two different 512-bit seeds through PBKDF2-HMAC-SHA512, because the passphrase is mixed into the salt of that key stretching function, not appended to the entropy. If a user forgets the passphrase, there is no computation that recovers it from the mnemonic, since the mnemonic's checksum only covers the entropy, not the passphrase, and the seed derivation is a one-way function. The words alone open the empty-passphrase wallet, never the passphrase-protected one, which is why the passphrase is what people call the "25th word" and losing it means losing that specific wallet permanently.
