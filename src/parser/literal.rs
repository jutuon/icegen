
use nom::{
    branch::alt, bytes::complete::{is_not, tag}, combinator::{opt}, sequence::{delimited}, IResult
};

pub const SINGLE_QUOTE: &str = "'";
pub const DOUBLE_QUOTE: &str = "\"";

// TODO: Support triple quotes """ and ''' for multiline strings.
//       Support raw strings r"string" and r'string'.

pub fn string_literal(input: &str) -> IResult<&str, String> {
    string_literal_str(input)
        .map(
            |(input, v)| (input, v.to_string())
        )
}

pub fn string_literal_str(input: &str) -> IResult<&str, &str> {
    alt((
        double_quote_string,
        single_quote_string,
    ))(input)
}

fn double_quote_string(input: &str) -> IResult<&str, &str> {
    quote_string(DOUBLE_QUOTE, is_not_newline_or_double_quote, input)
}

fn single_quote_string(input: &str) -> IResult<&str, &str> {
    quote_string(SINGLE_QUOTE, is_not_newline_or_single_quote, input)
}

fn quote_string<'a>(
    quote_character: &str,
    parse_until: impl Fn(&str) -> IResult<&str, &str> + 'a,
    input: &'a str
) -> IResult<&'a str, &'a str> {
    let (input, text) = delimited(
        tag(quote_character),
        opt(
            parse_until,
        ),
        tag(quote_character),
    )(input)?;
    Ok((input, text.unwrap_or_default()))
}

fn is_not_newline_or_single_quote(input: &str) -> IResult<&str, &str> {
    is_not("\n'")(input)
}

fn is_not_newline_or_double_quote(input: &str) -> IResult<&str, &str> {
    is_not("\n\"")(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_quote_string_works() {
        assert_eq!(
            double_quote_string("\"a\""),
            Ok(("", "a"))
        );
    }

    #[test]
    fn single_quote_string_empty_string() {
        assert!(single_quote_string("").is_err());
    }

    #[test]
    fn single_quote_string_empty_quote_string() {
        assert_eq!(
            single_quote_string("''"),
            Ok(("", ""))
        );
    }

    #[test]
    fn single_quote_string_newline_errors() {
        assert!(single_quote_string("\n").is_err());
    }

    #[test]
    fn single_quote_string_works() {
        assert_eq!(
            single_quote_string("'a'"),
            Ok(("", "a"))
        );
    }

    #[test]
    fn is_not_newline_or_single_quote_failure_with_newline() {
        assert!(is_not_newline_or_single_quote("\n").is_err());
    }

    #[test]
    fn is_not_newline_or_single_quote_failure_with_single_quote() {
        assert!(is_not_newline_or_single_quote("'").is_err());
    }

    #[test]
    fn is_not_newline_or_single_quote_success_with_allowed_character() {
        assert_eq!(
            is_not_newline_or_single_quote("a"),
            Ok(("", "a"))
        );
    }

    #[test]
    fn is_not_newline_or_double_quote_failure_with_newline() {
        assert!(is_not_newline_or_double_quote("\n").is_err());
    }

    #[test]
    fn is_not_newline_or_double_quote_failure_with_double_quote() {
        assert!(is_not_newline_or_double_quote("\"").is_err());
    }

    #[test]
    fn is_not_newline_or_double_quote_success_with_allowed_character() {
        assert_eq!(
            is_not_newline_or_double_quote("a"),
            Ok(("", "a"))
        );
    }
}
