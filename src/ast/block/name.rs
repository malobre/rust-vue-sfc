use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

pub use self::error::InvalidBlockName;
use self::error::InvalidBlockNameKind;

mod error {
    use std::error::Error;
    use std::fmt::Display;

    /// Returned when a function was unable to convert a string to a [`BlockName`][super::BlockName].
    #[derive(Debug)]
    pub struct InvalidBlockName(pub(super) InvalidBlockNameKind);

    impl Display for InvalidBlockName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self(InvalidBlockNameKind::StartsWithNonAsciiAlpha) => {
                    write!(f, "block name must start with ASCII alpha")
                }
                Self(InvalidBlockNameKind::IllegalChar(ch)) => {
                    write!(f, "block name cannot contain `{ch}`")
                }
            }
        }
    }

    impl Error for InvalidBlockName {}

    #[derive(Debug)]
    pub(super) enum InvalidBlockNameKind {
        IllegalChar(char),
        StartsWithNonAsciiAlpha,
    }
}

/// The name of a block, i.e: `script` in `<script lang="ts">`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
pub struct BlockName<'a>(Cow<'a, str>);

impl<'a> BlockName<'a> {
    /// Attempts to convert a string to a [`BlockName`].
    ///
    /// # Errors
    /// Will return an error if the string:
    /// - doesn't start with an ASCII alpha,
    /// - contains any of the following characters:
    ///   - `U+0009 CHARACTER TABULATION`,
    ///   - `U+000A LINE FEED`,
    ///   - `U+000C FORM FEED`,
    ///   - `U+0020 SPACE`,
    ///   - `U+002F SOLIDUS (/)`,
    ///   - `U+003E GREATER-THAN SIGN (>)`.
    pub fn from_cow(mut src: Cow<'a, str>) -> Result<Self, InvalidBlockName> {
        if !src.starts_with(|ch: char| ch.is_ascii_alphabetic()) {
            return Err(InvalidBlockName(
                InvalidBlockNameKind::StartsWithNonAsciiAlpha,
            ));
        }

        if let Some(ch) = src.chars().find(|ch| {
            matches!(
                ch,
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{002F}' | '\u{003E}'
            )
        }) {
            return Err(InvalidBlockName(InvalidBlockNameKind::IllegalChar(ch)));
        }

        if src.contains(|ch: char| ch.is_ascii_uppercase()) {
            src.to_mut().make_ascii_lowercase();
        }

        Ok(Self(src))
    }

    /// Convert a string into a [`BlockName`] **without** validating
    /// (unless `debug_assertions` is enabled).
    ///
    /// # Panics
    /// If `debug_assertions` is enabled, validate the input and panic on failure.
    ///
    /// # Safety
    /// See string prerequisites of [`BlockName::from_cow`].
    pub unsafe fn from_cow_unchecked(src: Cow<'a, str>) -> Self {
        if cfg!(debug_assertions) {
            match Self::from_cow(src) {
                Ok(val) => val,
                Err(err) => {
                    panic!("BlockName::from_cow_unchecked(): {err}")
                }
            }
        } else {
            Self(src)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for BlockName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Deref for BlockName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for BlockName<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TryFrom<&'a str> for BlockName<'a> {
    type Error = InvalidBlockName;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for BlockName<'a> {
    type Error = InvalidBlockName;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Owned(value))
    }
}

impl<'a> TryFrom<Cow<'a, str>> for BlockName<'a> {
    type Error = InvalidBlockName;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::from_cow(value)
    }
}
