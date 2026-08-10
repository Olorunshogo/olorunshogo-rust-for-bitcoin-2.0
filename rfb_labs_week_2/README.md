# Rust for Bitcoin 2.0 — Week 2

A simplified Bitcoin transaction model built with structs, enums, traits,
ownership, borrowing, collections, and `Result`-based error handling. No external
Bitcoin crate is used; the dependency list in `Cargo.toml` is empty on purpose.

All monetary values are integer satoshis (`1 BTC = 100,000,000 sats`).
The formular for determining the amount of `fee` paid to the miner if calculated by the following:

```rust
fee = satoshi(sats) / byte
```

## Layout

| File | Contents |
| --- | --- |
| `src/transaction.rs` | Parts 1 to 3 and 5 to 7: data model, methods, validation, traits, borrowing helpers |
| `src/error.rs` | Part 4: `TransactionError` and its `Display` messages |
| `src/utxo.rs` | Part 9: `Utxo` and `select_utxos` |
| `src/state.rs` | Part 10 (optional): `TxState` lifecycle guard |
| `src/main.rs` | Part 8: the 70,000 + 50,000 sat payment example |
| `tests/transaction.rs` | 19 tests covering totals, fee, traits, borrowing, and every validation error |
| `tests/utxo.rs` | 5 selection tests including insufficient funds |

The Parts 1 and 2 data model walkthrough lives in
[Assignment_Answers.md](Assignment_Answers.md).

## Commands

```bash
cargo test
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

24 tests pass, no test is `#[ignore]`d, and both `fmt --check` and
`clippy -D warnings` are clean. `cargo test -- --ignored` runs zero tests because
none remain ignored.

## Written answers

### 1. What is a Bitcoin transaction input?

An input is a reference to a previous output that this transaction is spending,
plus the data that proves the spender is allowed to spend it. It points at the
earlier output through an outpoint rather than carrying a copy of the coins. In
this model that is `InputKind::Regular`. A coinbase input is the exception: it
spends nothing and instead mints the block subsidy plus collected fees.

### 2. What is a Bitcoin transaction output?

An output is an amount of satoshis locked to a spending condition. Here it is
modelled as a value, a recipient address, and an `OutputType` standing in for the
locking script. Once an output exists it can be spent exactly once, and only by
whoever satisfies its condition.

### 3. What is a UTXO?

An unspent transaction output: an output that has been created by a confirmed
transaction and that no later transaction has consumed yet. The set of all UTXOs
is the actual ledger state. A wallet balance is just the sum of the UTXOs it can
spend, which is why `select_utxos` operates over a slice of them.

### 4. What does an outpoint identify?

Exactly one output anywhere in the chain, by the pair `(txid, vout)`: the id of
the transaction that created it, and its zero-based index in that transaction's
output list. The `Display` implementation prints it in the conventional
`txid:vout` form.

### 5. How is a transaction fee calculated?

`fee = total input value - total output value`. It is never stated explicitly in
a transaction; it is whatever the inputs cover that the outputs do not claim,
and miners collect it. That implicitness is why `fee()` must guard the
subtraction rather than trust the caller.

### 6. Why use integers rather than floating-point numbers for bitcoin amounts?

Binary floating point cannot represent most decimal fractions exactly, so `f64`
arithmetic drifts. Repeated additions of amounts like `0.1 BTC` accumulate error,
and a comparison that should be exact silently fails. Satoshis are whole units by
definition, so `u64` represents every valid amount exactly, addition is exact, and
overflow or underflow can be caught with `checked_add` / `checked_sub`. `f64`
appears only in `value_in_btc()`, which is for display and nothing else.

### 7. Why does `total_input_value()` borrow `self`?

Because it only reads. Taking `&self` lets the caller keep using the transaction
afterwards, allows several immutable borrows at once, and documents in the
signature that the method has no side effects. Taking `self` would consume the
transaction just to compute a number.

### 8. Why does `add_input()` take `&mut self`?

