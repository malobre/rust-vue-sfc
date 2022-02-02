pub use self::name::{AttributeName, InvalidAttributeName};
pub use self::value::{AttributeValue, InvalidAttributeValue};

/// Convenience type alias.
pub type Attribute<'a> = (AttributeName<'a>, Option<AttributeValue<'a>>);

mod name;
mod value;
