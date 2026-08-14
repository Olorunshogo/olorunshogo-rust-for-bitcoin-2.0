# Lab 08 — Block security

## Commands used

TODO: Record block-header inspection and additional mining commands.

```bash
# verbose header of the block that confirmed the payment
bitcoin-cli getblockheader <block_hash>

# depth before mining further
bitcoin-cli -rpcwallet=receiver gettransaction <txid>

# five more blocks
bitcoin-cli generatetoaddress 5 <mining_address>

# depth after
bitcoin-cli -rpcwallet=receiver gettransaction <txid>
```

Rust entry points, from `src/labs/lab08_security.rs`:

| Function | What it does |
|---|---|
| `get_block_header` | `getblockheader`, decodes `hash`, `height`, `previousblockhash`, `merkleroot`, `nonce`, `difficulty`, `bits`, `confirmations`, `chainwork` |
| `mine_additional_blocks` | `generatetoaddress <count>` |
| `get_confirmations` | reads `confirmations` via `lab07_confirm::transaction_confirmations` |
| `build_security_report` | records the header and depth, mines `ADDITIONAL_CONFIRMATIONS = 5`, records depth again |

`previous_block_hash` is `Option<String>` because the genesis block genuinely has no
parent. Every other field is required, and a missing one is a decode error rather than a
default.

```bash
cargo test --test lab_08
```

## Terminal output

TODO: Show header fields and confirmation count changing from one to six.

The confirming block's header:

```text
$ bitcoin-cli getblockheader 610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68
{
  "hash": "610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68",
  "confirmations": 1,
  "height": 104,
  "version": 536870912,
  "versionHex": "20000000",
  "merkleroot": "8430e47096e634fb3b5e709e829be4a9ab3ecf8ef5e522f93e15f8ae1fd52f41",
  "time": 1785761333,
  "mediantime": 1785761259,
  "nonce": 0,
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000d2",
  "nTx": 2,
  "previousblockhash": "26f3a28774438838a4108cb90f7fc59f992a31ec1c1344e7b07a27b19a1f3e0c"
}
```

Recorded header fields:

| Field | Value |
|---|---|
| `hash` | `610edad4a58ee42a390b0a02d96d687b5cb92a11610466ab07b4eba88ac73d68` |
| `height` | `104` |
| `previousblockhash` | `26f3a28774438838a4108cb90f7fc59f992a31ec1c1344e7b07a27b19a1f3e0c` |
| `merkleroot` | `8430e47096e634fb3b5e709e829be4a9ab3ecf8ef5e522f93e15f8ae1fd52f41` |
| `nonce` | `0` |
| `bits` | `207fffff` |
| `target` | `7fffff00...00` |
| `difficulty` | `4.656542373906925e-10` |
| `chainwork` | `00000000...00d2` |
| `confirmations` before | `1` |
| `confirmations` after 5 blocks | `6` |

Depth going from one to six:

```text
$ bitcoin-cli -rpcwallet=receiver gettransaction dc4d0f2c...5a62
  "confirmations": 1,

$ bitcoin-cli generatetoaddress 5 bcrt1q79s3z9essjqpj6629ktcg3a4zjw5jqpxt0u5k4
[
  "525ad6acf843b4432472d2b85ffb894bccc1a4fb8df7f2f9fe1d77b10f9df06c",
  "066564cffbc9a29a543571515ba4c63d93458f4dddc14c82bb37b862bfd49d78",
  "4e271b3706cf48e3cfb24967cc30707bc86a4f34fc35bb72b58fe05603ff74ca",
  "6dcb7ee0e20e367a28556c109e3b1c4046cc349a76c15c8607b2e3d99680d2c0",
  "2d5d1d51df1e55467e67ddccbc229326df83b7f5f4de111e4e2c1e6813d49ff8"
]

$ bitcoin-cli -rpcwallet=receiver gettransaction dc4d0f2c...5a62
  "confirmations": 6,
```

Two observations from the real values.

**`"nonce": 0` is the regtest target made visible.** On mainnet the nonce is the record of
an enormous search, typically a large arbitrary-looking number. Here the very first
header tried already hashed below the `207fffff` target, so no search was needed at all.
The mechanism is identical to mainnet, the threshold is just set so low that the work is
trivially found.

**`chainwork` moved from `...0004` to `...00d2`.** In Lab 01 the chain was 1 block deep
with chainwork 4. At height 104 it is `0xd2`, which is 210. Chainwork accumulates two per
block on regtest at this target, and it is a running total over the whole chain rather
than a per-block figure. That total is precisely what Lab 10 compares between competing
branches. Note also that the header's own `chainwork` stayed at `...00d2` after five more
blocks were mined while its `confirmations` rose to 6: chainwork is a property of the
block's position in history, depth is a property of the current tip.

