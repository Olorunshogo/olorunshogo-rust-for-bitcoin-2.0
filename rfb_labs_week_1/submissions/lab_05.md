# Lab 05 — Broadcast and mempool

## Commands used

TODO: Record the payment, mempool, transaction, and balance commands.

```bash
# send exactly 1 BTC from miner to the receiver address, and do not mine
bitcoin-cli -rpcwallet=miner sendtoaddress <classmate_address> 1

# the node's local mempool
bitcoin-cli getrawmempool

# the sender's view of the transaction
bitcoin-cli -rpcwallet=miner gettransaction <txid>

# the receiver's balance while the payment is still unconfirmed
bitcoin-cli -rpcwallet=receiver getbalances
```

Rust entry points, from `src/labs/lab05_mempool.rs`:

| Function | RPC it drives |
|---|---|
| `send_btc` | `sendtoaddress`, reused from `lab03_maturity::attempt_payment` |
| `get_raw_mempool` | `getrawmempool`, returns the TXID list |
| `get_transaction_status` | `gettransaction`, reads `txid`, `confirmations`, `amount`, `fee`, `blockhash` |
| `observe_unconfirmed_payment` | sends, then captures mempool membership, sender status, and receiver balance in one report |

`block_hash` is modelled as `Option<String>` because `gettransaction` omits `blockhash`
entirely while a transaction is unconfirmed. Its absence is part of the evidence, not a
parsing gap. Likewise `confirmations` is `i64` rather than `u64` because Bitcoin Core
reports `-1` for a conflicted transaction.

```bash
cargo test --test lab_05
```

## Terminal output

TODO: Show the TXID, zero confirmations, mempool entry, and pending balance.

The payment, broadcast but deliberately not mined:

```text
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qgv33xv090gnld05zly6ta2k44q4nufxgq8as56 1
dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62
```

The same TXID is in the node's local mempool, so the transaction is real, valid, and
accepted for relay:

```text
$ bitcoin-cli getrawmempool
[
  "dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62"
]
```

The sender's view. `confirmations` is 0 and there is no `blockhash` field at all:

```text
$ bitcoin-cli -rpcwallet=miner gettransaction dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62
{
  "amount": -1.00000000,
  "fee": -0.00002820,
  "confirmations": 0,
  "trusted": true,
  "txid": "dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62",
  "wtxid": "12726efce86f630dde15686966622dece5e44e48d69cac7699172ba725a5ea95",
  "walletconflicts": [
  ],
  "mempoolconflicts": [
  ]
}
```

The absence of `blockhash` is the point, and it is why `WalletTransactionStatus.block_hash`
is `Option<String>` in my model rather than `String`. Compare this response with the same
call in Lab 07, where the field appears.

The receiver can see the payment but will not treat it as spendable:

```text
$ bitcoin-cli -rpcwallet=receiver getbalances
{
  "mine": {
    "trusted": 0.00000000,
    "untrusted_pending": 1.00000000,
    "immature": 0.00000000
  }
}
```

That split is the whole lab in one response. The 1 BTC is visible in
`untrusted_pending` and absent from `trusted`. The receiver knows about the payment and
declines to rely on it.

Two details worth noting. The sender's `amount` is negative, `-1.00000000`, because
`gettransaction` reports the effect on the querying wallet, and for the sender that
effect is outgoing. The `fee` of `-0.00002820` is likewise reported by the sending wallet
only, since the receiver pays nothing and its own `gettransaction` carries no fee field.
This is the same wallet-relative framing as Lab 02: the transaction is one object, but
each wallet describes it from its own side.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_05/`.

| Screenshot | Shows |
|---|---|
| [Lab_05_01_sendtoaddress.png](Evidence/Lab_05/Lab_05_01_sendtoaddress.png) | The 1 BTC send and the TXID it returned |
| [Lab_05_02_getrawmempool.png](Evidence/Lab_05/Lab_05_02_getrawmempool.png) | The same TXID present in the node's mempool |
| [Lab_05_03_sender_zero_conf.png](Evidence/Lab_05/Lab_05_03_sender_zero_conf.png) | `gettransaction` on the miner wallet showing `confirmations: 0` and no `blockhash` |
| [Lab_05_04_receiver_pending.png](Evidence/Lab_05/Lab_05_04_receiver_pending.png) | The receiver's `getbalances` with the 1 BTC in `untrusted_pending`, not `trusted` |

Captured command logs, written directly from the live `polar-n1-backend1` node:

- [Lab_05_01_unconfirmed.txt](Evidence/Lab_05/Lab_05_01_unconfirmed.txt)

## Explanation

TODO: Distinguish signed, broadcast, mempool, and confirmed states.

**The four states of a transaction.**

1. **Built and signed.** The wallet selects UTXOs, constructs inputs and outputs, and
   signs. At this point the transaction is a valid byte string that exists nowhere but
   in memory. No one else knows about it, and nothing has been committed. It can simply
   be discarded.
2. **Broadcast.** The transaction is handed to the node, which relays it to peers. This
   is an act of communication, not of settlement. Broadcasting says "here is something I
   would like included", and nothing more.
3. **In the mempool.** Each node independently validates the transaction and, if it
   passes and pays enough fee, holds it in its own memory pool of candidates for the
   next block. This is where the transaction sits in this lab. Three things about the
   mempool matter: it is per node, so different nodes can hold different sets and the
   two Polar nodes in Lab 10 demonstrate exactly that; it lives in memory, so a node
   restart drops it; and membership expires, so a transaction can be evicted for a low
   fee or replaced under RBF.
4. **Confirmed.** A miner includes the transaction in a block that extends the
   most-work chain. Only now is it part of the agreed history, and even then depth
   matters, which is the subject of Lab 08.

**Broadcast is not confirmation, and this lab proves it three ways.** The TXID exists
and is in the mempool, so the transaction is real, well formed, and accepted by the
node. The sender reports `confirmations: 0` and has no `blockhash`, so no block contains
it. And the receiver's value lands in `untrusted_pending` rather than `trusted`. The
wallet is drawing exactly the distinction this lab is about: it can see the incoming
payment, but it will not treat it as spendable, because until a block commits to it the
payment can still disappear.

**Why it can still disappear.** An unconfirmed transaction has no guarantee of ever
being mined. It can be evicted for paying too little, replaced by a higher-fee version
spending the same inputs, dropped when nodes restart, or double-spent by a conflicting
transaction that reaches miners first. Nothing has been committed to, and the mempool is
not a queue with a promise attached, only a set of candidates.

**Why a TXID exists before confirmation.** The TXID is the hash of the transaction
itself, so it is determined the moment the transaction is signed and is entirely
independent of any block. Having a TXID proves the transaction was constructed and
signed. It says nothing about whether it was accepted, and this is the exact confusion
behind treating a payment as settled because a TXID was produced.

**Practical consequence.** A merchant accepting zero confirmations is making a risk
judgement, not reading a settlement fact. The wallet's own `trusted` versus
`untrusted_pending` split encodes that judgement, and Lab 07 shows the moment the value
crosses from one to the other.
