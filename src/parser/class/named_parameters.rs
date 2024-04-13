
use nom::{
    branch::alt, bytes::complete::{is_not, tag, take, take_until, take_while}, character::complete::{multispace0, multispace1}, combinator::{map, not, opt}, multi::many0, sequence::{delimited, pair, preceded, tuple}, IResult
};

use crate::parser::{annotation::{annotations0, Annotation}, identifier::{identifier, Identifier}, keyword::required_keyword, utils::comma_separated0, whitespace::wsc};

#[derive(Debug, PartialEq)]
pub struct NamedParameter {
    pub annotations: Vec<Annotation>,
    pub required: bool,
    pub parameter_type: Identifier,
    pub name: Identifier,
}

pub fn named_parameter<'a>(input: &'a str) -> IResult<&'a str, NamedParameter> {
    let (input, annotations) = annotations0(input)?;
    let (input, _) = wsc(input)?;
    let (input, required) = opt(required_keyword)(input)?;
    let (input, _) = wsc(input)?;
    let (input, parameter_type) = identifier(input)?;
    let (input, _) = wsc(input)?;
    let (input, name) = identifier(input)?;

    Ok((input, NamedParameter {
        annotations,
        required: required.is_some(),
        parameter_type,
        name,
    }))
}

pub fn named_parameters0(input: &str) -> IResult<&str, Vec<NamedParameter>> {
    let (input, params) = comma_separated0(named_parameter, input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = opt(tag(","))(input)?;
    Ok((input, params))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::identifier::Identifier;

    fn a(name: &str) -> Annotation {
        Annotation {
            name: Identifier { name: name.to_string() },
            parameters: "".to_string(),
        }
    }

    fn c_name(name: &str) -> Identifier {
        Identifier { name: name.to_string() }
    }

    fn parameter(class_name: &str, name: &str) -> NamedParameter {
        NamedParameter {
            annotations: vec![],
            required: false,
            parameter_type: c_name(class_name),
            name: Identifier { name: name.to_string() },
        }
    }

    #[test]
    fn named_parameter_parsed_correctly() {
        assert_eq!(
            named_parameter("@a @b @c  required A a"),
            Ok((
                "",
                NamedParameter {
                    annotations: vec![a("a"), a("b"), a("c")],
                    required: true,
                    parameter_type: c_name("A"),
                    name: Identifier { name: "a".to_string() },
                }
            ))
        );
    }

    #[test]
    fn list_of_named_paramters_prevents_comma_in_beginning() {
        assert!(named_parameters0("  , A a").is_err())
    }

    #[test]
    fn empty_parameter_list() {
        assert_eq!(
            named_parameters0(" "),
            Ok((
                "",
                vec![]
            ))
        );
    }

    #[test]
    fn one_paramters_allow_comma_in_end() {
        assert_eq!(
            named_parameters0("A a, "),
            Ok((
                " ",
                vec![
                    parameter("A", "a"),
                ]
            ))
        );
    }

    #[test]
    fn one_paramters_trailing_comma_consumed_even_if_whitespace_before_it() {
        assert_eq!(
            named_parameters0("A a   , "),
            Ok((
                " ",
                vec![
                    parameter("A", "a"),
                ]
            ))
        );
    }

    #[test]
    fn one_named_paramters_no_comma_in_end_works() {
        assert_eq!(
            named_parameters0("A a"),
            Ok((
                "",
                vec![
                    parameter("A", "a"),
                ]
            ))
        );
    }

    #[test]
    fn two_named_paramters_allow_comma_in_end() {
        assert_eq!(
            named_parameters0("A a, B b, "),
            Ok((
                " ",
                vec![
                    parameter("A", "a"),
                    parameter("B", "b"),
                ]
            ))
        );
    }

    #[test]
    fn two_named_paramters_no_comma_in_end_works() {
        assert_eq!(
            named_parameters0("A a, B b"),
            Ok((
                "",
                vec![
                    parameter("A", "a"),
                    parameter("B", "b"),
                ]
            ))
        );
    }
}