`previousblockhash` here is `26f3a287...3e0c`, which is the hash returned as the last of
the 100 blocks mined in Lab 03. The hash link between those two labs is literal.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_08/`.

| Screenshot | Shows |
|---|---|
| [Lab_08_01_getblockheader.png](Evidence/Lab_08/Lab_08_01_getblockheader.png) | The confirming block's full verbose header |
| [Lab_08_02_confirmations_one.png](Evidence/Lab_08/Lab_08_02_confirmations_one.png) | The payment at one confirmation before extra mining |
| [Lab_08_03_mine_five.png](Evidence/Lab_08/Lab_08_03_mine_five.png) | `generatetoaddress 5` and the five returned block hashes |
| [Lab_08_04_confirmations_six.png](Evidence/Lab_08/Lab_08_04_confirmations_six.png) | The same payment now at six confirmations |
| [Lab_08_05_chainwork.png](Evidence/Lab_08/Lab_08_05_chainwork.png) | Accumulated chainwork, which Lab 10 uses as the tiebreaker between branches |

Captured command logs, written directly from the live `polar-n1-backend1` node:

- [Lab_08_01_header_and_depth.txt](Evidence/Lab_08/Lab_08_01_header_and_depth.txt)

## Explanation

TODO: Explain hash links, Merkle roots, proof of work, and confirmation depth.

**Hash links make history append-only.** Every header contains `previousblockhash`, the
hash of its parent's header. Because a block's own hash is computed over its header,
including that pointer, changing anything in an old block changes its hash, which breaks
the `previousblockhash` of its child, and so on to the tip. There is no way to edit one
block in place. Rewriting history means rebuilding every block from the edit forward,
and each of those rebuilds needs fresh proof of work. That is the whole reason depth
translates into security.

**The Merkle root commits to the transaction list.** `merkleroot` is the root of a binary
hash tree over every transaction in the block, and it is one small field in an 80-byte
header. Two properties follow. First, it is a commitment: change, add, remove, or reorder
any transaction and the root changes, so the header no longer matches its own contents
and the block is rejected. This is what connects Lab 07's membership proof to the header
inspected here, since the TXID found in the block's `tx` array is a leaf of this tree.
Second, it enables compact inclusion proofs. Proving a transaction is in a block needs
only the path of sibling hashes from leaf to root, about log2(n) hashes rather than the
whole block, which is what makes SPV clients possible.

**Proof of work is a search, not a calculation.** `bits` encodes a target threshold, and
a block is valid only if the hash of its header, interpreted as a number, is below that
target. The hash function is not invertible, so there is no way to construct a header
that hashes low. The only method is to vary the `nonce` field, and then other mutable
fields once the nonce space is exhausted, and hash repeatedly until a result happens to
fall under the target. `difficulty` is the same threshold expressed relative to the
easiest allowed target. On regtest `bits` is `207fffff`, an almost trivially high target,
which is why `generatetoaddress` returns instantly. On mainnet the same mechanism costs
enormous energy, and that cost is the security. The work is hard to produce and trivial
to verify, which is exactly the asymmetry the system needs.

**Chainwork is the honest measure, not height.** `chainwork` is the cumulative expected
number of hashes needed to build the chain up to this block. It matters because height
can be gamed while work cannot. A branch of many easy blocks can be taller than a branch
of fewer hard ones, so height is not a reliable signal of effort. Nodes compare
`chainwork` when deciding between competing branches, which is precisely what Lab 10
demonstrates.

**Why confirmations raise the cost of a reorganization.** To remove a transaction that is
buried `n` blocks deep, an attacker must build an alternative branch from before the
containing block that has more accumulated work than the current chain, then broadcast
it. That means redoing the work of all `n` blocks while the honest network keeps
extending its own chain. The cost compounds with depth, and unless the attacker
controls a majority of hash power the probability of ever catching up falls off sharply.
Six confirmations is a convention, not a rule in the code, marking the depth at which the
cost is considered prohibitive for ordinary amounts.

**What confirmations do not do.** Depth buys ordering confidence, and nothing else.
Burying an invalid transaction under a thousand blocks does not make it valid, because
every node validates independently and rejects any block containing an invalid
transaction outright, regardless of how much work sits on top of it. A chain built on an
invalid block is not a competing chain that loses on work, it is not a chain at all from
the perspective of a validating node. This is why a node's ordering, expressed by
chainwork, is only ever applied among branches that are already fully valid, and it is
why running your own node matters: work decides between valid options, but validity is
never up for a vote.
