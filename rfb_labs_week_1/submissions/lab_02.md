# Lab 02 — Wallets and addresses

## Commands used

TODO: Record how you created and inspected both wallets and addresses.

```bash
# alias set first so every call targets regtest
alias bitcoin-cli="bitcoin-cli -regtest"

# create both wallets on backend1
bitcoin-cli createwallet miner
bitcoin-cli createwallet receiver

# confirm both are loaded by the node
bitcoin-cli listwallets

# generate one labelled address from each wallet
bitcoin-cli -rpcwallet=miner getnewaddress mining
bitcoin-cli -rpcwallet=receiver getnewaddress classmate

# ask each wallet whether it controls its own address
bitcoin-cli -rpcwallet=miner getaddressinfo <mining_address>
bitcoin-cli -rpcwallet=receiver getaddressinfo <classmate_address>

# cross-check: ask the wrong wallet about the other wallet's address
bitcoin-cli -rpcwallet=receiver getaddressinfo <mining_address>
```

Rust entry points, from `src/labs/lab02_wallets.rs`:

| Function | RPC it drives | Wallet context |
|---|---|---|
| `create_wallet` | `createwallet <name>` | none, node-wide |
| `list_wallets` | `listwallets` | none, node-wide |
| `get_new_address` | `getnewaddress <label>` | `-rpcwallet=<name>` |
| `address_belongs_to_wallet` | `getaddressinfo <address>`, reads `ismine` | `-rpcwallet=<name>` |

```bash
cargo test --test lab_02
```

## Terminal output

TODO: Include loaded wallets, addresses, and ownership evidence.

Both wallets created and loaded. The empty name `""` is Polar's default wallet, which
already existed on the node:

```text
$ bitcoin-cli createwallet miner
{
  "name": "miner"
}
$ bitcoin-cli createwallet receiver
{
  "name": "receiver"
}
$ bitcoin-cli listwallets
[
  "",
  "miner",
  "receiver"
]
```

Both addresses carry the `bcrt1` regtest prefix:

```text
$ bitcoin-cli -rpcwallet=miner getnewaddress mining
bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx

$ bitcoin-cli -rpcwallet=receiver getnewaddress classmate
bcrt1qdv4x83ylqzz5pl54pxqjewuzcfepcd0t0szsjq
```

Ownership, asked of the correct wallet in each case. Descriptor fields are trimmed:

```text
$ bitcoin-cli -rpcwallet=miner getaddressinfo bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx
{
  "address": "bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx",
  "scriptPubKey": "0014487ccfe673132d5b3e642960a43d7fd38927d9ec",
  "ismine": true,
  "solvable": true,
  "iswatchonly": false,
  "isscript": false,
  "iswitness": true,
  "witness_version": 0,
  "witness_program": "487ccfe673132d5b3e642960a43d7fd38927d9ec",
  "labels": [
    "mining"
  ]
}

$ bitcoin-cli -rpcwallet=receiver getaddressinfo bcrt1qdv4x83ylqzz5pl54pxqjewuzcfepcd0t0szsjq
{
  "address": "bcrt1qdv4x83ylqzz5pl54pxqjewuzcfepcd0t0szsjq",
  "scriptPubKey": "00146b2a63c49f008540fe9509812cbb82c2721c35eb",
  "ismine": true,
  "solvable": true,
  "iswatchonly": false,
  "isscript": false,
  "iswitness": true,
  "witness_version": 0,
  "witness_program": "6b2a63c49f008540fe9509812cbb82c2721c35eb",
  "labels": [
    "classmate"
  ]
}
```

The negative half of the proof. Asking the receiver wallet about the miner's address
succeeds and returns a real answer, it is simply the wrong one to act on:

