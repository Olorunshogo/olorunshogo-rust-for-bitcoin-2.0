# Lab 09 — Multi-UTXO coin selection

## Commands used

TODO: Record funding, confirmation, spending, and decoding commands.

```bash
# create Alice's wallet and an address for her
bitcoin-cli createwallet alice
bitcoin-cli -rpcwallet=alice getnewaddress funding

# three separate 0.4 BTC sends, so Alice ends up with three distinct UTXOs
bitcoin-cli -rpcwallet=miner sendtoaddress <alice_address> 0.4
bitcoin-cli -rpcwallet=miner sendtoaddress <alice_address> 0.4
bitcoin-cli -rpcwallet=miner sendtoaddress <alice_address> 0.4

# confirm them
bitcoin-cli generatetoaddress 1 <mining_address>

# prove Alice owns three separate coins
bitcoin-cli -rpcwallet=alice listunspent

# Alice pays 1 BTC, which no single 0.4 BTC coin can cover
bitcoin-cli -rpcwallet=receiver getnewaddress payment
bitcoin-cli -rpcwallet=alice sendtoaddress <new_receiver_address> 1

# audit the resulting spend
bitcoin-cli getrawtransaction <spend_txid> 2
```

Rust entry points, from `src/labs/lab09_coin_selection.rs`:

| Function | What it does |
|---|---|
| `create_three_funding_transactions` | three separate `sendtoaddress` calls at `FUNDING_AMOUNT_BTC = 0.4` |
| `confirmed_utxos_for_address` | `listunspent` filtered to Alice's address with `confirmations > 0` |
| `send_combined_payment` | `sendtoaddress` for `COMBINED_PAYMENT_BTC = 1.0` |
| `audit_multi_utxo_spend` | sends, decodes with `lab06_decode`, and reports input count, payment, change, and fee |

The three sends are deliberately separate transactions rather than one transaction with
three outputs. One transaction paying the same address three times would also create
three UTXOs, but separate sends make each funding outpoint independently traceable, which
is what the audit compares against.

The 0.4 and 1.0 figures are chosen so that no single coin suffices and two coins at
0.8 BTC still fall short. At least three inputs are required, so multi-input selection is
forced rather than merely likely.

```bash
cargo test --test lab_09
```

## Terminal output

TODO: Show Alice's three UTXOs and the combined transaction inputs and outputs.

Alice's wallet and funding address:

```text
$ bitcoin-cli createwallet alice
{
  "name": "alice"
}
$ bitcoin-cli -rpcwallet=alice getnewaddress funding
bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe
```

Three separate 0.4 BTC sends, each its own transaction:

```text
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe 0.4
f9e512c09219432fc59f6279903c10c7c407e2c59ab3b096cdeabdfe56e73496
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe 0.4
7049556436f6d3d9317c4ddd61615b67d1471c2fe9919d8461417e297f42a8a1
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe 0.4
37195215cd00ca9540a2c815f50e6cc1f628447c854bb0cfe627ce744d25337e
```

Confirmed, then Alice owns three distinct UTXOs at the same address:

```text
$ bitcoin-cli -rpcwallet=alice listunspent
    "txid": "37195215cd00ca9540a2c815f50e6cc1f628447c854bb0cfe627ce744d25337e",
    "vout": 0,
    "address": "bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe",
    "amount": 0.40000000,
    "confirmations": 1,
    "spendable": true,

    "txid": "7049556436f6d3d9317c4ddd61615b67d1471c2fe9919d8461417e297f42a8a1",
    "vout": 1,
    "address": "bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe",
    "amount": 0.40000000,
    "confirmations": 1,
    "spendable": true,

    "txid": "f9e512c09219432fc59f6279903c10c7c407e2c59ab3b096cdeabdfe56e73496",
    "vout": 0,
    "address": "bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe",
    "amount": 0.40000000,
    "confirmations": 1,
    "spendable": true,
```

Three distinct outpoints, one address. Note the `vout` values differ: two coins are at
index 0 and one at index 1, because in that funding transaction the miner's change
happened to be placed first. Same address, same amount, different coins.

Alice's 1 BTC payment, which no single 0.4 BTC coin can cover:

