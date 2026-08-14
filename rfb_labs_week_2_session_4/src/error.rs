use std::fmt;

/// Every expected failure in the lending library.
///
/// This is the only file whose types are written for you. Nothing here should
/// ever be produced by a `panic!`, an `unwrap`, or an `expect` — these are
/// ordinary outcomes a caller is expected to handle.
#[derive(Debug, PartialEq, Eq)]
pub enum LibraryError {
    EmptyTitle,
    DuplicateItemId {
        id: u32,
    },
    DuplicateMemberId {
        id: u32,
    },
    ItemNotFound {
        id: u32,
    },
    MemberNotFound {
        id: u32,
    },
    ItemAlreadyOnLoan {
        id: u32,
        member_id: u32,
    },
    ItemNotOnLoan {
        id: u32,
    },
    ItemIsLost {
        id: u32,
    },
    BorrowLimitReached {
        member_id: u32,
        limit: usize,
    },
    InvalidReturnDay {
        day_borrowed: u32,
        day_returned: u32,
    },
}

impl fmt::Display for LibraryError {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(Part 2): return a useful, human-readable message for every
        // variant. Include the ids and numbers the variant carries.
        // todo!("implement Display for LibraryError")
        match self {
            Self::EmptyTitle => write!(_formatter, "item title cannot be empty"),
            Self::DuplicateItemId { id } => {
                write!(_formatter, "an item with id {id} is already stocked")
            }
            Self::DuplicateMemberId { id } => {
                write!(_formatter, "a member with id {id} is already registered")
            }
            Self::ItemNotFound { id } => write!(_formatter, "item {id} was not found"),
            Self::MemberNotFound { id } => write!(_formatter, "member {id} was not found"),
            Self::ItemAlreadyOnLoan { id, member_id } => {
                write!(
                    _formatter,
                    "item {id} is already on loan to member {member_id}"
                )
            }
            Self::ItemNotOnLoan { id } => write!(_formatter, "item {id} is not on loan"),
            Self::ItemIsLost { id } => write!(_formatter, "item {id} is lost"),
            Self::BorrowLimitReached { member_id, limit } => {
                write!(
                    _formatter,
                    "member {member_id} has reached the borrow limit of {limit}"
                )
            }
            Self::InvalidReturnDay {
                day_borrowed,
                day_returned,
            } => write!(
                _formatter,
                "return day {day_returned} is earlier than borrow day {day_borrowed}"
            ),
        }
    }
}

impl std::error::Error for LibraryError {}
