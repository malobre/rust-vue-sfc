use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

#[derive(Debug)]
pub struct Invalid {
    kind: InvalidKind,
}

impl std::fmt::Display for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            InvalidKind::StartsWithNonAsciiAlpha => {
                write!(f, "block name must start with ASCII alpha")
            }
            InvalidKind::IllegalChar(ch) => write!(f, "block name cannot contain `{}`", ch),
        }
    }
}

impl std::error::Error for Invalid {}

#[derive(Debug)]
enum InvalidKind {
    IllegalChar(char),
    StartsWithNonAsciiAlpha,
}

/// The name of a block, i.e: `script` in `<script lang="ts">`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockName<'a>(Cow<'a, str>);

impl<'a> BlockName<'a> {
    /// Create a new [`BlockName`].
    ///
    /// # Panics
    /// Will panic where [`Self::try_new`] would error.
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        match Self::try_new(value) {
            Ok(name) => name,
            Err(err) => panic!("{}", err),
        }
    }

    /// Try to create a new [`BlockName`].
    ///
    /// # Errors
    /// Will return an error if the string contains any of the following characters:
    /// - `U+0009 CHARACTER TABULATION`,
    /// - `U+000A LINE FEED`,
    /// - `U+000C FORM FEED`,
    /// - `U+0020 SPACE`,
    /// - `U+002F SOLIDUS (/)`,
    /// - `U+003E GREATER-THAN SIGN (>)`.
    pub fn try_new(value: impl Into<Cow<'a, str>>) -> Result<Self, Invalid> {
        let mut value = value.into();

        if !value.starts_with(|ch: char| ch.is_ascii_alphabetic()) {
            return Err(Invalid {
                kind: InvalidKind::StartsWithNonAsciiAlpha,
            });
        }

        if let Some(ch) = value.chars().find(|ch| {
            matches!(
                ch,
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{002F}' | '\u{003E}'
            )
        }) {
            return Err(Invalid {
                kind: InvalidKind::IllegalChar(ch),
            });
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

impl<'a> TryFrom<Cow<'a, str>> for BlockName<'a> {
    type Error = Invalid;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'a> TryFrom<&'a str> for BlockName<'a> {
    type Error = Invalid;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'a> TryFrom<String> for BlockName<'a> {
    type Error = Invalid;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
