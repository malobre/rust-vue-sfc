pub use self::name::AttributeName;
pub use self::value::AttributeValue;

/// Convenience type alias.
pub type Attribute<'a> = (AttributeName<'a>, Option<AttributeValue<'a>>);

mod name;
mod value;
