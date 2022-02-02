pub use self::name::Name;
pub use self::value::Value;

/// Convenience type alias.
pub type Attribute<'a> = (Name<'a>, Option<Value<'a>>);

mod name;
mod value;