It mutates `self.inputs`, and Rust requires an exclusive borrow to do that. The
`&mut` also guarantees no other reference into the transaction is alive during
the push, which matters because a `Vec` can reallocate and invalidate any
outstanding reference to its elements.

### 9. What happens when an input is moved into a transaction?

`add_input` takes `input: InputKind` by value, so ownership transfers to the
`Vec` inside the transaction. The caller's binding is left uninitialised and any
later use of it is a compile error. Nothing is copied and nothing is
reference-counted; the transaction becomes the single owner, and the input's
heap data (the `String` txid) is freed when the transaction is dropped.

### 10. Why is `Result` preferable to `panic!` for validation failures?

Invalid transaction data is expected input, not a bug. `Result` makes the failure
part of the signature so the caller cannot forget it, carries the offending
amounts in the error so the message is useful, and lets the caller decide whether
to retry, report, or abort. `panic!` would take the whole process down over data
that a peer can supply at will, which in a real node is a denial-of-service vector.

### 11. How do enums help model regular and coinbase inputs?

The two kinds carry genuinely different data: a regular input has an outpoint, a
value, and a sequence number; a coinbase input has a block height and a reward.
An enum lets each variant hold only its own fields, so no impossible state exists,
whereas a single struct would need nullable fields that could be filled in wrongly.
Matching then forces both cases to be handled, as in `BitcoinValue for InputKind`
and in `validate()`. If a third variant were added, the compiler would list every
`match` that must be updated instead of letting a case fall through silently.

### 12. How does the `BitcoinValue` trait reduce duplication?

`value()` gives outputs and both input variants one shared name for "how many
satoshis is this worth", so `total_input_value` and `total_output_value` are the
same one-line fold over different types. The provided `value_in_btc()` method is
written once on the trait and every implementer gets it free, so the conversion
constant appears in exactly one place. Anything implementing `BitcoinValue` can be
summed or displayed by generic code without knowing its concrete type.

## Ownership observations (Part 7)

The experiment: build a `Regular` input, move it into a transaction with
`add_input`, then try to print it.

```rust
let mut transaction = Transaction::new(2, 0);

let input = InputKind::Regular {
    previous_output: OutPoint { txid: "aaaa".into(), vout: 0 },
    value: 70_000,
    sequence: u32::MAX,
};

transaction.add_input(input);

println!("{input}");
```

```text
error[E0382]: borrow of moved value: `input`
  --> examples/ownership_experiment.rs:17:16
   |
 6 |     let input = InputKind::Regular {
   |         ----- move occurs because `input` has type `InputKind`, which does not implement the `Copy` trait
...
15 |     transaction.add_input(input);
   |                           ----- value moved here
16 |
17 |     println!("{input}");
   |                ^^^^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
```

What caused it: `add_input` takes its argument by value, and `InputKind` is not
`Copy` because its `Regular` variant owns a heap-allocated `String` txid. Copying
it implicitly would mean either a silent allocation or two owners of the same
buffer, so Rust moves instead. After the call the `input` binding no longer owns
anything, and `println!` needs to borrow it, which is rejected at compile time.

The fixes, depending on intent: read the value before the move, borrow it back out
of `transaction.inputs` afterwards, or derive `Clone` and pass a clone if two
independent copies really are wanted. This project takes the second route, which
is why `highest_value_output` and `find_outputs_for_recipient` hand back `&TxOutput`
references tied to the transaction's lifetime and never clone. The returned `Vec`
owns only the references, so filtering costs one pointer per match rather than a
duplicate of every `String`.

The same rule shapes `select_utxos`: it takes `&[Utxo]` and returns `Vec<&Utxo>`,
so the wallet keeps ownership of its UTXOs and the caller gets a view. The test
`selection_stops_as_soon_as_the_target_is_covered` asserts the source vector is
still intact afterwards.

## Design notes

**Validation order.** `validate()` checks structure before arithmetic: empty
inputs, empty outputs, then per-input coinbase counting and txid checks, then
zero-value outputs, and finally `self.fee()?` for the balance. Reusing `fee()`
through `?` means the input-versus-output comparison exists in exactly one place,
and the `OutputsExceedInputs` error carries both totals for free.

