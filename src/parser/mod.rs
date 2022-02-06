use std::borrow::Cow;

pub use self::error::ParseError;
use self::util::parse_start_tag;

use crate::{parser::util::parse_end_tag, Attribute, Block, BlockName, Raw, Section};

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
pub fn parse(mut input: &str) -> Result<Vec<Section<'_>>, ParseError> {
    //TODO: Improve readability / refactor code.

    #[derive(Debug)]
    enum State<'a> {
        OutsideBlock {
            index: usize,
        },
        InsideBlock {
            name: BlockName<'a>,
            attributes: Vec<Attribute<'a>>,
            depth: usize,
            index: usize,
        },
    }

    let mut buffer = Vec::new();
    let mut state = State::OutsideBlock { index: 0 };

    while !input.is_empty() {
        // Check for start tag
        match state {
            State::InsideBlock {
                ref name,
                index,
                ref mut depth,
                ..
            } => {
                if let Ok((_, (next_tag_name, _))) = parse_start_tag(&input[index..]) {
                    if *name == next_tag_name {
                        *depth += 1;
                    }
                }
            }
            State::OutsideBlock { index } => {
                if let Ok((remaining, (name, attributes))) = parse_start_tag(&input[index..]) {
                    let content = input[..index].trim_start_matches(['\n', '\r']).trim_end();

                    if !content.is_empty() {
                        // SAFETY: `content` is end-trimmed and non-empty.
                        let raw = unsafe { Raw::from_cow_unchecked(Cow::Borrowed(content)) };
                        buffer.push(Section::Raw(raw));
                    }

                    state = State::InsideBlock {
                        name,
                        attributes,
                        depth: 0,
                        index: 0,
                    };
                    input = remaining;
                }
            }
        }

        // Check for end tag
        if let State::InsideBlock {
            ref name,
            ref mut depth,
            index,
            ..
        } = state
        {
            if let Ok((remaining, _)) = parse_end_tag(name, &input[index..]) {
                if *depth == 0 {
                    if let State::InsideBlock {
                        name, attributes, ..
                    } = std::mem::replace(&mut state, State::OutsideBlock { index: 0 })
                    {
                        let content = Cow::Borrowed(
                            input[..index].trim_start_matches(['\n', '\r']).trim_end(),
                        );

                        buffer.push(Section::Block(Block {
                            name,
                            attributes,
                            content,
                        }));
                    }

                    input = remaining;

                    // Index was just reset, don't advance to next `<`.
                    continue;
                }

                *depth -= 1;
            }
        }

        // Advance index to next `<`.
        match state {
            State::InsideBlock { ref mut index, .. } | State::OutsideBlock { ref mut index } => {
                if let Some(j) = input.get((*index + 1)..).and_then(|input| input.find('<')) {
                    *index += j + 1;
                } else if let State::InsideBlock { name, .. } = state {
                    return Err(ParseError::MissingEndTag(name.as_str().to_owned()));
                } else {
                    let content = input.trim_start_matches(['\n', '\r']).trim_end();

                    if !content.is_empty() {
                        // SAFETY: `content` is end-trimmed and non-empty.
                        let raw = unsafe { Raw::from_cow_unchecked(Cow::Borrowed(content)) };
                        buffer.push(Section::Raw(raw));
                    }

                    return Ok(buffer);
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
