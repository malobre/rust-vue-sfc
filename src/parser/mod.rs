use std::borrow::Cow;

pub use self::error::ParseError;
use self::util::parse_start_tag;

use crate::{
    parser::util::parse_end_tag, Attribute, AttributeValue, Block, BlockName, Raw, Section,
};

mod error;
mod util;

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
    #[derive(Debug)]
    enum State<'a> {
        OutsideBlock {
            offset: usize,
        },
        InsideBlock {
            name: BlockName<'a>,
            attributes: Vec<Attribute<'a>>,
            depth: usize,
            offset: usize,
        },
    }

    let mut less_than_symbols = std::iter::successors(input.find('<'), |&last| {
        let offset = last + 1;

        input
            .get(offset..)
            .and_then(|s| s.find('<'))
            .map(|n| offset + n)
    });

    let mut buffer = Vec::new();
    let mut state = State::OutsideBlock { offset: 0 };

    loop {
        if let Some(index) = less_than_symbols.next() {
            if let Ok((remaining, (name, attributes))) = parse_start_tag(&input[index..]) {
                match state {
                    State::OutsideBlock { offset } => {
                        let raw = input[offset..index]
                            .trim_start_matches(['\n', '\r'])
                            .trim_end();

                        if !raw.is_empty() {
                            // SAFETY: `raw` is end-trimmed and non-empty.
                            let raw = unsafe { Raw::from_cow_unchecked(Cow::Borrowed(raw)) };
                            buffer.push(Section::Raw(raw));
                        }

                        state = State::InsideBlock {
                            name,
                            attributes,
                            depth: 0,
                            offset: input.len() - remaining.len(),
                        };
                    }
                    State::InsideBlock {
                        name: ref parent_name,
                        ref mut depth,
                        ref attributes,
                        ..
                    } => {
                        let raw_text = parent_name.as_str() != "template"
                            || attributes.iter().any(|(name, value)| {
                                matches!(
                                    (name.as_str(), value.as_ref().map(AttributeValue::as_str)),
                                    ("lang", Some(lang)) if lang != "html"
                                )
                            });

                        if !raw_text && parent_name == &name {
                            *depth += 1;
                        }
                    }
                }
            } else if let Ok((remaining, name)) = parse_end_tag(&input[index..]) {
                match state {
                    State::OutsideBlock { .. } => {
                        return Err(ParseError::UnexpectedEndTag(name.as_str().to_owned()));
                    }
                    State::InsideBlock {
                        name: ref parent_name,
                        ref mut depth,
                        ref mut attributes,
                        offset,
                    } => {
                        if &name == parent_name {
                            if *depth == 0 {
                                buffer.push(Section::Block(Block {
                                    name,
                                    attributes: std::mem::take(attributes),
                                    content: Cow::Borrowed(
                                        input[offset..index]
                                            .trim_start_matches(['\n', '\r'])
                                            .trim_end(),
                                    ),
                                }));

                                state = State::OutsideBlock {
                                    offset: input.len() - remaining.len(),
                                };
                            } else {
                                *depth -= 1;
                            }
                        }
                    }
                }
            }
        } else {
            match state {
                State::OutsideBlock { offset } => {
                    let raw = input[offset..].trim_start_matches(['\n', '\r']).trim_end();

                    if !raw.is_empty() {
                        // SAFETY: `raw` is end-trimmed and non-empty.
                        let raw = unsafe { Raw::from_cow_unchecked(Cow::Borrowed(raw)) };
                        buffer.push(Section::Raw(raw));
                    }

                    break;
                }
                State::InsideBlock { name, .. } => {
                    return Err(ParseError::MissingEndTag(name.as_str().to_owned()));
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
