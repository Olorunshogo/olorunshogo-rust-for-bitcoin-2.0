use rfb_labs_week_2_session_4::{
    Item, Library, LibraryError, LoanStatus, LoanTerms, MediaKind, Member,
};

fn library_with_items() -> Library {
    let mut library = Library::new();

    for (id, title, author, kind) in [
        (1, "Dune", "Frank Herbert", MediaKind::Book { pages: 320 }),
        (
            2,
            "Children of Dune",
            "Frank Herbert",
            MediaKind::Book { pages: 180 },
        ),
        (
            3,
            "Project Hail Mary",
            "Andy Weir",
            MediaKind::Audiobook { minutes: 540 },
        ),
        (
            4,
            "The Rust Programming Language",
            "Steve Klabnik",
            MediaKind::Ebook { size_kb: 1_200 },
        ),
    ] {
        library
            .add_item(Item::new(id, title.into(), author.into(), kind))
            .unwrap();
    }

    library
        .register_member(Member::new(100, "Ada".into()))
        .unwrap();

    library
}

// These tests are ignored so the starter repository builds before students
// implement the TODOs. Remove `#[ignore]` from one test at a time while working.

#[test]
// #[ignore = "enable after completing Parts 3 and 5"]
fn checkout_updates_both_the_item_and_the_member() {
    let mut library = library_with_items();

    library.checkout(1, 100, 5).unwrap();

    assert_eq!(
        library.find_item(1).unwrap().status,
        LoanStatus::OnLoan {
            member_id: 100,
            day_borrowed: 5,
        }
    );
    assert_eq!(library.find_member(100).unwrap().borrowed_item_ids, vec![1]);
}

#[test]
// #[ignore = "enable after completing Part 5"]
fn a_member_cannot_exceed_the_borrow_limit() {
    let mut library = library_with_items();

    library.checkout(1, 100, 0).unwrap();
    library.checkout(2, 100, 0).unwrap();
    library.checkout(3, 100, 0).unwrap();

    assert_eq!(
        library.checkout(4, 100, 0),
        Err(LibraryError::BorrowLimitReached {
            member_id: 100,
            limit: 3,
        })
    );
}

#[test]
// #[ignore = "enable after completing Parts 4 and 6"]
fn returning_a_book_late_charges_a_daily_fee() {
    let mut library = library_with_items();

    // A book may be kept 21 days. Held for 30, so 9 days are overdue.
    library.checkout(1, 100, 10).unwrap();

    assert_eq!(library.return_item(1, 40), Ok(9 * 25));
    assert_eq!(library.find_item(1).unwrap().status, LoanStatus::Available);
    assert!(
        library
            .find_member(100)
            .unwrap()
            .borrowed_item_ids
            .is_empty()
    );
}

#[test]
// #[ignore = "enable after completing Part 3"]
fn searching_by_author_borrows_rather_than_clones() {
    let library = library_with_items();

    let found = library.items_by_author("Frank Herbert");

    assert_eq!(found.len(), 2);
    assert_eq!(found[0].title, "Dune");
    // `found` holds references into `library`, so these are the same item.
    assert!(std::ptr::eq(found[0], library.find_item(1).unwrap()));
}

#[test]
fn item_cannot_be_lent_twice() {
    let mut library = library_with_items();

    library.checkout(1, 100, 5).unwrap();

    assert_eq!(
        library.checkout(1, 100, 6),
        Err(LibraryError::ItemAlreadyOnLoan {
            id: 1,
            member_id: 100,
        })
    );
}

#[test]
fn on_time_return_owes_nothing() {
    let mut library = library_with_items();

    library.checkout(1, 100, 10).unwrap();

    assert_eq!(library.return_item(1, 31), Ok(0));
}

#[test]
fn ebook_returned_late_owes_nothing() {
    let mut library = library_with_items();

    library.checkout(4, 100, 0).unwrap();

    assert_eq!(library.return_item(4, 100), Ok(0));
}

