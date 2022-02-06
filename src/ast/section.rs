use std::fmt::Display;

use crate::{Block, Raw};

/// A Vue SFC section.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Section<'a> {
    /// See [`Raw`];
    Raw(Raw<'a>),
    /// See [`Block`].
    Block(Block<'a>),
}

impl Display for Section<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(content) => content.fmt(f),
            Self::Block(block) => block.fmt(f),
        }
    }
}
