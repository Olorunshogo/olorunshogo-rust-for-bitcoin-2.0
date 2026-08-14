# Rust for Bitcoin 2.0 — Week 1 Practical Labs

This assignment turns the Week 1 Bitcoin Fundamentals class into ten practical labs.
You will write Rust functions that drive Bitcoin Core through `bitcoin-cli`, then run
those functions against Bitcoin Core nodes managed by Polar.

## Prerequisites

- [Rust](https://rust-lang.org/tools/install/) stable (`rustup`, `cargo`, and `rustfmt`)
- [Docker](https://www.docker.com/)
- [Polar v4.0.0 or a compatible release](https://lightningpolar.com/)
- A Polar regtest network containing Bitcoin Core
- `bitcoin-cli` available inside the Bitcoin Core terminal

## What you implement

Each file under `src/labs/` corresponds to one lab and contains four functions marked
with `todo!()`. Implement every function without changing its public signature.

| Lab | Topic |
|---:|---|
| 01 | Regtest network inspection |
| 02 | Wallets and addresses |
| 03 | Coinbase maturity |
| 04 | UTXOs and outpoints |
| 05 | Broadcast and mempool state |
| 06 | Transaction decoding and value conservation |
| 07 | Confirmation and block membership |
| 08 | Block headers, proof of work, and confirmations |
| 09 | Multi-UTXO coin selection |
| 10 | Competing branches and the most-work rule |

The provided [`ProcessRpc`](src/rpc.rs) implementation runs `bitcoin-cli`. The lab
functions accept the [`RpcClient`](src/rpc.rs) trait so the same code can run against
Polar and deterministic mock clients in the grader.

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

## Running against Polar

Use `ProcessRpc::default()` when calling your completed functions:

```rust
use rfb_labs_week_1::rpc::ProcessRpc;

let rpc = ProcessRpc::default();
```

Run the program from a terminal where `bitcoin-cli` can reach the intended regtest
node. When working with multiple Polar nodes, construct separate clients with the
appropriate CLI arguments:

```rust
use rfb_labs_week_1::rpc::ProcessRpc;

let node_a = ProcessRpc::new("bitcoin-cli").with_base_args(["-regtest"]);
```

## Submitting evidence

Complete `submissions/lab_01.md` through `submissions/lab_10.md`. Each file must
contain:

1. Commands used
2. Terminal output
3. Screenshots or other evidence references
4. Your explanation

Do not submit private keys, wallet seed material, access tokens, or real-mainnet
wallet data.

## Scoring

Each lab is worth 10 points:

| Category | Points | Graded by |
|---|---:|---|
| Correct Rust execution | 4 | Public tests in GitHub Actions |
| Commands and evidence | 3 | Automated completeness checks |
| Accurate explanation | 3 | Instructor review |

GitHub Actions reports an **automated score out of 70**. The instructor then adds up
to 30 explanation points to produce the final score out of 100. Automated evidence
checks only confirm that required sections were completed; the instructor may still
reject fabricated, irrelevant, or unsafe evidence.

Run the same automated grader locally:

```bash
bash grader/grade.sh
```