```text
$ bitcoin-cli -rpcwallet=receiver getnewaddress payment
bcrt1q5e02rz2vp060uztfmdgq7av46e9hvvvt5rr7dr

$ bitcoin-cli -rpcwallet=alice sendtoaddress bcrt1q5e02rz2vp060uztfmdgq7av46e9hvvvt5rr7dr 1
2726d012ab27d250e15d3e78ef864dc587d052fae535e3cec87168227ffb4dc7
```

The decoded spend, confirmed first so that `prevout` is populated (see Lab 06):

```text
$ bitcoin-cli getrawtransaction 2726d012ab27d250e15d3e78ef864dc587d052fae535e3cec87168227ffb4dc7 2
  "txid": "2726d012ab27d250e15d3e78ef864dc587d052fae535e3cec87168227ffb4dc7",
  "vsize": 276,
  "vin": [
    { "txid": "7049556436f6d3d9317c4ddd61615b67d1471c2fe9919d8461417e297f42a8a1", "vout": 1,
      "prevout": { "value": 0.40000000,
                   "address": "bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe" } },
    { "txid": "f9e512c09219432fc59f6279903c10c7c407e2c59ab3b096cdeabdfe56e73496", "vout": 0,
      "prevout": { "value": 0.40000000,
                   "address": "bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe" } },
    { "txid": "37195215cd00ca9540a2c815f50e6cc1f628447c854bb0cfe627ce744d25337e", "vout": 0,
      "prevout": { "value": 0.40000000,
                   "address": "bcrt1qtt6v37kk8hwu5qjv6flcplhnuptyhn2fuqjlhe" } }
  ],
  "vout": [
    { "value": 1.00000000, "n": 0,
      "address": "bcrt1q5e02rz2vp060uztfmdgq7av46e9hvvvt5rr7dr" },
    { "value": 0.19994480, "n": 1,
      "address": "bcrt1q20wh6hejzqgflhqu7crmamp3g6md45wdepv0xv" }
  ],
  "fee": 0.00005520,
```

Audit:

```text
funding outpoints, all three consumed:
  7049556436f6d3d9317c4ddd61615b67d1471c2fe9919d8461417e297f42a8a1:1   0.40000000 BTC
  f9e512c09219432fc59f6279903c10c7c407e2c59ab3b096cdeabdfe56e73496:0   0.40000000 BTC
  37195215cd00ca9540a2c815f50e6cc1f628447c854bb0cfe627ce744d25337e:0   0.40000000 BTC

spend txid:    2726d012ab27d250e15d3e78ef864dc587d052fae535e3cec87168227ffb4dc7
input count:   3        (more than one, so selection was forced to combine)
sum(inputs):   1.20000000 BTC
payment:       1.00000000 BTC -> bcrt1q5e02rz2vp060uztfmdgq7av46e9hvvvt5rr7dr
change:        0.19994480 BTC -> bcrt1q20wh6hejzqgflhqu7crmamp3g6md45wdepv0xv
fee:           1.20000000 - (1.00000000 + 0.19994480) = 0.00005520 BTC

check:         1.00000000 + 0.19994480 + 0.00005520 = 1.20000000  ✓

vsize:         276 vB
fee rate:      5520 sat / 276 vB = 20.0 sat/vB
```

Every one of the three `vin` entries matches a funding outpoint recorded before the
spend, and all three were consumed in full: none of them appears in Alice's `listunspent`
afterwards, and no residual 0.4 BTC coin survives.

The comparison with Lab 06 is the useful one. That transaction had one input and a
`vsize` of 141 vB. This one has three inputs and a `vsize` of 276 vB, so it costs roughly
double at the identical 20.0 sat/vB rate. The fee rose from 2820 to 5520 satoshis purely
because more inputs mean more bytes, even though this payment moved less value than the
Lab 06 one did. That is the concrete cost side of the privacy trade-off discussed below.

