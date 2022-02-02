use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

use crate::ast::error::IllegalChar;

/// The name of an attribute, i.e: `lang` in `<script lang="ts">`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AttributeName<'a>(Cow<'a, str>);

impl<'a> AttributeName<'a> {
    /// Attempts to convert a string to an [`AttributeName`].
    ///
    /// # Errors
    /// Will return an error if the string contains any of the following characters:
    /// - `U+0009 CHARACTER TABULATION`
    /// - `U+000A LINE FEED`
    /// - `U+000C FORM FEED`
    /// - `U+0020 SPACE`
    /// - `U+002F SOLIDUS (/)`
    /// - `U+003D EQUAL SIGN (=)`
    /// - `U+003E GREATER-THAN SIGN (>)`.
    pub fn from_str(src: &'a str) -> Result<Self, IllegalChar> {
        if let Some(ch) = src.chars().find(|ch| {
            matches!(
                ch,
                '\u{0009}'
                    | '\u{000A}'
                    | '\u{000C}'
                    | '\u{0020}'
                    | '\u{002F}'
                    | '\u{003D}'
                    | '\u{003E}'
            )
        }) {
            return Err(IllegalChar(ch));
        }

        if src.contains(|ch: char| ch.is_ascii_uppercase()) {
            Ok(Self(Cow::Owned(src.to_ascii_uppercase())))
        } else {
            Ok(Self(Cow::Borrowed(src)))
        }
    }

    /// Convert a string into an [`AttributeName`] **without** validating.
    pub unsafe fn from_str_unchecked(src: &'a str) -> Self {
        if cfg!(debug_assertions) {
            match Self::from_str(src) {
                Ok(val) => val,
                Err(_) => {
                    panic!("AttributeName::from_str_unchecked() with illegal chars")
                }
            }
        } else {
            Self(Cow::Borrowed(src))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for AttributeName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for AttributeName<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for AttributeName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> TryFrom<&'a str> for AttributeName<'a> {
    type Error = IllegalChar;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<'a> TryFrom<&'a String> for AttributeName<'a> {
    type Error = IllegalChar;
    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}
