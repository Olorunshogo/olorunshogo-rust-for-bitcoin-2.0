# Lab 07 — Confirmation and block membership

## Commands used

TODO: Record the mining, mempool, transaction, and block commands.

```bash
# mine exactly one block
bitcoin-cli generatetoaddress 1 <mining_address>

# the mempool should now be empty
bitcoin-cli getrawmempool

# the sender's view: one confirmation, and now a blockhash
bitcoin-cli -rpcwallet=miner gettransaction <txid>

# the receiver's balance should have moved to trusted
bitcoin-cli -rpcwallet=receiver getbalances

# the containing block's transaction list
bitcoin-cli getblock <block_hash> 1
```

Rust entry points, from `src/labs/lab07_confirm.rs`:

| Function | What it does |
|---|---|
| `mine_one_block` | `generatetoaddress 1`, returns the single block hash |
| `mempool_is_empty` | `getrawmempool`, true when the list is empty |
| `transaction_confirmations` | `gettransaction`, reads `confirmations` |
| `confirm_and_locate_transaction` | mines, checks the mempool, reads `confirmations` and `blockhash` from one wallet lookup, then verifies the TXID appears in that block's `tx` array |

`getblock` is called at verbosity 1, which returns `tx` as an array of TXID strings.
Verbosity 2 would return full transaction objects, which is more data than the
membership check needs.

```bash
cargo test --test lab_07
```

## Terminal output

TODO: Show the empty mempool, confirmation count, block hash, and TXID in block.

One block mined:

```text
$ bitcoin-cli generatetoaddress 1 bcrt1q79s3z9essjqpj6629ktcg3a4zjw5jqpxt0u5k4
[
  "610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68"
]
```

The TXID left the mempool:

```text
$ bitcoin-cli getrawmempool
[
]
```

The sender now reports one confirmation and, unlike Lab 05, a `blockhash`:

```text
$ bitcoin-cli -rpcwallet=miner gettransaction dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62
{
  "amount": -1.00000000,
  "fee": -0.00002820,
  "confirmations": 1,
  "blockhash": "610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68",
  "blockheight": 104,
  "blockindex": 1,
  "blocktime": 1785761333,
  "txid": "dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62"
}
```

The receiver's balance crossed from pending to trusted:

```text
$ bitcoin-cli -rpcwallet=receiver getbalances
{
  "mine": {
    "trusted": 1.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 0.00000000
  }
}
```

And the block itself contains the TXID, verified from the chain rather than from the
wallet:

```text
$ bitcoin-cli getblock 610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68 1
{
  "hash": "610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68",
  "confirmations": 1,
  "height": 104,
  "merkleroot": "8430e47096e634fb3b5e709e829be4a9ab3ecf8ef5e522f93e15f8ae1fd52f41",
  "nonce": 0,
  "bits": "207fffff",
  "nTx": 2,
  "previousblockhash": "26f3a28774438838a4108cb90f7fc59f992a31ec1c1344e7b07a27b19a1f3e0c",
  "tx": [
    "3585ce00c54b0ea22d0fa2817220ee26ba9e5c4884ad5be018b9eef997f69108",
    "dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62"
  ]
}
```

Two transactions in the block. The first is the coinbase paying the block reward, always
at index 0. The second is my payment, which matches the `"blockindex": 1` the wallet
reported above.

**The TXID is unchanged.** `dc4d0f2c...5a62` here is the identical string returned by
`sendtoaddress` in Lab 05, before any block existed. The transaction was not rewritten,
re-signed, or renumbered by being mined. Only its context changed.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_07/`.

| Screenshot | Shows |
|---|---|
| [Lab_07_01_mine_one_block.png](Evidence/Lab_07/Lab_07_01_mine_one_block.png) | `generatetoaddress 1` and the returned block hash |
| [Lab_07_02_mempool_empty.png](Evidence/Lab_07/Lab_07_02_mempool_empty.png) | `getrawmempool` returning `[]`, so the TXID left the mempool |
| [Lab_07_03_one_confirmation.png](Evidence/Lab_07/Lab_07_03_one_confirmation.png) | `gettransaction` with `confirmations: 1` and a populated `blockhash` |
| [Lab_07_04_receiver_trusted.png](Evidence/Lab_07/Lab_07_04_receiver_trusted.png) | The receiver's 1 BTC moved from `untrusted_pending` to `trusted` |
| [Lab_07_05_txid_in_block.png](Evidence/Lab_07/Lab_07_05_txid_in_block.png) | `getblock` with the TXID present in the block's `tx` list |

Captured command logs, written directly from the live `polar-n1-backend1` node:

- [Lab_07_01_confirmed.txt](Evidence/Lab_07/Lab_07_01_confirmed.txt)

## Explanation

TODO: Explain exactly what changed when the transaction became confirmed.

**Mining did not change the transaction.** This is the central point of the lab. The
serialized bytes are byte-for-byte identical to what was broadcast in Lab 05. The same
inputs, the same outputs, the same signatures, and therefore the same TXID, since the
TXID is the hash of those bytes. If mining had altered anything the TXID would differ,
and the wallet would not be able to find the transaction at all.

**What changed is the transaction's place in history.** Before mining, the transaction
was a candidate held in each node's local mempool. Mempools are per node and hold no
authority. After mining, the transaction is committed inside a block whose header
contains a Merkle root over its transaction list, and that header is chained to its
predecessor and backed by proof of work. The transaction has moved from "something a
node is willing to relay" to "something the network's agreed history contains".

**The five proofs, and why each is needed.**

- *The TXID left the mempool.* The mempool holds candidates awaiting inclusion. Once
  included, the transaction is no longer pending, so the node drops it. An empty mempool
  is the negative half of the proof.
- *There is a `blockhash`.* In Lab 05 this field was absent entirely, which is why my
  model types it as `Option<String>`. Its presence is the wallet naming the specific
  block that contains the transaction.
- *`confirmations` is 1.* Depth counts the containing block itself, so a transaction in
  the tip has exactly one confirmation. Each block mined on top adds one, which Lab 08
  extends to six.
- *The receiver's balance became trusted.* The wallet's own risk classification changed.
  The same 1 BTC that was `untrusted_pending` is now `trusted` and can fund a payment.
- *The block's `tx` array contains the TXID.* This is the one that closes the loop. The
  first four are all the wallet's account of things. Fetching the block independently
  and finding the TXID in its transaction list confirms membership from the chain side
  rather than taking the wallet's word for it.

**Why one confirmation is not the end.** A one-confirmation transaction lives in the
current tip, and the tip is the single most replaceable block in the chain. Any
competing block found at the same height can displace it, at which point the transaction
returns to the mempool and its confirmation count falls back to zero. Nothing about it
becomes invalid, it simply becomes uncommitted again. Depth is what converts inclusion
into practical finality, and that is what Lab 08 measures and Lab 10 demonstrates by
actually causing a reorganization.

**Confirmation is not validation.** The transaction was already fully valid when it was
signed in Lab 05, and every node had verified it before accepting it into the mempool.
Mining did not make it valid; it made it *ordered*. What blocks provide is agreement on
sequence, which is what solves double spending. Validity is a property of the
transaction, agreed ordering is a property of the chain, and this lab is the moment the
second one is acquired.
