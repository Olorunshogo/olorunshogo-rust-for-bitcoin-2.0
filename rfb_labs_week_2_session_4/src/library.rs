use crate::catalogue::Item;
use crate::error::LibraryError;
use crate::member::Member;
use crate::{LoanStatus, LoanTerms};

pub const MAX_ITEMS_PER_MEMBER: usize = 3;

/// Owns every item and every member.
///
/// The fields are private because the library is responsible for keeping an
/// item's `LoanStatus` and a member's borrowed-id list in agreement. Callers
/// reach the data through the borrowing lookups below.
// TODO(Part 3): delete this attribute once your lookups actually read the
// fields. It is here only so the untouched starter crate compiles clean.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Library {
    items: Vec<Item>,
    members: Vec<Member>,
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), LibraryError> {
        // TODO(Part 3): move `item` into the library. Reject an empty title
        // and an id that is already stocked.
        // let _ = item;
        // todo!("add an item")
        if item.title.trim().is_empty() {
            return Err(LibraryError::EmptyTitle);
        }

        if self.items.iter().any(|existing| existing.id == item.id) {
            return Err(LibraryError::DuplicateItemId { id: item.id });
        }

        self.items.push(item);

        Ok(())
    }

    pub fn register_member(&mut self, member: Member) -> Result<(), LibraryError> {
        // TODO(Part 3): move `member` in. Reject an id already registered.
        // let _ = member;
        // todo!("register a member")
        if self.members.iter().any(|existing| existing.id == member.id) {
            return Err(LibraryError::DuplicateMemberId { id: member.id });
        }

        self.members.push(member);
        Ok(())
    }

    pub fn find_item(&self, id: u32) -> Option<&Item> {
        // TODO(Part 3): borrow from `self`; do not clone.
        // let _ = id;
        // todo!("find an item")
        self.items.iter().find(|item| item.id == id)
    }

    pub fn find_member(&self, id: u32) -> Option<&Member> {
        // TODO(Part 3)
        // let _ = id;
        // todo!("find a member")
        self.members.iter().find(|member| member.id == id)
    }

    pub fn items_by_author<'a>(&'a self, author: &str) -> Vec<&'a Item> {
        // TODO(Part 3): return references to all matching items.
        // let _ = author;
        // todo!("find items by author")
        self.items
            .iter()
            .filter(|item| item.author == author)
            .collect()
    }

    pub fn available_items(&self) -> Vec<&Item> {
        // TODO(Part 3)
        // todo!("find the available items")
        self.items
            .iter()
            .filter(|item| matches!(item.status, LoanStatus::Available))
            .collect()
    }

    pub fn longest_loan_item(&self) -> Option<&Item> {
        // TODO(Part 4): the item that may be kept longest, via `LoanTerms`.
        // todo!("find the longest-loan item")
        // return self.items.iter().max_by_key(|item| item.loan_days());
        let longest_days = self.items.iter().map(|item| item.loan_days()).max()?;

        self.items
            .iter()
            .find(|item| item.loan_days() == longest_days)
    }

    pub fn checkout(&mut self, item_id: u32, member_id: u32, day: u32) -> Result<(), LibraryError> {
        // TODO(Part 5): validate in the order given in ASSIGNMENT.md, then
        // update the item's status and the member's list together.
        // let _ = (item_id, member_id, day);
        // todo!("check an item out")
        let item_index = self
            .items
            .iter()
            .position(|item| item.id == item_id)
            .ok_or(LibraryError::ItemNotFound { id: item_id })?;

        let member_index = self
            .members
            .iter()
            .position(|member| member.id == member_id)
            .ok_or(LibraryError::MemberNotFound { id: member_id })?;

        match self.items[item_index].status {
            LoanStatus::Lost => {
                return Err(LibraryError::ItemIsLost { id: item_id });
            }
            LoanStatus::OnLoan {
                member_id: current_member_id,
                ..
            } => {
                return Err(LibraryError::ItemAlreadyOnLoan {
                    id: item_id,
                    member_id: current_member_id,
                });
            }
            LoanStatus::Available => {}
        }

        if self.members[member_index].borrowed_item_ids.len() >= MAX_ITEMS_PER_MEMBER {
            return Err(LibraryError::BorrowLimitReached {
                member_id,
                limit: MAX_ITEMS_PER_MEMBER,
            });
        }

        self.items[item_index].status = LoanStatus::OnLoan {
            member_id,
            day_borrowed: day,
        };

        self.members[member_index].borrowed_item_ids.push(item_id);

        Ok(())
    }

    /// Returns the late fee owed, in cents.
    pub fn return_item(&mut self, item_id: u32, day: u32) -> Result<u32, LibraryError> {
        // TODO(Part 6): checked subtraction must return InvalidReturnDay.
        // let _ = (item_id, day);
        // todo!("return an item")

        let item_index = self
            .items
            .iter()
            .position(|item| item.id == item_id)
            .ok_or(LibraryError::ItemNotFound { id: item_id })?;

        let (member_id, day_borrowed, fee) = match self.items[item_index].status {
            LoanStatus::Lost => {
                return Err(LibraryError::ItemIsLost { id: item_id });
            }
            LoanStatus::Available => {
                return Err(LibraryError::ItemNotOnLoan { id: item_id });
            }
            LoanStatus::OnLoan {
                member_id,
                day_borrowed,
            } => {
                let days_held =
                    day.checked_sub(day_borrowed)
                        .ok_or(LibraryError::InvalidReturnDay {
                            day_borrowed,
                            day_returned: day,
                        })?;

                let fee = self.items[item_index].late_fee_cents(days_held);
                (member_id, day_borrowed, fee)
            }
        };

        let member_index = self
            .members
            .iter()
            .position(|member| member.id == member_id)
            .ok_or(LibraryError::MemberNotFound { id: member_id })?;

        self.items[item_index].status = LoanStatus::Available;
        self.members[member_index]
            .borrowed_item_ids
            .retain(|id| *id != item_id);

        let _ = day_borrowed;

        Ok(fee)
    }

    // Returns all books/items currently in the library.
    pub fn list_all_books(&self) -> Vec<&Item> {
        self.items.iter().collect()
    }

    // Returns all members registered with the library.
    pub fn list_all_members(&self) -> Vec<&Member> {
        self.members.iter().collect()
    }
}
