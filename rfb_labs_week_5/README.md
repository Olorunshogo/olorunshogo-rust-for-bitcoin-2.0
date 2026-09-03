# Rust for Bitcoin 2.0 — Week 5 Practical Labs

**Author:** [Gideon Bature](https://github.com/GideonBature)

This assignment turns the Week 5 classes on Bitcoin address formats and HD wallets
into ten practical Rust labs. You will use `rust-bitcoin` and `bip39` to translate
addresses into scripts, compare compatibility and transaction weight, and reproduce
deterministic wallet branches from public test recovery data.

## Prerequisites

- Rust stable (`rustup`, `cargo`, and `rustfmt`)
- The Week 5 Bitcoin Address Formats class
- The Week 5 HD Wallets: BIP39, BIP32, and BIP44 class
- No Bitcoin Core, Polar, Docker, or live funds are required

## What you implement

Each file under `src/labs/` corresponds to one lab and contains four functions marked
with `todo!()`. Implement every function without changing its public signature.

| Lab | Topic |
|---:|---|
| 01 | Address prefixes, networks, and scriptPubKeys |
| 02 | Legacy P2PKH construction and ScriptSig |
| 03 | P2SH 2-of-3 multisig and redeemScripts |
| 04 | Native P2WPKH witness programs |
| 05 | Legacy, wrapped SegWit, native SegWit, and Taproot compatibility |
| 06 | Transaction weight, virtual size, and fees |
| 07 | BIP39 mnemonics, seeds, and passphrases |
| 08 | BIP32 extended keys and hardened derivation |
| 09 | BIP44 path decoding and address derivation |
| 10 | Deterministic recovery across BIP44, BIP49, and BIP84 |

## Running the tests

Run all public tests:

```bash
cargo test
```

Run one lab:

```bash
cargo test --test lab_01
```

Format your code:

```bash
cargo fmt --check
```

## Safe test data

The labs use this public BIP39 test mnemonic:

```text
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
```

It is published, known to everyone, and must never receive real funds. Never paste a
real mnemonic, passphrase, xprv, private key, or production wallet data into the code,
terminal output, screenshots, commits, or submission files.

## Submitting evidence

Complete `submissions/lab_01.md` through `submissions/lab_10.md`. Each file must
contain:

1. Commands used
2. Terminal output
3. Evidence references
4. Your explanation

## Scoring

Each lab is worth 10 points:

| Category | Points | Graded by |
|---|---:|---|
| Correct Rust execution | 4 | Public tests in GitHub Actions |
| Commands and evidence | 3 | Automated completeness checks |
| Accurate explanation | 3 | Instructor review |

GitHub Actions reports an **automated score out of 70**. The instructor then adds up
to 30 explanation points to produce the final score out of 100.

Run the same automated grader locally:

```bash
bash grader/grade.sh
```

## Primary references

- [BIP16 — Pay to Script Hash](https://github.com/bitcoin/bips/blob/master/bip-0016.mediawiki)
- [BIP32 — Hierarchical Deterministic Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP39 — Mnemonic Code](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP44 — Multi-Account Hierarchy](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)
- [BIP49 — P2WPKH nested in P2SH](https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki)
- [BIP84 — Native P2WPKH](https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki)
- [BIP141 — Segregated Witness](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki)
- [BIP173 — Bech32](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki)
- [BIP350 — Bech32m](https://github.com/bitcoin/bips/blob/master/bip-0350.mediawiki)

