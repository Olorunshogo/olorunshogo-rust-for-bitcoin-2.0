# Lab 06 — Transaction decoding

## Commands used

TODO: Record the verbose transaction-decoding commands.

```bash
# verbosity 2 attaches each input's prevout, which is what makes the fee derivable
bitcoin-cli getrawtransaction <txid> 2
```

Rust entry points, from `src/labs/lab06_decode.rs`:

| Function | What it does |
|---|---|
| `decode_verbose_transaction` | calls `getrawtransaction <txid> 2` and builds a `DecodedTransaction` with inputs, outputs, and `vsize` |
| `input_outpoints` | returns every consumed `txid:vout` |
| `identify_payment_and_change` | finds the output paying the receiver, then treats the remaining non-`OP_RETURN` output as change |
| `calculate_fee` | `sum(inputs) - sum(outputs)`, rounded to the nearest satoshi |

Two implementation details that matter for the numbers below:

- **Verbosity 2, not 1.** At verbosity 1 each `vin` carries only `txid` and `vout`. The
  value being spent lives in the *previous* transaction, so the fee cannot be computed
  from the transaction alone. Verbosity 2 attaches `prevout.value` to each input, which
  is the only reason `calculate_fee` needs a single RPC rather than one lookup per input.
- **Verbosity 2 only yields `prevout` once the transaction is confirmed.** I found this
  by running it. Bitcoin Core builds the `prevout` and `fee` fields from the block undo
  data, which exists only for transactions that are in a block. Requesting verbosity 2
  for a transaction still sitting in the mempool returns a decode with no `prevout` on
  any input and no `fee`, so the value being spent is simply not there. The transaction
  therefore has to be confirmed before this decode can support the fee arithmetic, which
  is why the capture below shows the mempool decode and the confirmed decode separately.
- **Satoshi rounding.** BTC amounts arrive as JSON floats, and binary floating point
  cannot represent values like `0.1` exactly. Subtracting output sums from input sums
  directly produces artefacts such as `0.00002199999999`. `round_to_satoshi` multiplies
  by `100_000_000`, rounds, and divides back, so the fee lands on a whole satoshi.
  `calculate_fee` also rejects a negative result, since outputs exceeding inputs would
  mean the decode was wrong rather than that a real transaction created value.
- **`OP_RETURN` exclusion.** Change detection skips outputs whose `scriptPubKey` hex
  starts with `6a`, the `OP_RETURN` opcode. Those are provably unspendable data carriers
  and never returned value.

```bash
cargo test --test lab_06
```

## Terminal output

TODO: Include vin, vout, addresses, values, vsize, and calculated fee.

**First, the unconfirmed decode.** Verbosity 2 while the transaction is still in the
mempool. The input carries `txid` and `vout` but no `prevout`, and there is no `fee`
field anywhere in the response:

```text
$ bitcoin-cli getrawtransaction dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62 2
{
  "txid": "dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62",
  "size": 222,
  "vsize": 141,
  "weight": 561,
  "vin": [
    {
      "txid": "5295b5ba4de09d33c979f6bc66c78689b97816e4c578564626df987157e9f3e9",
      "vout": 0,
      "sequence": 4294967293
    }
  ],
  ...
}
```

**After one block, the same transaction decodes with `prevout` and `fee`.** The TXID is
unchanged, which is the Lab 07 point arriving early:

```text
$ bitcoin-cli getrawtransaction dc4d0f2c9dcee12b0d400b0b61cea1984e98cc7effef2cec16fe53a083da5a62 2
  "vin": [
    {
      "txid": "5295b5ba4de09d33c979f6bc66c78689b97816e4c578564626df987157e9f3e9",
      "vout": 0,
      "prevout": {
        "generated": true,
        "height": 3,
        "value": 50.00000000,
        "scriptPubKey": {
          "hex": "0014f1611117308480196b4a2d978447b5149d490026",
          "address": "bcrt1q79s3z9essjqpj6629ktcg3a4zjw5jqpxt0u5k4",
          "type": "witness_v0_keyhash"
        }
      },
      "sequence": 4294967293
    }
  ],
  "fee": 0.00002820,
```

`"generated": true` marks the input as a coinbase output, and `"height": 3` is where it
was created, which matches the block mined in Lab 03.

**The outputs.** Two of them, for one payment:

```text
  "vout": [
    {
      "value": 48.99997180,
      "n": 0,
      "scriptPubKey": {
        "hex": "00144c169d62a457c153322e4e2e1083c1ba14068134",
        "address": "bcrt1qfstf6c4y2lq4xv3wfchppq7phg2qdqf596mq94",
        "type": "witness_v0_keyhash"
      }
    },
    {
      "value": 1.00000000,
      "n": 1,
      "scriptPubKey": {
        "hex": "001443231331e57a27f6be82f934beaad5a82b3e24c8",
        "address": "bcrt1qgv33xv090gnld05zly6ta2k44q4nufxgq8as56",
        "type": "witness_v0_keyhash"
      }
    }
  ]
```

