use std::borrow::Cow;

pub use self::error::ParseError;
use self::util::parse_start_tag;

use crate::{parser::util::parse_end_tag, Attribute, Block, BlockName, Section};

mod error;

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

mod util {
    use crate::{Attribute, AttributeName, AttributeValue, BlockName};

    use nom::{
        branch::alt,
        bytes::complete::{tag_no_case, take_until, take_while1},
        character::complete::{char, multispace0, multispace1},
        combinator::opt,
        multi::many0,
        sequence::{delimited, pair, preceded, tuple},
        IResult, Parser,
    };

    pub fn parse_end_tag<'a, 'b>(name: &'b str, input: &'a str) -> IResult<&'a str, &'a str> {
        delimited(
            tuple((char('<'), char('/'), multispace0)),
            tag_no_case(name),
            tuple((multispace0, char('>'))),
        )
        .parse(input)
    }

    pub fn parse_start_tag(input: &str) -> IResult<&str, (BlockName, Vec<Attribute>)> {
        delimited(
            char('<'),
            tuple((
                preceded(multispace0, parse_start_tag_name),
                many0(preceded(multispace1, parse_start_tag_attribute)),
            )),
            preceded(multispace0, char('>')),
        )
        .parse(input)
    }

    fn parse_start_tag_attribute(input: &str) -> IResult<&str, Attribute> {
        pair(
            parse_start_tag_attribute_name,
            opt(preceded(
                delimited(multispace0, char('='), multispace0),
                alt((
                    delimited(char('\u{0022}'), take_until("\u{0022}"), char('\u{0022}')),
                    delimited(char('\u{0027}'), take_until("\u{0027}"), char('\u{0027}')),
                ))
                .map(AttributeValue::new),
            )),
        )
        .parse(input)
    }

    fn parse_start_tag_attribute_name(input: &str) -> IResult<&str, AttributeName> {
        take_while1(|ch: char| {
            !matches!(
                ch,
                '\u{0009}'
                    | '\u{000A}'
                    | '\u{000C}'
                    | '\u{0020}'
                    | '\u{002F}'
                    | '\u{003D}'
                    | '\u{003E}'
            )
        })
        .map(AttributeName::new)
        .parse(input)
    }

    fn parse_start_tag_name(input: &str) -> IResult<&str, BlockName> {
        take_while1(|ch: char| {
            !matches!(
                ch,
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{002F}' | '\u{003E}'
            )
        })
        .map(BlockName::new)
        .parse(input)
    }
}
