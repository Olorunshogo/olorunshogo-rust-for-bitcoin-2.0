# Week 2 Session 4 Assignment — A Community Lending Library

The point is enums, structs, traits,
ownership, borrowing, and `Result`, with nothing else competing for attention.
Money is in whole cents; time is a whole day number counted from an arbitrary
epoch.

## Required work

- [ ] **Part 1 — Data model:** review the provided `MediaKind`, `LoanStatus`,
  `Item`, and `Member` types. Explain why `LoanStatus` is an enum rather than a
  `bool` plus two `Option` fields, and what `match` forces you to handle.
- [ ] **Part 2 — Errors:** implement useful `Display` messages for every
  `LibraryError`, including the ids each variant carries. Expected invalid data
  must never call `panic!`.
- [ ] **Part 3 — Ownership and borrowing:** implement `add_item` and
  `register_member`, which take ownership and reject empty titles and duplicate
  ids. Implement `find_item`, `find_member`, `items_by_author`, and
  `available_items` using borrowed references without cloning.
- [ ] **Part 4 — Traits:** implement `LoanTerms` for both `MediaKind` and
  `Item`, writing the shared fee formula once in `late_fee_cents`. Implement
  `Display` for `MediaKind`, `LoanStatus`, and `Item`, and `longest_loan_item`.
- [ ] **Part 5 — Checkout:** implement `checkout`. Validate first and mutate
  second; on success the item's status and the member's borrowed list must both
  change. Use `?` where appropriate.
- [ ] **Part 6 — Return:** implement `return_item`. Compute the days held with
  checked arithmetic, charge the fee through `LoanTerms`, set the item back to
  `Available`, and drop its id from the member's list.
- [ ] **Part 7 — Experiments:** complete the two ownership experiments and
  record the compiler errors in `README.md`.
- [ ] **Part 8 — Demo:** in `main.rs`, stock a library, register a member, run
  a complete loan and a late return, and print one handled error using its
  `Display` message. `main` returns `Result`, so use `?`.
- [ ] **Part 9 (optional) — Generic search:** add `filter_items` taking a
  `Fn(&Item) -> bool` and re-express the two filtered lookups in terms of it.

## Loan terms and validation rules

Books may be kept 21 days, audiobooks 14, ebooks 7. Late items cost 25 cents a
day; ebooks are never late. `checkout` checks, in this order: unknown item,
unknown member, lost item, item already on loan, then borrow limit reached. The
order matters — a caller fixing one problem at a time deserves a predictable
next error. `return_item` rejects an unknown item, a lost item, an item that is
not on loan, and a return day earlier than the borrow day.

## The two experiments

Run each, paste the real `cargo check` error into `README.md`, explain it, then
comment the line out. **A** — read `item.title` after `library.add_item(item)?`.
**B** — hold the result of `library.find_item(1)`, call `library.checkout(..)?`,
then print what you held.

## Testing checklist

Write tests for a successful checkout, an item that cannot be lent twice, the
borrow limit, a late return's fee, an on-time return owing nothing, an ebook
returned late still owing nothing, and author search returning borrowed items.
Also test each validation error. The repository contains a few ignored starter
tests; remove their `#[ignore]` attributes and add the remaining cases.