Output `n: 1` pays the `classmate` address from Lab 02, so that is the payment. Output
`n: 0` pays `bcrt1qfstf6c...` which appears nowhere in this lab so far, and it is the
change returning to a freshly generated address in the miner wallet. Note that the
change is output index 0 and the payment is index 1, so output order carries no meaning.

**Consumed outpoint:**

```text
5295b5ba4de09d33c979f6bc66c78689b97816e4c578564626df987157e9f3e9:0
```

That is exactly the UTXO selected in Lab 04, now destroyed in full.

**Value conservation, with actual values:**

```text
sum(inputs)  = 50.00000000 BTC
payment      =  1.00000000 BTC  -> bcrt1qgv33xv090gnld05zly6ta2k44q4nufxgq8as56
change       = 48.99997180 BTC  -> bcrt1qfstf6c4y2lq4xv3wfchppq7phg2qdqf596mq94
fee          = 50.00000000 - (1.00000000 + 48.99997180)
             =  0.00002820 BTC

check:          1.00000000 + 48.99997180 + 0.00002820 = 50.00000000  ✓

vsize        = 141 vB
fee rate     = 2820 sat / 141 vB = 20.0 sat/vB
```

The independently computed fee of `0.00002820` matches both the `fee` field in the
confirmed decode and the `-0.00002820` the sending wallet reported in Lab 05, from three
different sources. The fee rate lands on exactly 20.0 sat/vB, which is the wallet's
default fee estimate on regtest, where there is no real fee market to estimate from.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_06/`.

| Screenshot | Shows |
|---|---|
| [Lab_06_01_decode_verbose.png](Evidence/Lab_06/Lab_06_01_decode_verbose.png) | `getrawtransaction <txid> 2` in full |
| [Lab_06_02_inputs_prevout.png](Evidence/Lab_06/Lab_06_02_inputs_prevout.png) | Each `vin` with its `txid:vout` and `prevout.value` |
| [Lab_06_03_outputs.png](Evidence/Lab_06/Lab_06_03_outputs.png) | Both outputs, identifying the 1 BTC payment and the change |
| [Lab_06_04_vsize_and_fee.png](Evidence/Lab_06/Lab_06_04_vsize_and_fee.png) | `vsize` and the arithmetic reconciling inputs, outputs, and fee |

Captured command logs, written directly from the live `polar-n1-backend1` node:

- [Lab_06_01_decode.txt](Evidence/Lab_06/Lab_06_01_decode.txt)
- [Lab_06_02_decode_confirmed.txt](Evidence/Lab_06/Lab_06_02_decode_confirmed.txt)

## Explanation

TODO: Prove value conservation and explain why the fee has no dedicated output.

**Value conservation is enforced, not conventional.** Consensus requires that the sum of
the values of the outputs a transaction creates never exceeds the sum of the values of
the outputs it consumes. A transaction violating that is rejected by every node, so it
cannot be mined regardless of fees or miner cooperation. The only exception is the
coinbase transaction, which has no real inputs and is allowed to create the block
subsidy up to the schedule limit.

**The fee is the gap, not an output.** Look at the structure: a transaction is a list of
inputs and a list of outputs, and there is no fee field anywhere in the serialization.
The fee is defined implicitly as

```text
fee = sum(input values) - sum(output values)
```

Whatever the transaction consumes but does not reassign is claimed by the miner, who
adds it to the coinbase output of the block containing the transaction. This design has
real consequences:

- **The fee cannot be read from the transaction in isolation.** Input values are not in
  the transaction, only the outpoints pointing at where those values live. Computing a
  fee requires looking up every previous output, which is exactly why verbosity 2 exists
  and why a node needs either the full chain or a UTXO index to report fees.
- **Forgetting change is a real and irreversible loss.** Since the fee is the leftover,
  a transaction that consumes a 50 BTC UTXO and creates a single 1 BTC output has not
  made a mistake by any consensus rule. It has paid a 49 BTC fee, and it is valid. There
  is no protection against this at the protocol level. Wallets add change outputs
  automatically for precisely this reason.
- **Fees are bid per byte, not per amount.** Block space is the scarce resource, so
  miners rank by fee rate, which is why `vsize` matters. `vsize` is weight units divided
  by four, the segwit-aware size measure, so witness data counts at a discount against
  the block limit. Fee divided by `vsize` gives sat/vB, the number that actually decides
  inclusion priority. Sending 1 BTC or 1000 BTC costs the same if the transactions are
  the same size.

**Why there are two outputs when one payment was made.** The payment output sends
exactly 1 BTC to the receiver address. The change output returns the remainder to an
address the sending wallet controls. This is forced by the UTXO model from Lab 04: an
input consumes its entire previous output, so any surplus above the payment and fee must
be explicitly sent back or it is silently forfeited to the miner. The change address is
freshly generated rather than reusing the sending address, which is a privacy measure,
though as Lab 09 shows, change is also one of the strongest heuristics chain analysts
use to link transactions.

**Identifying which output is which.** Nothing in the transaction labels an output as
"payment" or "change". Both are just scripts and values. `identify_payment_and_change`
resolves it by matching the known receiver address first and treating the remaining
spendable output as change. An outside observer without that knowledge has to guess, and
that ambiguity is a genuine, if modest, privacy property.
