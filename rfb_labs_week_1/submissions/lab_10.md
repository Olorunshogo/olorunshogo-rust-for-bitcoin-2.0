# Lab 10 — Competing branches and reorganization

## Commands used

TODO: Record peer, mining, chain-tip, and reconnection commands for both nodes.

Two Bitcoin Core nodes in the same Polar network, `backend1` as Node A and `backend2` as
Node B. Every RPC below is run inside the terminal of the node named.

```bash
# 1. confirm both nodes agree before the split
# Node A
bitcoin-cli getblockchaininfo
# Node B
bitcoin-cli getblockchaininfo
bitcoin-cli getpeerinfo          # record the peer address needed to disconnect

# 2. split the network, from Node A
bitcoin-cli disconnectnode <node_b_address>
bitcoin-cli getpeerinfo          # expect no peers, so the split is real

# 3. mine privately on each side
# Node A, the shorter branch
bitcoin-cli generatetoaddress 2 <node_a_address>
bitcoin-cli getblockchaininfo    # record tip and chainwork
# Node B, the longer branch
bitcoin-cli generatetoaddress 4 <node_b_address>
bitcoin-cli getblockchaininfo    # record tip and chainwork

# 4. reconnect, from Node A
bitcoin-cli addnode <node_b_address> onetry
bitcoin-cli getpeerinfo

# 5. after synchronization, confirm both tips match
# Node A
bitcoin-cli getblockchaininfo
# Node B
bitcoin-cli getblockchaininfo
```

Rust entry points, from `src/labs/lab10_reorg.rs`:

| Function | What it does |
|---|---|
| `get_chain_tip` | `getblockchaininfo`, reads `blocks`, `bestblockhash`, `chainwork` |
| `disconnect_peer` | `disconnectnode <address>` |
| `reconnect_peer` | `addnode <address> onetry` |
| `build_reorg_report` | compares the two competing tips against the final tips and sets `converged` when both height and best-block hash match |

Two clients are constructed, one per node, since each has its own chain state:

```rust
let node_a = ProcessRpc::new("bitcoin-cli").with_base_args(["-regtest"]);
```

`build_reorg_report` requires both the height and the best-block hash to match before
setting `converged`. Equal heights alone would not prove convergence, since two different
branches can sit at the same height, which is exactly the situation the split creates.

```bash
cargo test --test lab_10
```

## Terminal output

TODO: Show the common tip, competing tips, chainwork, and final convergence.

Comparison to be filled in:

```text
before split (both nodes)
  height:     <h>
  best hash:  <hash>

after private mining
  Node A  height <h+2>  tip <hash_a>  chainwork <work_a>
  Node B  height <h+4>  tip <hash_b>  chainwork <work_b>
  chainwork_b > chainwork_a

after reconnection
  Node A  height <h+4>  tip <hash_b>
  Node B  height <h+4>  tip <hash_b>
  converged: true
  Node A's two private blocks are now stale
```

## Evidence references

TODO: Link screenshots or describe the attached evidence.

Screenshots are stored under `submissions/Evidence/Lab_10/`.

| Screenshot | Shows |
|---|---|
| [Lab_10_01_common_tip.png](Evidence/Lab_10/Lab_10_01_common_tip.png) | Both nodes at the same height and best-block hash before the split |
| [Lab_10_02_disconnect.png](Evidence/Lab_10/Lab_10_02_disconnect.png) | `disconnectnode` and `getpeerinfo` confirming the nodes are isolated |
| [Lab_10_03_node_a_two_blocks.png](Evidence/Lab_10/Lab_10_03_node_a_two_blocks.png) | Node A's private tip and chainwork after mining 2 blocks |
| [Lab_10_04_node_b_four_blocks.png](Evidence/Lab_10/Lab_10_04_node_b_four_blocks.png) | Node B's private tip and chainwork after mining 4 blocks |
| [Lab_10_05_reconnect.png](Evidence/Lab_10/Lab_10_05_reconnect.png) | `addnode ... onetry` and the peer connection restored |
| [Lab_10_06_converged.png](Evidence/Lab_10/Lab_10_06_converged.png) | Both nodes reporting the same height and best-block hash after synchronization |

