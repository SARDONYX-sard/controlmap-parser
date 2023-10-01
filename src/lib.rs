//! # ControlMap Parser
//!
//! `controlmap_parser` is a minimal parser for controlmap.txt files.
//!
//! # EBNF
//! ```EBNF
//! <file> ::= <line>*
//!
//! <line> ::= <comment-line>
//!          | <event-line>
//!          | <blank-line>
//!
//! <comment-line> ::= "//" <string>
//!
//! <event-line> ::= <event-name> "\t"+ <keyboard-id> "\t"+ <mouse-id> "\t"+ <gamepad-id> "\t"+ <remap-key> "\t"+ <remap-mouse> "\t"+ <remap-gamepad> "\t"+ [<event-binary-flag>] <new-line>
//!
//! <event-name> ::= <string>
//!
//! <keyboard-id> ::= <key-map>
//!
//! <mouse-id> ::= <key-map>
//!
//! <gamepad-id> ::= <key-map>
//!
//! <remap-key> ::= "1" | "0"
//!
//! <remap-mouse> ::= "1" | "0"
//!
//! <remap-gamepad> ::= "1" | "0"
//!
//! <event-binary-flag> ::= <hexadecimal>
//!
//! <blank-line> ::= <new-line>
//!
//! # KeyMap
//! <key-map> ::= <key-alias> | <key-or> | <key-and> | <hexadecimal>
//! <key-alias> ::= "!0," <event-name>
//! <key-or> ::= <key-map> "," <key-map>
//! <key-and> ::= <key-id> "+" <key-id>
//! <key-id> ::= <hexadecimal>
//!
//! # primitives
//!
//! <new-line> ::= "\r"? "\n"
//! <string> ::= Any valid string
//!
//! <hexadecimal> ::= "0x" (<hex-digit>)+
//!
//! <hex-digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"  |
//!                   "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F"
//! ```
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{
        char, hex_digit1, line_ending, multispace0, multispace1, not_line_ending, space0,
    },
    combinator::{map, opt, recognize},
    error::{context, ErrorKind, ParseError},
    multi::{many0, many1},
    sequence::preceded,
    AsChar, InputTakeAtPosition,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyID<'a> {
    Or(Vec<KeyID<'a>>),
    And(Vec<KeyID<'a>>),
    One(&'a str),
    Alias(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventLine<'a> {
    pub event_name: &'a str,
    pub keyboard_id: KeyID<'a>,
    pub mouse_id: KeyID<'a>,
    pub gamepad_id: KeyID<'a>,
    pub remap_key: bool,
    pub remap_mouse: bool,
    pub remap_gamepad: bool,
    pub event_binary_flag: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Line<'a> {
    Comment(&'a str),
    EventLine(EventLine<'a>),
    BlankLine,
}

/// one or more tab space
fn tab_space1<T, E: ParseError<T>>(input: T) -> nom::IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(|item| !(item.as_char() == '\t'), ErrorKind::Space)
}

type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

fn parse_hex<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let (input, _) = space0(input)?;
    recognize(preceded(tag("0x"), hex_digit1))(input)
}

fn parse_flag(input: &str) -> IResult<&str, bool> {
    let (input, flag) = alt((char('0'), char('1')))(input)?;
    let flag = match flag {
        '0' => false,
        '1' => true,
        _ => unreachable!(),
    };
    Ok((input, flag))
}

fn parse_key_and<'a>(input: &'a str) -> IResult<&'a str, KeyID<'a>> {
    let (input, key) = parse_key_one(input)?;
    let (input, ref mut keys) = many1(preceded(tag("+"), parse_key_one))(input)?;
    let mut res = vec![key];
    res.append(keys);
    Ok((input, KeyID::And(res)))
}

fn parse_key_or<'a>(input: &'a str) -> IResult<&'a str, KeyID<'a>> {
    let mut parse_and1 = alt((parse_key_and, parse_key_one));

    let (input, key) = parse_and1(input)?;
    let (input, ref mut keys) = many1(preceded(tag(","), parse_and1))(input)?;
    let mut res = vec![key];
    res.append(keys);
    Ok((input, KeyID::Or(res)))
}

fn parse_key_one<'a>(input: &'a str) -> IResult<&'a str, KeyID<'a>> {
    map(parse_hex, |key| KeyID::One(key))(input)
}

fn parse_key_alias<'a>(input: &'a str) -> IResult<&'a str, KeyID<'a>> {
    map(preceded(tag("!0,"), parse_event_name), |key| {
        KeyID::Alias(key)
    })(input)
}

fn parse_key_id<'a>(input: &'a str) -> IResult<&'a str, KeyID<'a>> {
    alt((parse_key_alias, parse_key_or, parse_key_and, parse_key_one))(input)
}

fn parse_event_name(input: &str) -> IResult<&str, &str> {
    context(
        "Expected ident. non tab any string",
        take_while1(|c: char| c != '\t'),
    )(input)
}

fn parse_comment_line<'a>(input: &'a str) -> IResult<&'a str, Line<'a>> {
    let (input, comment) = preceded(tag("//"), not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, Line::Comment(comment)))
}

