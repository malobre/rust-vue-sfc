use std::borrow::Cow;

pub use self::error::ParseError;
use self::util::parse_start_tag;

use crate::{parser::util::parse_end_tag, Attribute, Block, BlockName, Section};

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
                    if !input[..index].is_empty() {
                        buffer.push(Section::Raw(Cow::Borrowed(&input[..index])));
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
                        buffer.push(Section::Block(Block {
                            name,
                            attributes,
                            content: Cow::Borrowed(&input[..index]),
                        }));
                    }

                    input = remaining;
                } else {
                    *depth -= 1;
                }
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
                    if !input.is_empty() {
                        buffer.push(Section::Raw(Cow::Borrowed(input)));
                    }

                    return Ok(buffer);
                }
            }
        }
    }

    Ok(buffer)
}
