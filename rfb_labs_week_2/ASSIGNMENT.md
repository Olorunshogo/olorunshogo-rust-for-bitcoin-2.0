# Week 2 Assignment — Modelling a Bitcoin Transaction

All monetary values are integer satoshis (`1 BTC = 100,000,000 sats`). This is a
simplified model: it does not serialize, sign, or broadcast a real transaction.

## Required work

- [x] **Parts 1–2 — Data model:** review the provided `TxOutput`, `OutputType`,
  `OutPoint`, `InputKind`, and `Transaction` types. Explain why `InputKind` is an
  enum and how matching forces both regular and coinbase inputs to be handled.
- [x] **Part 3 — Methods:** implement `add_input`, `add_output`, input/output totals,
  and `fee`. Adding values must transfer ownership. `fee` returns
  `OutputsExceedInputs` instead of underflowing.
- [x] **Part 4 — Errors:** implement useful `Display` messages for every
  `TransactionError`. Expected invalid data must never call `panic!`.
- [x] **Part 5 — Validation:** reject no inputs, no outputs, non-`OpReturn` zero
  outputs, outputs exceeding inputs, mixed coinbase/regular inputs, multiple
  coinbase inputs, and empty regular-input TXIDs. Use `?` where appropriate.
- [x] **Part 6 — Traits:** implement `BitcoinValue` for outputs and both input
  variants. Implement `Display` for `OutPoint`, `TxOutput`, `InputKind`, and
  `Transaction`.
- [x] **Part 7 — Borrowing:** implement `highest_value_output` and
  `find_outputs_for_recipient` using borrowed references without cloning. Complete
  the ownership experiment and record the compiler error in `README.md`.
- [x] **Part 8 — Payment:** in `main.rs`, spend UTXOs of 70,000 and 50,000 sats,
  pay 90,000 sats to `bc1qreceiver`, return change to `bc1qsender`, and leave a
  calculated 2,000-sat fee. Use version 2 and locktime 0.
- [x] **Part 9 — Selection:** implement `select_utxos` over a borrowed slice. The
  basic algorithm selects in input order and returns borrowed UTXOs. Return
  `InsufficientFunds` when necessary. Bonus: justify a better selection algorithm.
- [x] **Part 10 (optional) — State:** model Created, Validated, Signed, Broadcast,
  Confirmed, and Rejected states and prevent invalid transitions.

## Required transaction summary

`Display` for `Transaction` must show its version, locktime, input/output counts,
total input, total output, and calculated fee. An invalid fee should be displayed
clearly rather than causing a panic.

## Testing checklist

Write tests for a valid regular transaction, totals, fee, highest output, recipient
filtering, valid coinbase, and successful UTXO selection. Also test each validation
error and insufficient funds. The repository contains a few ignored starter tests;
remove their `#[ignore]` attributes and add the remaining cases.

## Submission standard

- All required TODOs are implemented; no required tests remain ignored.
- At least eight meaningful tests pass.
- `cargo fmt --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `README.md` contains all written answers, ownership observations, design notes,
  and example output.
- Do not add an external Bitcoin library; the goal is to practise core Rust.
