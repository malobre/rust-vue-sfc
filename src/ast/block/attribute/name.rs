use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

pub use self::error::InvalidAttributeName;

mod error {
    use std::error::Error;
    use std::fmt::Display;

    /// Returned when a function was unable to convert a string to an
    /// [`AttributeName`][super::AttributeName].
    #[derive(Debug)]
    pub struct InvalidAttributeName(pub(super) char);

    impl Display for InvalidAttributeName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "illegal char: `{}`", self.0)
        }
    }

    impl Error for InvalidAttributeName {}
}

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
    pub fn from_cow(mut src: Cow<'a, str>) -> Result<Self, InvalidAttributeName> {
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
            return Err(InvalidAttributeName(ch));
        }

        if src.contains(|ch: char| ch.is_ascii_uppercase()) {
            src.to_mut().make_ascii_lowercase();

            Ok(Self(src))
        } else {
            Ok(Self(src))
        }
    }

    /// Convert a string into an [`AttributeName`] **without** validating.
    pub unsafe fn from_cow_unchecked(src: Cow<'a, str>) -> Self {
        if cfg!(debug_assertions) {
            match Self::from_cow(src) {
                Ok(val) => val,
                Err(err) => {
                    panic!("AttributeName::from_cow_unchecked(): {err}")
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
    type Error = InvalidAttributeName;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for AttributeName<'a> {
    type Error = InvalidAttributeName;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Owned(value))
    }
}

impl<'a> TryFrom<Cow<'a, str>> for AttributeName<'a> {
    type Error = InvalidAttributeName;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::from_cow(value)
    }
}
