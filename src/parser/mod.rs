use std::borrow::Cow;

pub use self::error::ParseError;
use self::util::{parse_start_tag, trim_start_newlines_end};

use crate::{
    parser::util::parse_end_tag, Attribute, AttributeValue, Block, BlockName, Raw, Section,
};

mod error;
mod util;

/// Represent the state of the parser.
#[derive(Debug)]
enum State<'a> {
    /// When the parser is at root level.
    Root,
    /// When the parser is in a block in `data state`.
    /// See <https://html.spec.whatwg.org/multipage/parsing.html#data-state>.
    Data {
        name: BlockName<'a>,
        attributes: Vec<Attribute<'a>>,
        depth: u16,
    },
    /// When the parser is in a block in `RAWTEXT state`.
    /// See <https://html.spec.whatwg.org/multipage/parsing.html#rawtext-state>.
    RawText {
        name: BlockName<'a>,
        attributes: Vec<Attribute<'a>>,
    },
}

/// Parse the given input as a Vue SFC.
///
/// # Errors
/// Will return an error if parsing fails.
///
/// # Example
/// ```rust
/// use vue_sfc::{Section, Block};
///
/// let sfc = vue_sfc::parse("<!-- your input -->").unwrap();
///
/// for section in sfc {
///     match section {
///         Section::Block(Block { name, attributes, content }) => {
///             println!(
///                 "Got a block named `{}` with {} attributes, content is {} bytes long.",
///                 name,
///                 attributes.len(),
///                 content.len()
///             )
///         }
///         Section::Raw(content) => {
///             println!(
///                 "Got a raw section, {} bytes long.",
///                 content.len()
///             )
///         }
///     }
/// }
/// ```
pub fn parse(input: &str) -> Result<Vec<Section<'_>>, ParseError> {
    let mut less_than_symbols = memchr::memmem::find_iter(input.as_bytes(), "<");

    let mut buffer = Vec::new();
    let mut offset = 0;
    let mut state = State::Root;

    loop {
        match state {
            State::Root => {
                let index = if let Some(index) = less_than_symbols.next() {
                    index
                } else {
                    let raw = trim_start_newlines_end(&input[offset..]);

                    if !raw.is_empty() {
                        // SAFETY: `raw` is end-trimmed and non-empty.
                        let raw = unsafe { Raw::from_cow_unchecked(Cow::Borrowed(raw)) };
                        buffer.push(Section::Raw(raw));
                    }

                    break;
                };

                if let Ok((_, name)) = parse_end_tag(&input[index..]) {
                    return Err(ParseError::UnexpectedEndTag(name.as_str().to_owned()));
                }

                if let Ok((remaining, (name, attributes))) = parse_start_tag(&input[index..]) {
                    let raw = trim_start_newlines_end(&input[offset..index]);

                    if !raw.is_empty() {
                        // SAFETY: `raw` is end-trimmed and non-empty.
                        let raw = unsafe { Raw::from_cow_unchecked(Cow::Borrowed(raw)) };
                        buffer.push(Section::Raw(raw));
                    }

                    let raw_text = name.as_str() != "template"
                        || attributes.iter().any(|(name, value)| {
                            matches!(
                                (name.as_str(), value.as_ref().map(AttributeValue::as_str)),
                                ("lang", Some(lang)) if lang != "html"
                            )
                        });

                    offset = input.len() - remaining.len();
                    state = if raw_text {
                        State::RawText { name, attributes }
                    } else {
                        State::Data {
                            name,
                            attributes,
                            depth: 0,
                        }
                    };
                }
            }
            State::Data {
                name: ref parent_name,
                ref mut attributes,
                ref mut depth,
            } => {
                let index = less_than_symbols
                    .next()
                    .ok_or_else(|| ParseError::MissingEndTag(parent_name.as_str().to_owned()))?;

                match parse_end_tag(&input[index..]) {
                    Ok((remaining, name)) if &name == parent_name => {
                        if *depth == 0 {
                            buffer.push(Section::Block(Block {
                                name,
                                attributes: std::mem::take(attributes),
                                content: Cow::Borrowed(trim_start_newlines_end(
                                    &input[offset..index],
                                )),
                            }));

                            offset = input.len() - remaining.len();
                            state = State::Root;
                        } else {
                            *depth -= 1;
                        }

                        // Skip start tag check.
                        continue;
                    }
                    _ => { /* Ignore parsing failure & non-matching end tag. */ }
                }

                match parse_start_tag(&input[index..]) {
                    Ok((_, (name, _))) if &name == parent_name => {
                        *depth += 1;
                    }
                    _ => { /* Ignore parsing failure & non-matching start tag. */ }
                }
            }
            State::RawText {
                name: ref parent_name,
                ref mut attributes,
            } => {
                let index = less_than_symbols
                    .next()
                    .ok_or_else(|| ParseError::MissingEndTag(parent_name.as_str().to_owned()))?;

                match parse_end_tag(&input[index..]) {
                    Ok((remaining, name)) if &name == parent_name => {
                        buffer.push(Section::Block(Block {
                            name,
                            attributes: std::mem::take(attributes),
                            content: Cow::Borrowed(trim_start_newlines_end(&input[offset..index])),
                        }));

                        offset = input.len() - remaining.len();
                        state = State::Root;
                    }
                    _ => { /* Ignore non-matching end tags. */ }
                }
            }
        }
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{Block, BlockName, Raw, Section};

    use super::parse;

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse("").unwrap(), vec![]);
    }

    #[test]
    fn test_parse_raw() {
        assert_eq!(
            parse("<!-- a comment -->").unwrap(),
            vec![Section::Raw(Raw::try_from("<!-- a comment -->").unwrap())]
        );
    }

    #[test]
    fn test_parse_block() {
        assert_eq!(
            parse("<template></template>").unwrap(),
            vec![Section::Block(Block {
                name: BlockName::try_from("template").unwrap(),
                attributes: vec![],
                content: Cow::default()
            })]
        );
    }

    #[test]
    fn test_parse_consecutive_blocks() {
        assert_eq!(
            parse("<template></template><script></script>").unwrap(),
            vec![
                Section::Block(Block {
                    name: BlockName::try_from("template").unwrap(),
                    attributes: vec![],
                    content: Cow::default()
                }),
                Section::Block(Block {
                    name: BlockName::try_from("script").unwrap(),
                    attributes: vec![],
                    content: Cow::default()
                })
            ]
        );
    }

    #[test]
    fn test_parse() {
        let raw = r#"<template>
  <router-view v-slot="{ Component }"
  >
    <suspense v-if="Component" :timeout="150">
      <template #default>
        <component :is="Component"/>
      </template>
      <template #fallback>
        Loading...
      </template>
    </suspense>
  </router-view>
</template>

<script lang="ts" setup>
onErrorCaptured((err) => {
  console.error(err);
});
</script>"#;

        let sfc = parse(raw).unwrap();

        match &sfc[0] {
            Section::Block(Block {
                name,
                attributes,
                content,
            }) => {
                assert_eq!(name.as_str(), "template");
                assert_eq!(content.len(), 266);
                assert!(attributes.is_empty());
            }
            _ => panic!("expected a block"),
        }

        match &sfc[1] {
            Section::Block(Block {
                name,
                attributes,
                content,
            }) => {
                assert_eq!(name.as_str(), "script");
                assert_eq!(content.len(), 52);
                assert_eq!(attributes[0].0.as_str(), "lang");
                assert_eq!(attributes[0].1.as_ref().unwrap().as_str(), "ts");
                assert_eq!(attributes[1].0.as_str(), "setup");
                assert!(attributes[1].1.is_none());
            }
            _ => panic!("expected a block"),
        }
    }
}
