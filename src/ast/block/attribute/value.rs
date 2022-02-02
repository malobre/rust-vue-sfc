use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

use crate::ast::error::IllegalChar;

/// The value of an attribute, i.e: `ts` in `<script lang="ts">`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Value<'a>(Cow<'a, str>);

impl<'a> Value<'a> {
    /// Create a new [`Value`].
    ///
    /// # Panics
    /// Will panic where `Self::try_new` would error.
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        match Self::try_new(value) {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        }
    }

    /// Create a new [`Value`].
    ///
    /// # Errors
    /// Will return an error if the string contains both
    /// a `U+0022 QUOTATION MARK (")` and an `U+0027 APOSTROPHE (')`.
    pub fn try_new(value: impl Into<Cow<'a, str>>) -> Result<Self, IllegalChar> {
        let value = value.into();

        if value.contains('\u{0022}') {
            if value.contains('\u{0027}') {
                return Err(IllegalChar('\u{0027}'));
            }

            return Ok(Self(value));
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Value<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for Value<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Value<'a> {
    type Error = IllegalChar;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'a> TryFrom<&'a str> for Value<'a> {
    type Error = IllegalChar;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'a> TryFrom<String> for Value<'a> {
    type Error = IllegalChar;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