## Explanation

TODO: Explain the stale branch, reorganization, and most-work-chain rule.

**What the split actually created.** While disconnected, neither node was doing anything
wrong. Each extended the chain it knew about with valid blocks it had mined itself.
Because they could not exchange blocks, they built two different valid histories from the
same common ancestor. Both nodes were correct about their own view and simply had
incomplete information. This is the normal state of affairs in a distributed system with
propagation delay, and Bitcoin does not try to prevent it. It resolves it after the fact.

**Why Node A's branch became stale.** On reconnection, each node learned about the other's
blocks and evaluated both branches under the same rule. Node B's branch carried more
accumulated work, so both nodes adopted it. Node A's two blocks did not become invalid.
They are still well-formed blocks with valid proof of work and valid transactions. They
simply are not part of the chain any more, because they descend from a branch the network
did not select. That is what "stale" means, and it is why the term is preferred over
"orphan": nothing is malformed, the blocks just lost.

Note that Node A reorganized itself, discarding blocks it had mined. No authority told it
to. It applied the same rule every node applies and reached the same conclusion.

**What a reorganization is.** A reorg is a node switching its active chain from one branch
to another. Concretely, it disconnects blocks from its current tip back to the fork point,
undoing their effects on the UTXO set, then connects the blocks of the new branch in
order, re-validating each. Transactions that were in the discarded blocks and are not in
the new ones return to the mempool, since they remain valid and are simply unconfirmed
again. This is exactly the mechanism that makes confirmation depth meaningful in Lab 08
and why coinbase maturity exists in Lab 03: coinbase outputs from stale blocks are
destroyed outright, unlike ordinary transactions, which is what the 100-block delay
protects against.

**Why most accumulated work, and not the alternatives.**

- *Not miner identity.* There is no registry of miners and no notion of a trusted or
  authoritative one. Anyone can mine, and identity is neither recorded in a block nor
  verifiable. A rule based on identity would need a list of who counts, and maintaining
  that list would require the very consensus the rule is trying to establish.
- *Not arrival time.* Nodes see blocks in different orders depending on topology and
  latency, so "first seen" is a local accident, not a global fact. If it decided the
  chain, different nodes would settle on different histories permanently, and the network
  would never converge. Nodes do use first-seen as a temporary tiebreaker between branches
  of identical work, but it is never allowed to override work.
- *Not social claim.* No announcement, reputation, or assertion of correctness is
  verifiable by a program. A node checking a claim would have to trust the claimant, which
  reintroduces exactly the trusted third party the system is designed to remove.
- *Work, because it is costly to produce and cheap to verify.* Chainwork is the only
  signal that is objective, locally computable from data every node already has, and
  expensive to fake. Every node given the same set of blocks computes the same chainwork
  totals and reaches the same answer without communicating. Convergence is the result of
  each node independently applying an identical rule, not of any coordination between
  them.

**Why chainwork rather than height.** Height counts blocks, chainwork sums the expected
hashes behind them. In this lab, where difficulty is uniform, four blocks beat two on both
measures. But when difficulty differs between branches, height and work can disagree, and
a branch of many easy blocks could look longer while representing far less effort. The
rule is stated in terms of work precisely so that it cannot be gamed that way. This is
also why my `get_chain_tip` records `chainwork` and not just `blocks`: work is the field
that carries the actual argument.

**What the rule does not do.** Most-work only ever chooses among branches that are already
fully valid. A branch containing an invalid transaction is rejected outright regardless of
how much work sits on top of it, and it never enters the comparison at all. An attacker
with unlimited hash power can reorder history, censor transactions, or double-spend their
own coins, but cannot make an invalid transaction valid, cannot create coins beyond the
schedule, and cannot spend outputs whose keys they do not hold. Work decides ordering.
Validity is checked independently by every node, and is not subject to any vote.
