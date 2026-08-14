# Lab 01 — Regtest network inspection

## Commands used

TODO: List the Rust command and Bitcoin Core RPCs you ran.

Polar network **Week 1 Bitcoin Fundamentals**, single Bitcoin Core node `backend1`.
All RPCs were issued from that node's terminal, which is the same `bitcoin-cli`
invocation my Rust code performs through `ProcessRpc`.

```bash
# inside the backend1 container terminal
bitcoin-cli getblockchaininfo
bitcoin-cli getblockcount
bitcoin-cli getbestblockhash
```

Rust entry points exercised, from `src/labs/lab01_network.rs`:

| Function | RPC it drives |
|---|---|
| `get_chain` | `getblockchaininfo`, reads the `chain` field |
| `get_block_height` | `getblockcount` |
| `get_best_block_hash` | `getbestblockhash` |
| `inspect_network` | composes all three and rejects any chain that is not `regtest` |

```bash
cargo test --test lab_01
```

## Terminal output

TODO: Record chain, block height, and best-block hash.

`getblockchaininfo` on `backend1`:

```json
{
  "chain": "regtest",
  "blocks": 1,
  "headers": 1,
  "bestblockhash": "7e14235b899e85cc1111601f2355e43875dda90ae2bf75daeda1a8c9f8b91f07",
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "time": 1785608534,
  "mediantime": 1785608534,
  "verificationprogress": 1,
  "initialblockdownload": false,
  "chainwork": "0000000000000000000000000000000000000000000000000000000000000004",
  "size_on_disk": 590,
  "pruned": false,
  "warnings": []
}
```

`getblockcount`:

```text
1
```

`getbestblockhash`:

```text
7e14235b899e85cc1111601f2355e43875dda90ae2bf75daeda1a8c9f8b91f07
```

The node answered every call, which is itself the proof that it is running. The
`NetworkSnapshot` my `inspect_network` builds from these three values is:

| Field | Value |
|---|---|
| `chain` | `regtest` |
| `block_height` | `1` |
| `best_block_hash` | `7e14235b899e85cc1111601f2355e43875dda90ae2bf75daeda1a8c9f8b91f07` |

Public test suite:

```text
running 4 tests
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_01/`.

| Screenshot | Shows |
|---|---|
| [Lab_01_01_getchain.png](Evidence/Lab_01/Lab_01_01_getchain.png) | Full `getblockchaininfo` response on `backend1`, with `"chain": "regtest"` |
| [Lab_01_02_getblockheight.png](Evidence/Lab_01/Lab_01_02_getblockheight.png) | `getblockcount` returning `1` |
| [Lab_01_03_getbestblockhash.png](Evidence/Lab_01/Lab_01_03_getbestblockhash.png) | `getbestblockhash` returning `7e14235b...b91f07` |

The best-block hash in screenshot 3 matches the `bestblockhash` field in screenshot 1,
so the two independent RPCs agree on the same tip.

## Explanation

TODO: Explain Polar, Docker, Bitcoin Core, and regtest in your own words.

**Bitcoin Core** is the node software itself. It is the thing that actually holds the
blockchain, validates every block and transaction against consensus rules, keeps a
mempool of unconfirmed transactions, manages wallets, and exposes the JSON-RPC
interface that `bitcoin-cli` talks to. Everything else in this stack exists only to
make it convenient to run.

**regtest** is one of the networks Bitcoin Core can run. Unlike mainnet and testnet it
is a private chain with a trivial proof-of-work target, visible above as
`"bits": "207fffff"` and a difficulty of roughly `4.66e-10`. That is why blocks can be
mined instantly with `generatetoaddress` instead of waiting for real hashing. The coins
have no value, the chain starts empty apart from genesis, and I control when blocks
appear. That determinism is what makes the later labs possible: I can put the chain in
an exact state and assert on it.

**Docker** is the container runtime. Each Bitcoin Core node runs as an isolated
container with its own filesystem, data directory, and network identity. This is why
the prompt in my screenshots reads `bitcoin@backend1:/$` rather than my own shell.
Containers let several nodes with independent chain state coexist on one machine
without conflicting ports or datadirs, which Lab 10 depends on directly.

**Polar** is the orchestration layer on top of Docker. It is a desktop application that
lets me define a regtest network, choose node counts and versions, and start or stop
the whole thing. It generates the container configuration, wires the nodes together as
peers, and gives each node a terminal and a GUI view. Polar does not validate anything
itself, it just manages the containers that run Bitcoin Core.

The layering is: Polar manages Docker, Docker runs Bitcoin Core, Bitcoin Core runs a
regtest chain. My Rust code sits outside all of it and shells out to `bitcoin-cli`,
which is why the `RpcClient` trait matters. The same lab functions run against a real
Polar node through `ProcessRpc` and against a deterministic `MockRpc` in the test
suite, with no change to the lab logic.

One detail worth noting from this lab: `inspect_network` deliberately fails if the
chain is not `regtest`. That guard is cheap here but it is the kind of check that stops
lab code from ever touching a real-value network by accident.
