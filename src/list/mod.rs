//! Lists are ordered sequences of values, which can be iterated over.
//!
//! Within the context of a type system lists may be all of one type, or of
//! many types. This library will eventually have implementations for all of
//! these types.
//!
//! All lists implement the standard collection traits, `FromIterator`, and
//! `IntoIterator`. This allows lists to be to be both easily converted to
//! and from. Additionally all lists implement the traits `Clone`, `Debug`,
//! `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`.
//!
//! # Examples
//!
//! ```
//! use std::iter::FromIterator;
//! use structures::list::LinkedList;
//!
//! // Easily create a linked list.
//! let list = LinkedList::from_iter((0..100));
//! assert_eq!(list.len(), 100);
//! ```

pub use list::linked_list::{LinkedList};

pub mod linked_list;
