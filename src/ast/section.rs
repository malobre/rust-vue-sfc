use std::{borrow::Cow, fmt::Display};

use crate::Block;

/// A Vue SFC section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Section<'a> {
    /// Represent any data before, after or between blocks.
    Raw(Cow<'a, str>),
    /// See [`Block`].
    Block(Block<'a>),
}

impl Display for Section<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(content) => content.trim_end().fmt(f),
            Self::Block(block) => block.fmt(f),
        }
    }
}
