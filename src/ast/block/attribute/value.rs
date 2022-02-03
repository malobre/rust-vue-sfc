use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

pub use self::error::InvalidAttributeValue;

mod error {
    use std::error::Error;
    use std::fmt::Display;

    /// Returned when a function was unable to convert a string to an
    /// [`AttributeValue`][super::AttributeValue].
    #[derive(Debug)]
    pub struct InvalidAttributeValue(pub(super) char);

    impl Display for InvalidAttributeValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "illegal char: `{}`", self.0)
        }
    }

    impl Error for InvalidAttributeValue {}
}

/// The value of an attribute, i.e: `ts` in `<script lang="ts">`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
pub struct AttributeValue<'a>(Cow<'a, str>);

impl<'a> AttributeValue<'a> {
    /// Attempts to convert a string to an [`AttributeValue`].
    ///
    /// # Errors
    /// Will return an error if the string contains both
    /// a `U+0022 QUOTATION MARK (")` and an `U+0027 APOSTROPHE (')`.
    pub fn from_cow(src: Cow<'a, str>) -> Result<Self, InvalidAttributeValue> {
        if src.contains('\u{0022}') {
            if src.contains('\u{0027}') {
                return Err(InvalidAttributeValue('\u{0027}'));
            }

            return Ok(Self(src));
        }

        Ok(Self(src))
    }

    /// Convert a string into an [`AttributeValue`] **without** validating
    /// (unless `debug_assertions` is enabled).
    ///
    /// # Panics
    /// If `debug_assertions` is enabled, validate the input and panic on failure.
    ///
    /// # Safety
    /// See string prerequisites of [`AttributeValue::from_cow`].
    pub unsafe fn from_cow_unchecked(src: Cow<'a, str>) -> Self {
        if cfg!(debug_assertions) {
            match Self::from_cow(src) {
                Ok(val) => val,
                Err(err) => {
                    panic!("AttributeValue::from_cow_unchecked(): {err}")
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

impl Deref for AttributeValue<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for AttributeValue<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for AttributeValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for AttributeValue<'a> {
    type Error = InvalidAttributeValue;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::from_cow(value)
    }
}

impl<'a> TryFrom<&'a str> for AttributeValue<'a> {
    type Error = InvalidAttributeValue;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for AttributeValue<'a> {
    type Error = InvalidAttributeValue;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Owned(value))
    }
}
