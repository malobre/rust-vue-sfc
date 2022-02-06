use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
};

pub use self::error::InvalidRaw;

mod error {
    use std::error::Error;
    use std::fmt::Display;

    /// Returned when a function was unable to convert a string to a [`Raw`][super::Raw].
    #[derive(Debug)]
    pub struct InvalidRaw;

    impl Display for InvalidRaw {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "raw section must not be empty once end-trimmed")
        }
    }

    impl Error for InvalidRaw {}
}

/// Represent non-empty text before, after or between blocks.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
pub struct Raw<'a>(Cow<'a, str>);

impl<'a> Raw<'a> {
    /// Attempts to convert a string to a [`Raw`].
    ///
    /// # Errors
    /// Will return an error if the string is empty once end-trimmed.
    pub fn from_cow(src: Cow<'a, str>) -> Result<Self, InvalidRaw> {
        let trimmed = match src {
            Cow::Borrowed(string) => Cow::Borrowed(string.trim_end()),
            Cow::Owned(mut string) => {
                string.truncate(string.trim_end().len());

                Cow::Owned(string)
            }
        };

        if trimmed.is_empty() {
            return Err(InvalidRaw);
        }

        Ok(Self(trimmed))
    }

    /// Convert a string into a [`Raw`] **without** validating
    /// (unless `debug_assertions` is enabled).
    ///
    /// # Panics
    /// If `debug_assertions` is enabled, validate the input and panic on failure.
    ///
    /// # Safety
    /// See string prerequisites of [`Raw::from_cow`].
    pub unsafe fn from_cow_unchecked(src: Cow<'a, str>) -> Self {
        if cfg!(debug_assertions) {
            match Self::from_cow(src) {
                Ok(val) => val,
                Err(err) => {
                    panic!("Raw::from_cow_unchecked(): {err}")
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

impl Display for Raw<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Deref for Raw<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for Raw<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TryFrom<&'a str> for Raw<'a> {
    type Error = InvalidRaw;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Raw<'a> {
    type Error = InvalidRaw;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_cow(Cow::Owned(value))
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Raw<'a> {
    type Error = InvalidRaw;
    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::from_cow(value)
    }
}