The change address `bcrt1q20wh6h...` is newly generated and belongs to Alice's wallet,
not to the funding address she received at.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_09/`.

| Screenshot | Shows |
|---|---|
| [Lab_09_01_three_fundings.png](Evidence/Lab_09/Lab_09_01_three_fundings.png) | The three 0.4 BTC sends and their TXIDs |
| [Lab_09_02_alice_three_utxos.png](Evidence/Lab_09/Lab_09_02_alice_three_utxos.png) | `listunspent` proving three distinct confirmed outpoints |
| [Lab_09_03_combined_spend.png](Evidence/Lab_09/Lab_09_03_combined_spend.png) | Alice's 1 BTC send and the resulting TXID |
| [Lab_09_04_decoded_inputs.png](Evidence/Lab_09/Lab_09_04_decoded_inputs.png) | The decoded `vin` list, showing more than one input, each matching a funding outpoint |
| [Lab_09_05_outputs_and_fee.png](Evidence/Lab_09/Lab_09_05_outputs_and_fee.png) | The 1 BTC payment, the change output, and the fee arithmetic |

Captured command logs, written directly from the live `polar-n1-backend1` node:

- [Lab_09_01_three_utxos.txt](Evidence/Lab_09/Lab_09_01_three_utxos.txt)
- [Lab_09_02_combined_spend.txt](Evidence/Lab_09/Lab_09_02_combined_spend.txt)

## Explanation

TODO: Explain input combination, change, fees, and the privacy implication.

**Why more than one input was unavoidable.** Alice holds three coins of 0.4 BTC each and
needs to pay 1 BTC plus a fee. From Lab 04, a UTXO is spent in full or not at all, so
there is no way to take 1 BTC out of a 0.4 BTC coin. One coin gives 0.4, two give 0.8,
and both fall short. The wallet must select all three, which is the point of the amounts
chosen. Coin selection is the algorithm that answers "which coins do I consume", and the
UTXO model, not the wallet's preference, forces its hand here.

**Inputs are consumed completely.** Each selected outpoint appears in the `vin` list and
is thereby destroyed. After this transaction, all three of Alice's funding UTXOs cease to
exist in the UTXO set. They are not partially drawn down, and there is no residue left at
those outpoints. Attempting to spend one of them again would be a double spend and would
be rejected, since the outpoint is simply no longer in the set.

**The surplus returns as change.** Three inputs bring in 1.2 BTC to pay 1 BTC. The
difference, less the fee, comes back to Alice at a change address her wallet controls.
Following Lab 06, the fee is not an output but the unassigned remainder:

```text
sum(inputs) = payment + change + fee
```

If the wallet omitted the change output, the entire 0.2 BTC surplus would go to the miner
as fee, and the transaction would still be perfectly valid. Nothing at the consensus
level protects against that.

**Consolidation and the privacy trade-off.** This is the substantive point of the lab.
Before this transaction, Alice's three coins were three unrelated entries in the UTXO
set. An observer could see that each was paid to the same address, but if she had used
three different addresses, as good practice suggests, there would have been little
linking them. The moment she signs a single transaction spending all three, she publishes
a proof that one party controlled all three, because producing valid signatures for every
input requires holding every key.

This is the **common input ownership heuristic**, and it is the single most productive
tool in chain analysis. It applies transitively: once these three coins are linked, any
future transaction combining one of them with a fourth coin links that one too, and
clusters grow rather than shrink. Address reuse is avoidable, but this leak is structural.
Alice did not reuse an address or reveal a key. She simply made a payment larger than any
one of her coins, and the transaction structure disclosed the relationship as a side
effect.

**The change output leaks too.** An observer seeing a 1.0 BTC output and a roughly 0.2
BTC output can usually guess which is the change, using signals like round-number
payments, matching script types, or the change being the only output whose address never
appeared before. Correctly identifying change lets an analyst follow the sender's coins
forward past this transaction and keep tracking them.

**The trade-off is genuine, with no free option.** Consolidating many small UTXOs into
one is efficient: fewer inputs means smaller transactions and lower fees later, which
matters when fee rates are high. But consolidation is precisely the act that links coins
together. Keeping coins separate preserves privacy but means paying more in fees and
sometimes being unable to make a payment at all without combining. Techniques such as
using separate wallets for unrelated funds, coin control to choose inputs manually, or
CoinJoin to make the ownership heuristic unreliable all mitigate this, but none of them
eliminates it. The leak is a consequence of how the UTXO model works, not a defect in any
particular wallet.
