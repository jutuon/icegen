
use nom::{
    branch::alt, bytes::complete::{tag, take, take_until}, character::complete::{multispace0, multispace1}, combinator::{fail, map, opt}, multi::many0, sequence::{delimited, pair, preceded, tuple}, IResult, Parser
};

use super::{identifier::{identifier, Identifier}, literal::{string_literal, string_literal_str}, whitespace::{some_whitespace, wsc, LINE_COMMENT_END, LINE_COMMENT_START, MULTI_LINE_COMMENT_END, MULTI_LINE_COMMENT_START}};

#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub name: Identifier,
    pub parameters: String,
}

impl Annotation {
    pub fn is_freezed_annotation(&self) -> bool {
        self.name.name == "freezed" || self.name.name == "Freezed"
    }
}

pub fn annotation(input: &str) -> IResult<&str, Annotation> {
    let (input, _) = tag("@")(input)?;
    let (input, _) = wsc(input)?;
    let (input, identifier) = identifier(input)?;
    let (input, _) = wsc(input)?;
    let (input, parameters_str) = if input.starts_with("(") {
        delimited(
            tag("("),
            get_parameters_string,
            tag(")")
        )(input)?
    } else {
        (input, "")
    };

    Ok((input, Annotation { name: identifier, parameters: parameters_str.to_string() }))
}

fn wsc_and_annotation(input: &str) -> IResult<&str, Annotation> {
    let (input, _) = wsc(input)?;
    annotation(input)
}

pub fn annotations0(input: &str) -> IResult<&str, Vec<Annotation>> {
    many0(wsc_and_annotation)(input)
}


enum State<'a> {
    WhitespaceOrComment,
    StringLiteal,
    Character(Option<&'a str>),
}

/// Get the parameters string of an annotation recursively.
fn get_parameters_string(
    input: &str,
) -> IResult<&str, &str> {
    let original_input = input;

    let mut current_input = input;
    let mut consumed_len = 0;
    loop {

        let (input, selected) = alt((
            some_whitespace.map(|_| State::WhitespaceOrComment),
            string_literal_str.map(|_| State::StringLiteal),
            opt(take(1usize)).map(|v| State::Character(v))
        ))(current_input)?;

        consumed_len += current_input.len() - input.len();
        current_input = input;

        if let State::Character(c) = selected {
            match c {
                Some("(") => {
                    let (input, _) = get_parameters_string(input)?;
                    let (input, _) = tag(")")(input)?;
                    consumed_len += current_input.len() - input.len();
                    current_input = input;
                },
                Some(")") => {
                    return Ok((
                        &original_input[(consumed_len - 1)..],
                        &original_input[..(consumed_len - 1)]
                    ));
                },
                Some(_) => {
                    continue;
                }
                None => {
                    return fail(current_input);
                }

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(name: &str) -> Annotation {
        Annotation {
            name: Identifier { name: name.to_string() },
            parameters: "".to_string(),
        }
    }

    fn a_params(name: &str, params: &str) -> Annotation {
        Annotation {
            name: Identifier { name: name.to_string() },
            parameters: params.to_string(),
        }
    }


    #[test]
    fn annotation_starts_with_wrong_letter() {
        assert!(annotation("$test").is_err());
    }


    #[test]
    fn annotation_without_parameters() {
        assert_eq!(
            annotation("@a "),
            Ok(("", a("a")))
        );
    }

    #[test]
    fn annotation_empty_parameters() {
        assert_eq!(
            annotation("@a() "),
            Ok((" ", a("a")))
        );
    }

    #[test]
    fn annotation_some_parameters() {
        assert_eq!(
            annotation("@a( test, 123 ) "),
            Ok((" ", a_params("a", " test, 123 ")))
        );
    }

    #[test]
    fn annotation_parameters_incomplete() {
        assert!(annotation("@a(").is_err());
    }

    #[test]
    fn annotations0_no_annotations_does_not_remove_whitespace() {
        assert_eq!(
            annotations0(" \n"),
            Ok((" \n", vec![]))
        );
    }

    #[test]
    fn annotations0_no_annotations_retuns_empty_vec() {
        assert_eq!(
            annotations0(" class"),
            Ok((" class", vec![]))
        );
    }

    #[test]
    fn annotations0_one_annotation() {
        assert_eq!(
            annotations0("@a "),
            Ok(("", vec![a("a")]))
        );
    }

    #[test]
    fn annotations0_multiple_annotations() {
        assert_eq!(
            annotations0("@a@b@c "),
            Ok(("", vec![a("a"), a("b"), a("c")]))
        );
    }

    #[test]
    fn annotations0_start_with_whitespace() {
        assert_eq!(
            annotations0(" @a "),
            Ok(("", vec![a("a")]))
        );
    }

    #[test]
    fn parameters_string_whitespace_fails() {
        assert!(get_parameters_string("  ").is_err());
    }

    #[test]
    fn parameters_string_end_parenthesis_works() {
        assert_eq!(
            get_parameters_string(")"),
            Ok((")", ""))
        );
    }

    #[test]
    fn parameters_string_whitespace_and_end_parenthesis_works() {
        assert_eq!(
            get_parameters_string(" ) "),
            Ok((") ", " "))
        );
    }

    #[test]
    fn parameters_string_multiple_parenthesis() {
        assert_eq!(
            get_parameters_string(" ( ( ) ) ) "),
            Ok((") ", " ( ( ) ) "))
        );
    }

    #[test]
    fn parameters_string_parenthesis_in_string_literals_skipped() {
        assert_eq!(
            get_parameters_string("')' \")\" )"),
            Ok((")", "')' \")\" "))
        );
    }

    #[test]
    fn parameters_string_parenthesis_in_comments_skipped() {
        assert_eq!(
            get_parameters_string("( // )\n/*)*/))"),
            Ok((")", "( // )\n/*)*/)"))
        );
    }
}
