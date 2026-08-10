//! Starter library for the Week 2 Session 4 lending-library assignment.
//!
//! Work through the `TODO` markers in part order. Keep the public names and
//! function signatures unchanged so the test suite can exercise your code.

pub mod catalogue;
pub mod error;
pub mod library;
pub mod member;

pub use catalogue::{Item, LoanStatus, LoanTerms, MediaKind};
pub use error::LibraryError;
pub use library::{Library, MAX_ITEMS_PER_MEMBER};
pub use member::Member;
