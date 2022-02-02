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

pub use self::ast::{
    Attribute, AttributeName, AttributeValue, Block, BlockName, IllegalCharError, Section,
};
pub use self::parser::{parse, ParseError};

mod ast;
mod parser;
