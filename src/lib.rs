//! # vue-sfc
//! vue-sfc provides a parser and data structures needed to represent a Vue SFC.
//!
//! ## Parsing
//! See [`parse`].
//!
//! ## Printing
//! [`Block`], [`Raw`] and [`Section`] implement [`std::fmt::Display`].

#[doc(no_inline)]
pub use self::ast::{Attribute, AttributeName, AttributeValue, Block, BlockName, Raw, Section};
pub use self::error::Error;
#[doc(no_inline)]
pub use self::parser::parse;

pub mod ast;
mod error;
pub mod parser;
