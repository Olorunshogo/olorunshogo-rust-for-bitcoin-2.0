# Rust for Bitcoin 2.0 — Week 2, Session 4

Build a small lending library while practising structs, enums, traits,
ownership, borrowing, collections, and `Result`-based error handling. No
Bitcoin and no external crates — just Rust.

The crate is intentionally incomplete. Search for `TODO` and implement each
part; do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Part 2 in `error.rs`, then Part 3 in `library.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits in Part 4 and the two operations in Parts 5–6.
5. Run the ownership experiments and record the errors.
6. Build the demo in `main.rs`.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in
the submission.

## Written answers

Answer in your own words. Add both ownership compiler errors from Part 7 as
fenced text blocks, then explain what caused each.

1. Why is `LoanStatus` an enum rather than a `bool` plus two `Option` fields?
2. What does `match` force you to do when a fourth `MediaKind` is added later?
3. `Item::new` takes `String` rather than `&str`. Who owns the title afterwards?
4. Why does `add_item` take `self` by `&mut` but `item` by value?
5. When `add_item` returns `Err`, what happened to the `Item` the caller passed
   in? Was that a good design choice, and what is the alternative?
6. Why does `find_item` return `Option<&Item>` rather than `Option<Item>`?
7. What is the lifetime `'a` in `items_by_author` actually saying?
8. Why can't `checkout` hold a `&mut Item` and a `&mut Member` from the same
   `Library` at once, and how did you structure the method around that?
9. Why are `Library`'s fields private?
10. What duplication does the provided `late_fee_cents` remove, and what would
    you lose by making it a free function instead?
11. Why is `Result` preferable to `panic!` for validation failures? Name a
    place in this crate where a panic would be defensible.
12. Which derive did you deliberately leave off a type, and why?

## Design notes

Describe any choices you made, including how you kept an item's status and its
borrower's list from drifting apart, and (if attempted) the optional generic
search.

## Example output

Paste the output of `cargo run` here once Part 8 is complete.

## Answers

**Data Model**`

`LoanStatus` is an enum instead of a `bool` plus two `Option` fields because a `boo` would only tell us two things, for example whether an item is available or not. But in our case, there can be different loan statuses, such as an item being available, being on loan or being lost. Therre can also be different information attached to each status, such as the `member_id `and `day_borrowed` when an item is on loan.

This is where enums are useful because they allow us to represent the different patterns or states that an item can have. Instead of having separate fields that might or might not contain values, `LoanStatus` makes the possible states clear and keeps the related data together.

For example, with:

```rust
LoanStatus::OnLoan {
    member_id: u32,
    day_borrowed: u32,
}
```

This way, we know something is on loan, and we also need to track who borrowed it and when it was borrowed.

Enums also work well with `match`. `match` forces us to handle the different patterns of the enum.So when we match on `LoanStatus`, Rust makes us consider the possible cases such as `Available, `OnLoan`, and `Lost`. This helps us avoid forgetting to handle one of the possible states.

So, instead of just using a `bool` to say "yes or no", the enums allows us to track different status, exhaust all options and the information that belongs to each status.

**2. `match` and Enums**

`match` forces me to handle the new variant. If I add a fourth `MediaKind`, Rust will show an error for any `match` that does not handle that new case. This is useful because it helps make sure I do not accidental forget.


**3. `Item::new` takes `String` rather than `&str`. Who owns the title afterwards?**

The `Item` owns the title because the `String` is moved into the `Item` when `Item::new` is called. The caller gives ownership of the `String` to the `Item`, so the title will stay valid as long as the `Item` owns it.

**4. Why does `add_item` take `self` by `&mut` but `item` by value?**

`&mut self` allows the method to change the existing `Library` without taking ownership of the whole library. The `item` is passed by value because the library needs to take ownership of it and store it inside its `Vec<Item>`.

**5. When `add_item` returns `Err`, what happened to the `Item` the caller passed in? Was that a good design choice, and what is the alternative?**

The `Item` was moved into the function, so if validation fails, the function returns without storing it and the `Item` is dropped. This is simple, but it means the caller cannot get the original item back. An alternative would be to return the item together with the error, for example `Result<(), (LibraryError, Item)>`, so the caller could recover it.

**6. Why does `find_item` return `Option<&Item>` rather than `Option<Item>`?**

It returns `Option<&Item>` because the library already owns the item. I only need to borrow it to look at it, so there is no need to clone or move the item out of the library. This is more efficient and keeps ownership with the `Library`.

**7. What is the lifetime `'a` in `items_by_author` actually saying?**

The lifetime `'a` says that the references returned in the `Vec<&'a Item>` are valid for as long as the borrowed `Library` is valid. The items are owned by the library, so the returned references cannot outlive the library.

**8. Why can't `checkout` hold a `&mut Item` and a `&mut Member` from the same `Library` at once, and how did you structure the method around that?**

Rust does not allow two mutable borrows of the same `Library` at the same time because that could cause conflicting changes to the same data. I structured `checkout` by first finding the indexes of the item and member, then using those indexes to update each one separately. This allows me to update both the item's status and the member's borrowed-item list safely.

**9. Why are `Library`'s fields private?**

The fields are private so callers cannot directly change the items or members and accidentally make the library inconsistent. The `Library` should control changes to an item's `LoanStatus` and a member's `borrowed_item_ids` so they stay in agreement.

**10. What duplication does the provided `late_fee_cents` remove, and what would you lose by making it a free function instead?**

`late_fee_cents` removes the duplication of calculating the overdue days and multiplying them by the daily late fee. The shared method works for both `MediaKind` and `Item` through the `LoanTerms` trait. If it was a free function, I would lose some of the behaviour being directly connected to the type's loan terms and would have to pass the necessary values into the function separately.

**11. Why is `Result` preferable to `panic!` for validation failures? Name a place in this crate where a panic would be defensible.**

`Result` is better because validation failures are expected situations that the caller can handle. A duplicate item, missing member, or invalid return day should not crash the program. A `panic!` could be defensible for a programmer error or an impossible internal state, but not for normal library operations such as checking out an unavailable item.

**12. Which derive did you deliberately leave off a type, and why?**

I left `Copy` off `Item` because `Item` contains `String` fields. `String` owns heap-allocated data and cannot implement `Copy`. I also would not want copying an `Item` automatically because the `Library` is supposed to own each item.

### Design Notes

I used the `Library` as the owner of all the items and members. The `Member` only stores the IDs of the items they have borrowed, rather than owning the items themselves.

For `checkout`, I first validate that the item and member exist, then check the item's status and the member's borrowing limit. Only after all the checks pass do I update both the item's `LoanStatus` and the member's `borrowed_item_ids`. This helps prevent the two pieces of information from drifting apart.

For `return_item`, I first check the item's current status and calculate the late fee before changing anything. After the validation succeeds, I set the item back to `Available` and remove its ID from the member's borrowed list.

I used references in the lookup methods instead of cloning items or members. This means the `Library` keeps ownership while callers can temporarily borrow the data.

I also used the `LoanTerms` trait so that the loan period and late-fee rules are defined by the media type. `Item` delegates these calculations to its `MediaKind`, which avoids repeating the same logic.

