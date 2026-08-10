//! Small executable for Part 8 of the assignment.

use rfb_labs_week_2_session_4::{Item, Library, LibraryError, MediaKind, Member};

fn main() -> Result<(), LibraryError> {
    // TODO(Part 8): stock a library, register a member, run a loan and a late
    // return, then print one handled error using its `Display` message.
    // println!("nothing to see yet — start at Part 1");
    let mut library = Library::new();

    let _ = library.add_item(Item::new(
        1,
        "How To be a core dev".to_string(),
        "Chibey Ilorah".to_string(),
        MediaKind::Book { pages: 972 },
    ));

    let _ = library.add_item(Item::new(
        2,
        "Follow For More".to_string(),
        "Yahya Tijani".to_string(),
        MediaKind::Audiobook { minutes: 540 },
    ));

    let _ = library.add_item(Item::new(
        3,
        "How To be a core dev".to_string(),
        "Chibey Ilorah".to_string(),
        MediaKind::Book { pages: 972 },
    ));

    let _ = library.add_item(Item::new(
        4,
        "The Rust Programming Language".into(),
        "Steve Klabnik".into(),
        MediaKind::Ebook { size_kb: 1200 },
    ));

    // === Community Library
    println!("=== Library");
    println!(" ");

    println!("== Books / Media");
    // for (number, item) in library.list_all_books().iter().enumerate() {
    for item in library.list_all_books().iter().enumerate() {
        // println!("#  {}", number + 1, item);
        println!("{:#?}", item);
    }

    println!(" ");

    // Register members
    let _ = library.register_member(Member::new(100, "Jigs".to_string()));
    library.register_member(Member::new(101, "Mark".into()))?;

    println!("== Members");
    println!(" ");
    for (number, member) in library.list_all_members().iter().enumerate() {
        println!("#{}  {} {:#?}", number + 1, member.name, member.id);
        if member.borrowed_item_ids.is_empty() {
            println!(" Borrowed: none");
        } else {
            println!(" Borrowed item IDs: {:?}", member.borrowed_item_ids);
        }
    }

    // == Checkout
    println!(" ");
    println!("== Checkout ");
    println!(" ");

    library.checkout(1, 100, 5)?;

    println!(
        "Checked out: {}",
        library
            .find_item(1)
            .ok_or(LibraryError::ItemNotFound { id: 1 })?
    );

    // == Update Books
    println!(" ");
    println!("==  Updated Books");
    println!(" ");
    for item in library.list_all_books().iter().enumerate() {
        // println!("#  {}", number + 1, item);
        println!("{:#?}", item);
    }

    // == Update Books
    println!(" ");
    println!("==  Updated Members");
    println!(" ");
    for (number, member) in library.list_all_members().iter().enumerate() {
        println!("#{}  {} {:#?}", number + 1, member.name, member.id);
        if member.borrowed_item_ids.is_empty() {
            println!(" Borrowed: none");
        } else {
            println!(" Borrowed item IDs: {:?}", member.borrowed_item_ids);
        }
    }

    let fee = library.return_item(1, 40)?;

    println!("Returned Dune on day 40; late fee");
    println!("Late fee: {fee} cents");

    // == Handled Error
    println!("== Handled Error");
    println!(" ");

    match library.checkout(999, 100, 41) {
        Ok(()) => {}
        Err(error) => println!("Handled error: {error}"),
    }

    Ok(())
}
