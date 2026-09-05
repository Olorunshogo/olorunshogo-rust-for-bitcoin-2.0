# Week 6 - Regtest Bitcoin Wallet

A minimal Bitcoin wallet built in Rust, running against `regtest`. It generates
keys from a BIP39 mnemonic, derives BIP84 (`wpkh`) receive/change descriptors,
tracks UTXOs and balance, persists its state to SQLite, and can build, sign,
and broadcast transactions through a connection to a local `bitcoind` node.

See [`Readme.md`](./Readme.md) for the original assignment brief. This file
covers setup, design rationale, and proof that it works.

## Setup

### 1. Run a regtest node

You need a local `bitcoind` reachable over RPC. Any of the following works:

```bash
bitcoind -regtest -datadir=/path/to/a/regtest/datadir \
  -server=1 -rpcbind=127.0.0.1 -rpcallowip=127.0.0.1 -fallbackfee=0.0002
```

Or use [Polar](https://lightningpolar.com/): spin up a network with a Bitcoin
Core backend node and use the RPC host/port/credentials it shows you for that
node.

### 2. Configure `.env`

```bash
cp .env.example .env
```

Then edit `.env`:

| Variable | Meaning |
|---|---|
| `BITCOIN_NETWORK` | `regtest` or `testnet` |
| `RPC_URL` | `host:port` of `bitcoind`'s RPC server, e.g. `127.0.0.1:18443` |
| `RPC_COOKIE_FILE` | path to `bitcoind`'s `.cookie` file (leave `RPC_USER`/`RPC_PASS` empty when using this) |
| `RPC_USER` / `RPC_PASS` | alternative to cookie auth |
| `WALLET_DB_PATH` | where the wallet's SQLite file lives (default `wallet.sqlite`) |
| `MNEMONIC` | leave empty on first run; the wallet generates one and prints it once, copy it back into `.env` to keep using the same wallet |

`.env` is gitignored. Never commit it.

### 3. Run it

```bash
cargo build
cargo run -- --help
```

Subcommands:

```bash
cargo run -- demo                        # full create -> fund -> sync -> send -> confirm walkthrough (regtest only)
cargo run -- balance                     # sync, then print balance + UTXO count
cargo run -- address                     # reveal & print the next receive address
cargo run -- address --internal          # reveal & print the next change address
cargo run -- sync                        # sync the wallet's view of the chain
cargo run -- send --to <address> --amount <btc> [--fee-rate <sat/vb>]
cargo run -- fund --blocks <n>           # regtest only: mine blocks to a fresh address
```

`demo` is the same walkthrough used to produce the proof of a working
transaction below; run it against a fresh `.env`/`wallet.sqlite` to
reproduce it.

## Design & library choices

| Library | Used for | Why |
|---|---|---|
| **BDK** (`bdk_wallet`) | Descriptor parsing, external/internal keychain management, address derivation, UTXO/balance tracking, PSBT construction and signing, SQLite persistence (`rusqlite` feature) | Reimplementing BIP32 derivation, a UTXO index, coin selection, and a persistence schema from scratch is multi-week work BDK already does correctly. Using it puts the effort into proving the wallet actually works (a real broadcast, real persistence) instead of re-deriving infrastructure. |
| **`bdk_bitcoind_rpc`** | `Emitter`-driven block-by-block + mempool sync against `bitcoind` | This crate *is* the wallet's use of `bitcoincore-rpc` for sync: it wraps a `bitcoincore_rpc::Client` internally rather than being an alternative to it. Covers "connect to a node via bitcoincore-rpc" for everything that isn't regtest chain administration. |
| **`bitcoincore-rpc`** (direct) | Regtest-only chain control: `generatetoaddress` (funding), `getblockchaininfo` (connectivity check), `sendrawtransaction` (broadcast) | These aren't wallet operations; mining blocks and broadcasting a raw transaction are node/chain administration, so they go through a plain `bitcoincore_rpc::Client` rather than through BDK. |
| **`rust-bitcoin`** (direct) | Post-signing transaction introspection in `tx::send`: `Psbt::extract_tx()`, then `Transaction::compute_txid()`, `.weight()`, `.vsize()`, and `consensus::encode::serialize_hex()` | See [rust-bitcoin vs BDK](#rust-bitcoin-vs-bdk) below. |
| `dotenvy` | Loading `.env` | Keeps secrets (mnemonic, RPC credentials) out of source and out of `Cargo.toml`. |
| `clap` | The CLI (`src/cli.rs`) | Stretch goal: operate the wallet (`balance`/`address`/`send`/`sync`/`fund`) without editing code. |
| `anyhow` / `thiserror` | Error handling | `thiserror` gives typed errors per module (`ConfigError`, `WalletError`, `NodeError`, `TxError`); `anyhow` propagates them cleanly out of `main` and adds context at the CLI boundary (e.g. address/amount parsing in `send`). |

### Descriptor structure

The wallet derives two **BIP84** (native segwit, `wpkh`) descriptors from a
single BIP39 mnemonic, at `m/84h/1h/0h/0` (external/receive) and
`m/84h/1h/0h/1` (internal/change), the standard account-0 split for a
single-sig segwit wallet. `wpkh` was chosen over `tr` (Taproot) as the
primary format because it's the simpler, more universally-supported case for
the minimum requirements.

### rust-bitcoin vs BDK

BDK's `TxBuilder`/PSBT flow handles everything the minimum requirements need:
building, signing, and finalizing a transaction. It doesn't, however,
surface low-level introspection like exact transaction weight, vsize, or a
raw hex dump. In `src/tx.rs`, once BDK has signed and finalized the PSBT:

```rust
let tx = psbt.extract_tx().map_err(|e| TxError::Extract(e.to_string()))?;
println!("tx txid: {}", tx.compute_txid());
println!("tx weight: {}", tx.weight());
println!("tx vsize: {}", tx.vsize());
println!("tx raw hex: {}", serialize_hex(&tx));
```

`psbt.extract_tx()` returns a plain `bitcoin::Transaction`; from that point
on this is raw `rust-bitcoin`, not BDK. This is the kind of thing you'd reach
for `rust-bitcoin` directly for: BDK's `TxBuilder` is the right tool for
*building* a transaction, but once you have a finalized one and want to
inspect its wire format directly (weight accounting, manual broadcast,
custom parsing), going straight to `rust-bitcoin`'s own types is simpler than
looking for an equivalent BDK convenience method.

### Constraints followed

- Regtest only (the code also accepts `testnet` in config, untested here).
- No hardcoded keys: the mnemonic comes from `.env` (gitignored) or is
  generated fresh at runtime and printed once with an explicit warning to
  save it and never commit it. `.env.example` only ever contains placeholders.
  Descriptor strings are only ever printed in their **public** (xpub) form:
  `keys::descriptors_from_mnemonic` returns both a private (tprv, used to
  build a signing wallet) and public (xpub, safe to log) variant of each
  descriptor, specifically so a private key never ends up in stdout.

## Known limitations

- Single-signature only, no multisig, no descriptor types beyond `wpkh`.
- No fee bumping / RBF.
- `sync_wallet` runs synchronously to completion; there's no background sync
  or graceful interrupt.
- Coin selection uses BDK's default algorithm, no explicit strategy.
- Only tested against regtest; testnet is wired into `Config` but unverified.
- `bitcoin-cli gettransaction` doesn't work against this setup and isn't used
  for verification below: it's a Bitcoin Core *wallet* RPC, and this project
  deliberately never loads a wallet in Core (all wallet state lives in our
  own SQLite file via BDK). `getrawtransaction <txid> true` is the correct,
  wallet-independent way to inspect a transaction on the node, and is what's
  used below.
- Getting the `bdk_wallet::descriptor!` macro to compile required adding
  `miniscript` as a direct dependency of this crate (matching the version
  BDK itself pins): one arm of the macro references the bare `miniscript::`
  path rather than `$crate::miniscript::`, so the caller's crate needs its
  own edge to that crate in the dependency graph for the unqualified path to
  resolve.

## Proof of a working transaction

Produced by running `cargo run -- demo` against a Polar-managed regtest
`bitcoind` node and a fresh `wallet.sqlite`/`.env`.

Wallet output:

```
generated a new mnemonic, save it to MNEMONIC in .env, never commit it:
owner inquiry rotate civil census repeat short rapid state dial pass gauge
external descriptor (public): wpkh([15f88e68/84'/1'/0']tpubDCxu57Fdt1Dq4s73ZQcWBQXLfV7Fo6FNxppMLK6Y5ynpEKhof4quLbMakDgQsxdm3ja7ojE9iQicNKXCUMKyvbYVpe1GurowCErzCzPMKR2/0/*)#6ul2e3gk
internal descriptor (public): wpkh([15f88e68/84'/1'/0']tpubDCxu57Fdt1Dq4s73ZQcWBQXLfV7Fo6FNxppMLK6Y5ynpEKhof4quLbMakDgQsxdm3ja7ojE9iQicNKXCUMKyvbYVpe1GurowCErzCzPMKR2/1/*)#tg6tyycw
receive address (external): bcrt1qc33eg8ggl02qh0mpadmphh9qj92903nv0rkvt7
change address (internal):  bcrt1qxhtlgcwfvl8287h7h26ld0tdh097c7z2rc0ned
mined 101 blocks to bcrt1qc33eg8ggl02qh0mpadmphh9qj92903nv0rkvt7
balance: total=5050 BTC confirmed=100 BTC trusted_pending=0 BTC
utxos: 101
tx txid: 5807dfa620b0271e0ead709b9a5de20730e028ab20f67460c74ffedd839c925a
tx weight: 561
tx vsize: 141
tx raw hex: 02000000000101ea8232001c7fe42931f28d894637311b0d3d4e4bd8bfa62f0d73931363a0e1080000000000fdffffff0200e1f50500000000160014ece28254a9959067872dfcc77c2d68b1491ab1f0e70f10240100000016001435d7f461c967cea3fafebab5f6bd6dbbcbec784a0247304402207143081c22eefe33167b34052eea3dfa6f11ba3408b9898f2c093f0d9158152902203e2558c5552fae3b0f92e9c93552475f549e1700266fe05c329a144fa81e4c8301210339246b7a699ba481b0832a3fd19d91c83d3c7dcccf48e3390414c554572d83d567000000
broadcast txid: 5807dfa620b0271e0ead709b9a5de20730e028ab20f67460c74ffedd839c925a
balance after 1 confirmation: total=5100 BTC confirmed=149.99999719 BTC
```

Independently verified on the node (no wallet loaded in Core, see
[Known limitations](#known-limitations)):

```bash
$ curl -s --user polaruser:polarpass \
    --data-binary '{"jsonrpc":"1.0","id":"t","method":"getrawtransaction","params":["5807dfa620b0271e0ead709b9a5de20730e028ab20f67460c74ffedd839c925a", true]}' \
    -H 'content-type:text/plain;' http://127.0.0.1:18443/
{
  "result": {
    "txid": "5807dfa620b0271e0ead709b9a5de20730e028ab20f67460c74ffedd839c925a",
    "vsize": 141,
    "weight": 561,
    "vout": [
      { "value": 1.00000000, "address": "bcrt1qan3gy49fjkgx0pedlnrhcttgk9y34v0ss949gz", ... },
      { "value": 48.99999719, "address": "bcrt1qxhtlgcwfvl8287h7h26ld0tdh097c7z2rc0ned", ... }
    ],
    "confirmations": 1,
    ...
  }
}
```

`confirmations: 1` confirms the broadcast transaction is real, mined, and
matches exactly what the wallet reported (1 BTC out, ~49 BTC change, same
txid, same weight/vsize): the wallet's own view of the world agrees with
the node's.

### Error-handling verification

Two failure paths were checked to exit cleanly instead of panicking:

- Missing required config (`.env` absent, no `BITCOIN_NETWORK` set): reports
  `Error: missing required env var: BITCOIN_NETWORK`, exit code 1.
- Syntactically valid but unreachable RPC endpoint
  (`RPC_URL=127.0.0.1:1`): reports the underlying connection error via
  `NodeError::Rpc`, exit code 1, no backtrace.
