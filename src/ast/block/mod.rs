use std::{borrow::Cow, fmt::Display};

pub use self::attribute::{Attribute, Name as AttributeName, Value as AttributeValue};
pub use self::name::Name;

mod attribute;
mod name;

/// A block as defined in the [SFC specifications][1].
///
/// [1]: https://v3.vuejs.org/api/sfc-spec.html#language-blocks
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    pub name: Name<'a>,
    pub attributes: Vec<(AttributeName<'a>, Option<AttributeValue<'a>>)>,
    pub content: Cow<'a, str>,
}

impl Display for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            attributes,
            content
        } = self;

        let content = content.trim_end();
        write!(f, "<{name}")?;

        for (name, value) in attributes {
            match value {
                Some(value) if value.as_str().contains('\u{0022}') => {
                    write!(f, " {name}='{value}'")?;
                }
                Some(value) => {
                    write!(f, r#" {name}="{value}""#)?;
                }
                None => {
                    write!(f, " {name}")?;
                }
            }
        }

        write!(f, ">")?;

        if !content.is_empty() {
            writeln!(f)?;
            writeln!(f, "{content}")?;
        }

        write!(f, "</{name}>")
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{AttributeName, AttributeValue, Block, Name};

    #[test]
    fn test_display() {
        assert_eq!(
            Block {
                name: Name::try_from("template").unwrap(),
                attributes: Vec::new(),
                content: Cow::Borrowed("")
            }
            .to_string(),
            "<template></template>"
        );

        assert_eq!(
            Block {
                name: Name::try_from("script").unwrap(),
                attributes: vec![(
                    AttributeName::try_from("lang").unwrap(),
                    Some(AttributeValue::try_from("ts").unwrap())
                )],
                content: Cow::Borrowed("")
            }
            .to_string(),
            r#"<script lang="ts"></script>"#
        );

        assert_eq!(
            Block {
                name: Name::try_from("script").unwrap(),
                attributes: vec![
                    (
                        AttributeName::try_from("lang").unwrap(),
                        Some(AttributeValue::try_from("ts").unwrap())
                    ),
                    (AttributeName::try_from("setup").unwrap(), None)
                ],
                content: Cow::Borrowed("")
            }
            .to_string(),
            r#"<script lang="ts" setup></script>"#
        );

        assert_eq!(
            Block {
                name: Name::try_from("style").unwrap(),
                attributes: vec![(AttributeName::try_from("scoped").unwrap(), None)],
                content: Cow::Borrowed("")
            }
            .to_string(),
            r#"<style scoped></style>"#
        );

        assert_eq!(
            Block {
                name: Name::try_from("template").unwrap(),
                attributes: Vec::new(),
                content: Cow::Borrowed("<!-- content -->")
            }
            .to_string(),
            concat!("<template>\n", "<!-- content -->\n", "</template>")
        );

        assert_eq!(
            Block {
                name: Name::try_from("template").unwrap(),
                attributes: Vec::new(),
                content: Cow::Borrowed("<!-- multiline -->\n<!-- content -->")
            }
            .to_string(),
            concat!(
                "<template>\n",
                "<!-- multiline -->\n",
                "<!-- content -->\n",
                "</template>"
            )
        );
    }
}