#[test]
fn empty_title_is_rejected() {
    let mut library = Library::new();

    assert_eq!(
        library.add_item(Item::new(
            1,
            "   ".into(),
            "Author".into(),
            MediaKind::Book { pages: 10 }
        )),
        Err(LibraryError::EmptyTitle)
    );
}

#[test]
fn duplicate_item_id_is_rejected() {
    let mut library = library_with_items();

    assert_eq!(
        library.add_item(Item::new(
            1,
            "Another Book".into(),
            "Another Author".into(),
            MediaKind::Book { pages: 100 }
        )),
        Err(LibraryError::DuplicateItemId { id: 1 })
    );
}

#[test]
fn duplicate_member_id_is_rejected() {
    let mut library = library_with_items();

    assert_eq!(
        library.register_member(Member::new(100, "Grace".into())),
        Err(LibraryError::DuplicateMemberId { id: 100 })
    );
}

#[test]
fn unknown_item_is_rejected_on_checkout() {
    let mut library = library_with_items();

    assert_eq!(
        library.checkout(999, 100, 0),
        Err(LibraryError::ItemNotFound { id: 999 })
    );
}

#[test]
fn unknown_member_is_rejected_on_checkout() {
    let mut library = library_with_items();

    assert_eq!(
        library.checkout(1, 999, 0),
        Err(LibraryError::MemberNotFound { id: 999 })
    );
}

#[test]
fn lost_item_is_rejected_on_checkout() {
    let mut library = Library::new();

    library
        .add_item(Item {
            id: 50,
            title: "Lost Item".into(),
            author: "Unknown".into(),
            kind: MediaKind::Book { pages: 10 },
            status: LoanStatus::Lost,
        })
        .unwrap();

    library
        .register_member(Member::new(100, "Ada".into()))
        .unwrap();

    assert_eq!(
        library.checkout(50, 100, 0),
        Err(LibraryError::ItemIsLost { id: 50 })
    );
}

#[test]
fn unknown_item_is_rejected_on_return() {
    let mut library = library_with_items();

    assert_eq!(
        library.return_item(999, 10),
        Err(LibraryError::ItemNotFound { id: 999 })
    );
}

#[test]
fn available_item_cannot_be_returned() {
    let mut library = library_with_items();

    assert_eq!(
        library.return_item(1, 10),
        Err(LibraryError::ItemNotOnLoan { id: 1 })
    );
}

#[test]
fn invalid_return_day_is_rejected() {
    let mut library = library_with_items();

    library.checkout(1, 100, 10).unwrap();

    assert_eq!(
        library.return_item(1, 9),
        Err(LibraryError::InvalidReturnDay {
            day_borrowed: 10,
            day_returned: 9,
        })
    );
}

#[test]
fn author_search_finds_matching_items() {
    let library = library_with_items();

    let found = library.items_by_author("Frank Herbert");

    assert_eq!(found.len(), 2);
    assert_eq!(found[0].title, "Dune");
    assert_eq!(found[1].title, "Children of Dune");
}

#[test]
fn available_items_returns_only_available_items() {
    let mut library = library_with_items();

    library.checkout(1, 100, 0).unwrap();

    let available = library.available_items();

    assert_eq!(available.len(), 3);
    assert!(available.iter().all(|item| item.id != 1));
}

#[test]
fn longest_loan_item_uses_loan_terms() {
    let library = library_with_items();

    assert_eq!(library.longest_loan_item().unwrap().id, 1);
}

#[test]
fn loan_terms_are_correct() {
    assert_eq!(MediaKind::Book { pages: 1 }.loan_days(), 21);
    assert_eq!(MediaKind::Audiobook { minutes: 1 }.loan_days(), 14);
    assert_eq!(MediaKind::Ebook { size_kb: 1 }.loan_days(), 7);

    assert_eq!(MediaKind::Book { pages: 1 }.late_fee_cents(30), 9 * 25);
    assert_eq!(MediaKind::Ebook { size_kb: 1 }.late_fee_cents(30), 0);
}
