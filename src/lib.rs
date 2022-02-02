//! # vue-sfc
//! vue-sfc provides a parser and data structures needed to represent a Vue SFC.
//!
//! ## Design
//! A Vue SFC is represented as a [`Vec<Section>`], a [`Section`] can either be:
//! - a [`Block`], e.g:
//!   ```vue
//!   <template>
//!     <!-- content -->
//!   </template>
//!   ```
//!   ```vue
//!   <script lang="ts" setup>
//!     /* --snip-- */
//!   </script>
//!   ```
//! - or something else, stored as a [`Section::Raw`].
//!
//! ## Parsing
//! See [`parse`].
//!
//! ## Printing
//! [`Block`] and [`Section`] implement [`std::fmt::Display`].
//! Note that, when printing, [`Section::Raw`] are end-trimmed.

#[doc(no_inline)]
pub use self::ast::{Attribute, AttributeName, AttributeValue, Block, BlockName, Section};
pub use self::error::Error;
#[doc(no_inline)]
pub use self::parser::parse;

pub mod ast;
mod error;
pub mod parser;
