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

/// # References
/// - <https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#end-tag-open-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state>
pub fn parse_end_tag<'a, 'b>(name: &'b str, input: &'a str) -> IResult<&'a str, &'a str> {
    delimited(
        tuple((char('<'), char('/'), multispace0)),
        tag_no_case(name),
        tuple((multispace0, char('>'))),
    )
    .parse(input)
}

/// # References
/// - <https://html.spec.whatwg.org/multipage/parsing.html#data-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state>
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

/// # References
/// - <https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-name-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state>
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

/// # References
/// - <https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state>
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

/// # References
/// - <https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state>
/// - <https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state>
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
