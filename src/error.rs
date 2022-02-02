use std::fmt::Display;

use crate::ast::{InvalidAttributeName, InvalidAttributeValue, InvalidBlockName};
use crate::parser::ParseError;

#[derive(Debug)]
enum ErrorKind {
    Parse(ParseError),
    InvalidBlockName(InvalidBlockName),
    InvalidAttributeName(InvalidAttributeName),
    InvalidAttributeValue(InvalidAttributeValue),
}

/// A generic error.
///
/// This error is less specific than others in the crate, but they can all be converted to this
/// error.
#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    fn get_ref(&self) -> &(dyn std::error::Error + 'static) {
        let Self(inner) = self;

        match inner {
            ErrorKind::Parse(err) => err,
            ErrorKind::InvalidBlockName(err) => err,
            ErrorKind::InvalidAttributeName(err) => err,
            ErrorKind::InvalidAttributeValue(err) => err,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get_ref(), f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.get_ref().source()
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self(ErrorKind::Parse(err))
    }
}

impl From<InvalidBlockName> for Error {
    fn from(err: InvalidBlockName) -> Self {
        Self(ErrorKind::InvalidBlockName(err))
    }
}

impl From<InvalidAttributeName> for Error {
    fn from(err: InvalidAttributeName) -> Self {
        Self(ErrorKind::InvalidAttributeName(err))
    }
}

impl From<InvalidAttributeValue> for Error {
    fn from(err: InvalidAttributeValue) -> Self {
        Self(ErrorKind::InvalidAttributeValue(err))
    }
}
