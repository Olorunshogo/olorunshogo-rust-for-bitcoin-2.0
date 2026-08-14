# Lab 04 — UTXOs and outpoints

## Commands used

TODO: Record the commands used to inspect and calculate wallet UTXOs.

```bash
# list every unspent output the miner wallet tracks
bitcoin-cli -rpcwallet=miner listunspent

# Bitcoin Core's own balance figure, for reconciliation
bitcoin-cli -rpcwallet=miner getbalances
```

Rust entry points, from `src/labs/lab04_utxos.rs`:

| Function | What it does |
|---|---|
| `list_unspent` | calls `listunspent` and decodes each entry into a `Utxo` |
| `decode_utxo` | maps one JSON entry to `txid`, `vout`, `address`, `scriptPubKey`, `amount`, `confirmations`, `spendable` |
| `select_spendable_utxo` | filters on `spendable`, then picks the most-confirmed output, breaking ties on `(txid, vout)` so repeat runs choose the same coin |
| `sum_spendable_utxos` | sums `amount` across spendable outputs only |
| `outpoint` | turns a `Utxo` into its `txid:vout` coordinate |

The tie-break in `select_spendable_utxo` is deliberate. Confirmation counts collide
constantly on regtest, and without a deterministic secondary key the "chosen" UTXO would
change between runs, which would make the recorded evidence unreproducible.

```bash
cargo test --test lab_04
```

## Terminal output

TODO: Include txid, vout, amount, confirmations, script, and spendable state.

The miner wallet tracks exactly one spendable output at this point, the single matured
coinbase reward from Lab 03. Descriptor fields are trimmed:

```text
$ bitcoin-cli -rpcwallet=miner listunspent
[
  {
    "txid": "5295b5ba4de09d33c979f6bc66c78689b97816e4c578564626df987157e9f3e9",
    "vout": 0,
    "address": "bcrt1q79s3z9essjqpj6629ktcg3a4zjw5jqpxt0u5k4",
    "label": "mining",
    "scriptPubKey": "0014f1611117308480196b4a2d978447b5149d490026",
    "amount": 50.00000000,
    "confirmations": 101,
    "spendable": true,
    "solvable": true,
    "safe": true
  }
]
```

Recorded fields for the selected UTXO:

| Field | Value |
|---|---|
| `txid` | `5295b5ba4de09d33c979f6bc66c78689b97816e4c578564626df987157e9f3e9` |
| `vout` | `0` |
| amount | `50.00000000` BTC |
| `confirmations` | `101` |
| address | `bcrt1q79s3z9essjqpj6629ktcg3a4zjw5jqpxt0u5k4` |
| `scriptPubKey` | `0014f1611117308480196b4a2d978447b5149d490026` |
| `spendable` | `true` |

Its outpoint:

```text
5295b5ba4de09d33c979f6bc66c78689b97816e4c578564626df987157e9f3e9:0
```

The locking script is worth reading. `0014` is a witness version 0 push of 20 bytes, and
the remaining `f1611117308480196b4a2d978447b5149d490026` is the HASH160 of the public
key. This is a P2WPKH script, and it is the same 20 bytes reported as `witness_program`
by `getaddressinfo` in Lab 02, which is what ties this coin to that address.

Reconciliation, computed independently from the entries above rather than read from the
node:

```text
sum of spendable UTXOs = 50.00000000 BTC

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 50.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 5000.00000000
  }
}
```

The independently summed figure matches `trusted` exactly. Note that it does not match
the wallet's total holdings, which are 5050 BTC once the immature rewards are counted.
That is the filter from `sum_spendable_utxos` doing its job: `listunspent` reports only
outputs the wallet can actually spend, so the 100 immature coinbase outputs never appear
in the list at all and correctly play no part in the reconciliation.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_04/`.

| Screenshot | Shows |
|---|---|
| [Lab_04_01_listunspent.png](Evidence/Lab_04/Lab_04_01_listunspent.png) | The `listunspent` array for the miner wallet |
| [Lab_04_02_selected_utxo.png](Evidence/Lab_04/Lab_04_02_selected_utxo.png) | The chosen entry with `txid`, `vout`, `amount`, `confirmations`, `address`, `scriptPubKey`, `spendable` |
| [Lab_04_03_outpoint.png](Evidence/Lab_04/Lab_04_03_outpoint.png) | The constructed `txid:vout` outpoint |
| [Lab_04_04_sum_vs_balance.png](Evidence/Lab_04/Lab_04_04_sum_vs_balance.png) | The summed spendable UTXOs next to `getbalances`, showing the two agree |

Captured command logs, written directly from the live `polar-n1-backend1` node:

- [Lab_04_01_listunspent.txt](Evidence/Lab_04/Lab_04_01_listunspent.txt)

## Explanation

TODO: Explain outpoints, UTXOs, and why a wallet balance is their sum.

**A UTXO is a coin, not a row in a ledger.** An unspent transaction output is a discrete
object created by one transaction and destroyed in full by another. It has a fixed
amount and a locking script that states the condition for spending it. There is no such
thing as spending part of a UTXO. To pay less than its value, the transaction must
consume the whole thing and create a new output paying the remainder back, which is
change.

**An outpoint is a UTXO's address in history.** `txid:vout` says "output number `vout` of
transaction `txid`". Nothing else in the chain shares that coordinate, which is why it
is the only identifier a spending input needs. `txid` alone is insufficient because one
transaction routinely creates several outputs, and `vout` alone is meaningless without
the transaction. This is also why `OutPoint` in `src/model.rs` derives `Eq`: two
outpoints are the same coin exactly when both halves match.

**Why a wallet balance is not an account entry.** Bitcoin has no accounts and no balance
field anywhere in its data structures. Nothing on chain records that an address or a
wallet "has" an amount. What exists is the UTXO set, the collection of every output that
has been created and not yet spent. A wallet computes its balance by scanning that set
for outputs whose locking scripts it can satisfy and adding them up. The number is
derived, not stored, and it is a property of the wallet's key material rather than of
the chain.

Several consequences follow, and this lab makes each of them visible:

- **The balance depends on who is asking.** A different wallet scanning the same chain
  gets a different number, because a different set of scripts is solvable. This is the
  concrete meaning of the wallet-context point from Lab 02.
- **The balance is a filtered sum, not a total.** `sum_spendable_utxos` deliberately
  skips outputs where `spendable` is false. Immature coinbase rewards from Lab 03 are on
  chain and belong to the wallet, but they cannot fund a payment, so including them
  would produce a number that does not reconcile with `trusted`.
- **Spending is selection, not subtraction.** Paying 1 BTC does not decrement a counter.
  It requires choosing specific UTXOs whose combined value covers the payment plus the
  fee, consuming them entirely, and creating new outputs. Lab 09 shows what happens when
  no single UTXO is large enough.

**Reconciling the sum against `getbalances`** is the actual check in this lab. Computing
the total independently from the raw `listunspent` entries and then getting the same
figure Bitcoin Core reports confirms that the wallet balance really is nothing more than
the sum of the coins the wallet can spend. If the two disagreed, the cause would be a
filter mismatch, typically immature or non-spendable outputs being counted, rather than
any disagreement about the chain.
