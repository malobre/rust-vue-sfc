use std::error::Error;
use std::fmt::Display;

/// Error returned when an unexpected char is present.
#[derive(Debug)]
pub struct IllegalChar(pub(crate) char);

impl Display for IllegalChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal char: `{}`", self.0)
    }
}

impl Error for IllegalChar {}