fn parse_event_line<'a>(input: &'a str) -> IResult<&'a str, Line<'a>> {
    let (input, event_name) = parse_event_name(input)?;
    let (input, _) = tab_space1(input)?;
    let (input, keyboard_id) = parse_key_id(input)?;
    let (input, _) = tab_space1(input)?;
    let (input, mouse_id) = parse_key_id(input)?;
    let (input, _) = tab_space1(input)?;
    let (input, gamepad_id) = parse_key_id(input)?;
    let (input, _) = tab_space1(input)?;
    let (input, remap_key) = parse_flag(input)?;
    let (input, _) = tab_space1(input)?;
    let (input, remap_mouse) = parse_flag(input)?;
    let (input, _) = tab_space1(input)?;
    let (input, remap_gamepad) = parse_flag(input)?;
    let (input, event_binary_flag) = opt(preceded(tab_space1, parse_hex))(input)?;
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        Line::EventLine(EventLine {
            event_name,
            keyboard_id,
            mouse_id,
            gamepad_id,
            remap_key,
            remap_mouse,
            remap_gamepad,
            event_binary_flag,
        }),
    ))
}

fn parse_blank_line<'a>(input: &'a str) -> IResult<&'a str, Line<'a>> {
    let (input, _) = multispace1(input)?;
    Ok((input, Line::BlankLine))
}

/// parse controlmap.txt
///
/// # Examples
/// ```
/// use pretty_assertions::assert_eq;
/// use controlmap_parser::{control_map_parser, EventLine, Line, KeyID};
///
/// let input = r#"
/// // Main Gameplay
/// Forward				0x11		0xff	0xff			1	1	0	0x801
/// Back				0x1f		0xff	0xff			1	1	0	0x801
/// // Menu Mode
/// Accept		!0,Activate	0xff	0x2000	0	0	0	0x8
/// "#;
/// let actual = control_map_parser(input);
///
/// let expected = Ok((
///     "",
///     vec![
///         Line::BlankLine,
///         Line::Comment(" Main Gameplay"),
///         Line::EventLine(EventLine {
///             event_name: "Forward",
///             keyboard_id: KeyID::One("0x11"),
///             mouse_id: KeyID::One("0xff"),
///             gamepad_id: KeyID::One("0xff"),
///             remap_key: true,
///             remap_mouse: true,
///             remap_gamepad: false,
///             event_binary_flag: Some("0x801"),
///         }),
///         Line::EventLine(EventLine {
///             event_name: "Back",
///             keyboard_id: KeyID::One("0x1f"),
///             mouse_id: KeyID::One("0xff"),
///             gamepad_id: KeyID::One("0xff"),
///             remap_key: true,
///             remap_mouse: true,
///             remap_gamepad: false,
///             event_binary_flag: Some("0x801"),
///         }),
///         Line::Comment(" Menu Mode"),
///         Line::EventLine(EventLine {
///             event_name: "Accept",
///             keyboard_id: KeyID::Alias("Activate"),
///             mouse_id: KeyID::One("0xff"),
///             gamepad_id: KeyID::One("0x2000"),
///             remap_key: false,
///             remap_mouse: false,
///             remap_gamepad: false,
///             event_binary_flag: Some("0x8"),
///         }),
///     ],
/// ));
/// assert_eq!(actual, expected);
/// ```
pub fn control_map_parser(input: &str) -> IResult<&str, Vec<Line>> {
    let parse_line = alt((parse_blank_line, parse_comment_line, parse_event_line));
    many0(parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_hex() {
        let input = "0x1234";
        let expected_output = Ok(("", "0x1234"));
        let result = parse_hex(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_parse_flag() {
        let input = "0";
        let expected_output = Ok(("", false));
        let result = parse_flag(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_parse_key_id_one() {
        let input = "0x1234";
        let expected_output = Ok(("", KeyID::One("0x1234")));
        let result = parse_key_id(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_parse_key_id_and() {
        let input = "0x1234+0x5678+0x9abc";
        let expected_output = Ok((
            "",
            KeyID::And(vec![
                KeyID::One("0x1234"),
                KeyID::One("0x5678"),
                KeyID::One("0x9abc"),
            ]),
        ));
        let result = parse_key_id(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_parse_key_id_or() {
        let input = "0x2a+0x0f,0x36+0x0,0x1234,0x5678,0x9abc";
        let expected_output = Ok((
            "",
            KeyID::Or(vec![
                KeyID::And(vec![KeyID::One("0x2a"), KeyID::One("0x0f")]),
                KeyID::And(vec![KeyID::One("0x36"), KeyID::One("0x0")]),
                KeyID::One("0x1234"),
                KeyID::One("0x5678"),
                KeyID::One("0x9abc"),
            ]),
        ));
        let result = parse_key_id(input);
        assert_eq!(result, expected_output)
    }

    #[test]
    fn test_parse_comment_line() {
        let input = "// This is a comment\n";
        let expected_output = Ok(("", Line::Comment(" This is a comment")));
        let result = parse_comment_line(input);
        assert_eq!(result, expected_output);
    }
}
