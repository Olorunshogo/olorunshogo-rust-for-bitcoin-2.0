# Assignment: Building a Bitcoin Wallet in Rust

## Goal

Build a functioning Bitcoin wallet in Rust (regtest) that demonstrates you can use the libraries covered in class effectively.

## Minimum Requirements

Your wallet must be able to:

1. **Generate or import keys** and derive a wallet from a descriptor.
2. **Generate addresses** from both an external (receiving) and internal (change) keychain.
3. **Track UTXOs and calculate balance** for the wallet.
4. **Persist wallet state locally** (e.g. with SQLite) so the wallet can be closed and reopened without losing track of its own state.
5. **Construct, sign, and broadcast a transaction** on testnet.
6. **Connect to a Bitcoin node** (via `bitcoincore-rpc`) to sync wallet state or broadcast transactions, i.e., your wallet should not be purely offline.

## Stretch Goals

Pick any of these if you want to push further:

- Support multiple descriptor types (e.g. compare `wpkh` vs `tr` Taproot)
- Build a simple CLI so a user can check balance, get a new address, and send funds without editing code
- Handle coin selection explicitly rather than relying on defaults
- Add basic error handling/logging that would make this usable by someone other than you
- Explain (in your README) a scenario where you reached for raw `rust-bitcoin` instead of BDK, and demonstrate it with a small code example

## Acceptance Criteria

1. **A PR TO THE RFB GITHUB** (source code, `Cargo.toml`, etc.) as a zip or a link to a repository.
2. **A README** that includes:
   - How to run your wallet (setup instructions, any node/config required)
   - A short explanation (project or descriptor structure, and why you chose it)
   - Which libraries you used where, and why (e.g. "I used `bitcoincore-rpc` for X, and BDK for Y, because...")
   - Any known limitations or things you'd improve with more time

## Constraints

- Testnet or regtest only.
- Do not hardcode private keys or seed phrases in files you submit, use a `.env`, config file, or generate fresh test keys. (This is also good practice for real-world Bitcoin development.)
- You may use any crates that support your chosen libraries (e.g. `dotenv`, `clap`, `tokio`), but the wallet logic itself should go through `rust-bitcoin`, `bitcoincore-rpc`, and/or BDK.

## Reference Material

- rust-bitcoin docs: https://docs.rs/bitcoin/0.32.102/bitcoin/index.html
- Bitcoin Core RPC reference: https://developer.bitcoin.org/reference/rpc/
- BDK Wallet docs: https://docs.rs/bdk_wallet/latest/bdk_wallet/index.html

---

## Grading Rubric (100 points)

| Category | Criteria |
|---|---|
| **Correctness — Core Functionality** | Wallet generates keys/addresses correctly from a descriptor; external and internal keychains are properly separated |
| **Correctness — UTXO & Balance Tracking** | Wallet accurately tracks UTXOs and reports correct balance after syncing |
| **Correctness — Transactions** | Wallet successfully creates, signs, and broadcasts a transaction on testnet/regtest; txid is verifiable |
| **Persistence** | Wallet state survives a restart (SQLite or equivalent) without needing to re-derive/re-sync from scratch |
| **Node Integration** | Wallet correctly connects to and communicates with a Bitcoin node (RPC or BDK-supported connection) |
| **Architecture & Library Use** | Sensible use of rust-bitcoin / bitcoincore-rpc / BDK together, student can justify *why* each was used where|
| **Code Quality** | Reasonably organized, readable, compiles cleanly, handles at least basic errors (doesn't just panic on bad input) |
| **README & Documentation** | Clear setup instructions, explains design decisions, includes proof of a working transaction |
| **Stretch Goals (bonus)** | Any stretch goal attempted and working |

## Timeline

- **Due:** 5th Septermber 2026