```text
$ bitcoin-cli -rpcwallet=receiver getaddressinfo bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx
{
  "address": "bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx",
  "scriptPubKey": "0014487ccfe673132d5b3e642960a43d7fd38927d9ec",
  "ismine": false,
  "solvable": false,
  "iswatchonly": false,
  "isscript": false,
  "iswitness": true,
  "witness_version": 0,
  "witness_program": "487ccfe673132d5b3e642960a43d7fd38927d9ec",
  "labels": [
  ]
}
```

The same address returns `ismine: true` under `miner` and `ismine: false` under
`receiver`, and the `labels` array is empty in the second case because the receiver
wallet has never seen that address. `solvable` also drops to `false`: without the key,
the wallet could not build a spending witness even if it wanted to.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_02/`.

| Screenshot | Shows |
|---|---|
| [Lab_02_01_createwallets.png](Evidence/Lab_02/Lab_02_01_createwallets.png) | A `createwallet` call succeeding on `backend1`, the first wallet created on the node |
| [Lab_02_createwallet_getnewwallet_getaddressinfo.png](Evidence/Lab_02/Lab_02_createwallet_getnewwallet_getaddressinfo.png) | The regtest alias, `createwallet miner`, `createwallet receiver`, `listwallets` returning `["", "miner", "receiver"]`, both `getnewaddress` calls, and `getaddressinfo` on the `mining` address under `miner` with `ismine: true` |
| [Lab_02_receiver_getaddressinfo.png](Evidence/Lab_02/Lab_02_receiver_getaddressinfo.png) | `getaddressinfo` on the `classmate` address under `receiver` with `ismine: true`, followed by the same wallet asked about the miner's address returning `ismine: false` |

The two Polar terminal screenshots are one continuous session, the second scrolled down
from the first, so the positive and negative ownership checks are visibly part of the
same run against `polar-n1-backend1`.

## Explanation

TODO: Explain wallet context and the purpose of `-rpcwallet`.

A Bitcoin Core node can have many wallets loaded at once. The node is a single process
with a single chain, but each wallet is a separate database of keys, addresses, labels,
and the subset of chain history relevant to those keys. Two categories of RPC follow
from that.

**Node-wide RPCs** describe the chain or the node and have no wallet at all.
`listwallets`, `getblockcount`, and `getbestblockhash` are in this group. Adding
`-rpcwallet` to them is meaningless because there is no per-wallet answer.

**Wallet-scoped RPCs** only make sense relative to one wallet's key set.
`getnewaddress`, `getaddressinfo`, `getbalances`, and `sendtoaddress` are in this group.
When more than one wallet is loaded, Bitcoin Core cannot guess which one I mean, so it
refuses the call rather than picking arbitrarily. `-rpcwallet=<name>` supplies that
missing context, and it is why my `RpcClient::call` takes `wallet: Option<&str>` and
turns `Some(name)` into a `-rpcwallet=` argument.

**What a wrong wallet context actually means.** It is usually not an error, and that is
the dangerous part. If I ask the receiver wallet about the miner's address,
`getaddressinfo` succeeds and returns `ismine: false`. That is a correct answer to the
question I asked, but if I believed I was querying the miner wallet, I would read it as
"the miner does not own its own address". The same trap applies to balances: querying
the wrong wallet returns a real balance, just not the one I wanted. With
`sendtoaddress`, a wrong wallet context spends the wrong coins.

This is exactly why `address_belongs_to_wallet` takes the wallet name and the address
as two separate arguments rather than inferring either. The pairing is the assertion
being made. Proving that the `mining` address is `ismine` under `miner` and not under
`receiver` establishes ownership both positively and negatively.

**The `bcrt1` prefix** is the regtest bech32 human-readable part. Mainnet native segwit
addresses start `bc1` and testnet `tb1`, so the prefix alone identifies the network.
Seeing `bcrt1...` on both addresses confirms the wallets were created on the regtest
chain and that no mainnet material is involved anywhere in this lab. The prefix is not
decoration, it is covered by the bech32 checksum, so a mainnet wallet rejects a `bcrt1`
address outright rather than sending real coins to it.