**Zero-value outputs.** Rejected unless the type is `OpReturn`, because an
`OP_RETURN` output is a provably unspendable data carrier and a zero value there
is correct. Every other type would be creating an unspendable dust output.

**Empty txid.** Checked with `trim().is_empty()` so a whitespace-only string is
caught too. This model does not validate hex length or characters, which a real
implementation would.

**`Display` never panics.** The transaction summary matches on `self.fee()` and
prints `fee: invalid (outputs exceed inputs: ...)` on the error branch instead of
unwrapping. `display_reports_an_invalid_fee_without_panicking` covers this.

**UTXO selection trade-offs.** The implemented algorithm is first-fit in slice
order: accumulate until the target is covered, then stop. It is O(n), it is
trivial to reason about, and it is deterministic, which makes it testable. The
costs are real though:

- It ignores UTXO size, so it will happily spend a 1 BTC output to send 10,000
  sats and produce a huge change output.
- More inputs means a larger transaction and therefore a higher absolute fee,
  since fees are charged per virtual byte.
- It can create dust change that costs more to spend later than it is worth.
- It leaks information: consistently spending in wallet order is a fingerprint.

A better approach, and the one Bitcoin Core actually uses, is **branch and bound**
with a fallback to knapsack. Branch and bound searches for a subset whose total
matches the target plus fee within a small window, so the transaction produces
**no change output at all**. That is the strongest option available because it
removes a whole output from the transaction (smaller, cheaper), avoids creating
new dust, and denies chain analysts the change-detection heuristic they rely on.
When no changeless match exists within the search budget, Core falls back to a
randomised knapsack over multiple passes and picks the cheapest result. A cheaper
middle ground worth mentioning is **largest-first**, which minimises input count
and thus fee, at the cost of consolidating the wallet into fewer, more traceable
outputs. I kept first-fit here because the assignment specifies input order, and
because the borrowing behaviour is what the exercise is actually testing.

**Optional Part 10.** `TxState` models the six states and `transition` returns
`Result<TxState, InvalidTransition>`. The legal moves are encoded in a single
`matches!` over `(from, to)` pairs: forward along
`Created -> Validated -> Signed -> Broadcast -> Confirmed`, with `Rejected`
reachable from any non-terminal state. `Confirmed` and `Rejected` are terminal, so
nothing leaves them, and `is_terminal()` exposes that. The state is `Copy` and
`transition` takes `self` by value and returns the next state, so an invalid move
cannot mutate anything in place. A type-state design with one struct per state
would push this to compile time, but it would change the public API the assignment
fixes, so the runtime guard is the right fit here.

## Example output

Output of `cargo run` after Part 8. The wallet holds 70,000 and 50,000 sats,
90,000 goes to `bc1qreceiver`, and the change of
`120,000 - 90,000 - 2,000 = 28,000` returns to `bc1qsender`, leaving the required
2,000 sat fee.

```text
Selected 2 UTXO(s):
  a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90:0 -> 70000 sats
  f0e1d2c3b4a5968778695a4b3c2d1e0ff0e1d2c3b4a5968778695a4b3c2d1e0f:1 -> 50000 sats

Inputs:
  regular input a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90:0 worth 70000 sats (sequence 4294967295)
  regular input f0e1d2c3b4a5968778695a4b3c2d1e0ff0e1d2c3b4a5968778695a4b3c2d1e0f:1 worth 50000 sats (sequence 4294967295)
Outputs:
  90000 sats to bc1qreceiver (P2wpkh)
  28000 sats to bc1qsender (P2wpkh)

Transaction
  version:      2
  locktime:     0
  inputs:       2
  outputs:      2
  total input:  120000 sats
  total output: 118000 sats
  fee:          2000 sats

Validation: ok
Largest output: 90000 sats to bc1qreceiver (P2wpkh)
Change back to bc1qsender: 1 output(s), 28000 sats (0.00028000 BTC)
Lifecycle state: broadcast
Rejected transition: cannot move a transaction from created to broadcast
```
