# Lab 03 — Coinbase maturity

## Commands used

TODO: Record mining, balance inspection, and premature-spend commands.

Addresses carried over from Lab 02:

- mining (`miner` wallet): `bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx`
- classmate (`receiver` wallet): `bcrt1qdv4x83ylqzz5pl54pxqjewuzcfepcd0t0szsjq`

```bash
# mine exactly one block to the miner address
bitcoin-cli generatetoaddress 1 <mining_address>

# state after that single block
bitcoin-cli getblockcount
bitcoin-cli -rpcwallet=miner getbalances

# attempt to spend the immature reward, this is expected to fail
bitcoin-cli -rpcwallet=miner sendtoaddress <classmate_address> 1

# mine 100 more blocks, taking the chain to height 101
bitcoin-cli generatetoaddress 100 <mining_address>

# state after maturity
bitcoin-cli getblockcount
bitcoin-cli -rpcwallet=miner getbalances
```

Rust entry points, from `src/labs/lab03_maturity.rs`:

| Function | RPC it drives |
|---|---|
| `mine_blocks` | `generatetoaddress <count> <address>`, returns the block hashes |
| `get_balances` | `getbalances`, reads `mine.trusted`, `mine.untrusted_pending`, `mine.immature` |
| `attempt_payment` | `sendtoaddress <address> <amount>`, returns the TXID or the Core error |
| `demonstrate_coinbase_maturity` | mines 1, records height and balances, captures the rejection, mines 100, records again |

`demonstrate_coinbase_maturity` treats a successful premature spend as a failure. If
`attempt_payment` returns a TXID at height 1, the function returns
`LabError::Parse("premature spend unexpectedly succeeded as <txid>")`, because that
would mean the maturity rule was not enforced.

```bash
cargo test --test lab_03
```

## Terminal output

TODO: Show balances at heights 1 and 101 plus the failed premature spend.

**Note on heights.** My Polar network was already at height 2 when I ran this lab, since
Polar mines a couple of blocks when a network is first started. Every height below is
therefore offset by 2 from the fresh-chain figures in LABS.md: the first mined block
lands at 3 rather than 1, and the chain finishes at 103 rather than 101. The rule being
demonstrated is unaffected, because maturity is measured in depth below the tip, not in
absolute height.

One block mined. The reward exists but is entirely immature:

```text
$ bitcoin-cli generatetoaddress 1 bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx
[
  "7c0861f3ca35dae1cac41b5f4edd3e9f489675043ee4ff428b50df89f6b5845a"
]

$ bitcoin-cli getblockcount
3

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 0.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 100.00000000
  },
  "lastprocessedblock": {
    "hash": "7c0861f3ca35dae1cac41b5f4edd3e9f489675043ee4ff428b50df89f6b5845a",
    "height": 3
  }
}
```

`immature` reads 100 BTC rather than 50 because the `miner` wallet holds two coinbase
outputs at this point, both paid to the same mining address, at 50 BTC each. Neither is
spendable, which is the only thing that matters for this lab: `trusted` is still zero.

The premature spend, rejected. This is the evidence the lab asks to preserve:

```text
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qdv4x83ylqzz5pl54pxqjewuzcfepcd0t0szsjq 1
error code: -6
error message:
Insufficient funds
```

`Insufficient funds` is worth reading carefully. The wallet holds 100 BTC, so the message
looks wrong at first glance. It is not. Coin selection only ever considers spendable
outputs, and the immature coinbase is not one, so from the selector's point of view there
is genuinely nothing to fund the payment with. `trusted: 0.00000000` above is the same
fact stated as a number.

After 100 more blocks the early rewards mature while the rest do not:

```text
$ bitcoin-cli generatetoaddress 100 bcrt1qfp7vlennzvk4k0ny99s2g0tl6wyj0k0v3h0klx
$ bitcoin-cli getblockcount
103
$ bitcoin-cli -rpcwallet=miner getbalances
```

The two rewards from heights 2 and 3 cross 100 confirmations once the tip reaches 103,
so they move into `trusted` while the 100 rewards mined by this command stay `immature`,
each waiting for its own depth. That split is the rule made visible in a single response.

I did not capture a screenshot of this final step, so the numbers above are limited to
what the commands were rather than a recorded response. The screenshot in the evidence
section covers the state at height 3, which is where the rule is actually demonstrated.

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_03/`.

| Screenshot | Shows |
|---|---|
| [Lab_03_01_generatetoaddress_getblockcount_getbalances.png](Evidence/Lab_03/Lab_03_01_generatetoaddress_getblockcount_getbalances.png) | `generatetoaddress 1` to the mining address returning block `7c0861f3...5845a`, `getblockcount` returning `3`, and `getbalances` with `trusted: 0.00000000` against `immature: 100.00000000`, with the `sendtoaddress` attempt beginning on the next line |

This single screenshot covers the first three commands of the lab, which together are
the whole demonstration: a reward exists on-chain, the wallet reports it as immature,
and the spendable balance is zero. The `sendtoaddress` rejection and the later 100-block
run were executed in the same session but were not captured as images.

## Explanation

TODO: Explain why the first coinbase reward becomes spendable at height 101.

**The rule.** A coinbase output, the transaction that pays a block's subsidy and fees to
the miner, cannot be spent until 100 further blocks have been built on top of the block
that created it. `COINBASE_MATURITY = 100` in Bitcoin Core. Ordinary transaction outputs
have no such restriction and are spendable at one confirmation.

**Why the rule exists.** Coinbase outputs are the one kind of output that can be
destroyed by a chain reorganization rather than merely re-mined. An ordinary transaction
that gets reorganized out of a block is still valid and usually returns to the mempool
to be mined again. A coinbase transaction is bound to the specific block that created
it. If that block becomes stale, the coinbase output ceases to exist entirely, along
with every transaction descending from it. Without a maturity delay, a miner could spend
a fresh reward, that spend could propagate widely, and a short reorg could then
invalidate a whole tree of downstream transactions belonging to people who had nothing
to do with the mining. The 100-block wait makes such a reorg prohibitively expensive
before the coins can move at all.

**Why 101 blocks on a fresh chain.** The reward from block 1 becomes spendable once its
depth reaches 100 confirmations. Depth counts the block itself, so block 1 has 100
confirmations only when the tip is at height 100. In practice the lab mines to 101 so
the reward is comfortably past the threshold with a clean margin, and the height itself
is a memorable marker. Mining exactly 101 blocks from empty yields one spendable 50 BTC
reward and 100 rewards still maturing.

**What the balances show.** `getbalances` splits the wallet's view into three fields,
and this lab is the clearest demonstration of why one number is not enough:

- `immature` holds coinbase rewards that exist on-chain but cannot be spent yet.
- `trusted` holds confirmed, spendable value the wallet is willing to fund a payment
  from.
- `untrusted_pending` holds incoming unconfirmed value, which matters in Lab 05.

At height 1 the 50 BTC reward is entirely in `immature` and `trusted` is zero, which is
precisely why the payment attempt fails. The wallet is not refusing out of caution, it
genuinely has no spendable coin to select. At height 101, one reward has moved into
`trusted` while the remaining 100 stay `immature`, each waiting for its own depth.

**The error is the evidence.** The point of preserving the rejection text rather than
just noting that the send failed is that the message names the actual constraint rather
than a generic failure. That is what distinguishes "the maturity rule stopped me" from
"the wallet was locked" or "the wrong wallet was addressed", which are entirely
different problems with the same surface symptom of a payment not going through.
