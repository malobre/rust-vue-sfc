use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

use crate::ast::error::IllegalChar;

/// The name of an attribute, i.e: `lang` in `<script lang="ts">`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name<'a>(Cow<'a, str>);

impl<'a> Name<'a> {
    /// Create a new [`Name`].
    ///
    /// # Panics
    /// Will panic where [`Self::try_new`] would error.
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        match Self::try_new(value) {
            Ok(name) => name,
            Err(err) => panic!("{}", err),
        }
    }

    /// Try to create a new [`Name`].
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
    pub fn try_new(value: impl Into<Cow<'a, str>>) -> Result<Self, IllegalChar> {
        let mut value = value.into();

        if let Some(ch) = value.chars().find(|ch| {
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

        if value.contains(|ch: char| ch.is_ascii_uppercase()) {
            value.to_mut().make_ascii_lowercase();
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Name<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for Name<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for Name<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Name<'a> {
    type Error = IllegalChar;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'a> TryFrom<&'a str> for Name<'a> {
    type Error = IllegalChar;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'a> TryFrom<String> for Name<'a> {
    type Error = IllegalChar;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
