use std::error::Error;
use std::fmt::Display;

/// A parsing error.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    MissingEndTag(#[doc(hidden)] String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEndTag(name) => write!(f, "missing end tag: `{}`", name),
        }
    }
}

impl Error for ParseError {}
