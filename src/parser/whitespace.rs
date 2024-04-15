use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    multi::many0,
    sequence::delimited,
    IResult,
};

pub const LINE_COMMENT_START: &str = "//";
pub const LINE_COMMENT_END: &str = "\n";
pub const MULTI_LINE_COMMENT_START: &str = "/*";
pub const MULTI_LINE_COMMENT_END: &str = "*/";

/// Remove whitespace and comments from the input
pub fn wsc(input: &str) -> IResult<&str, &str> {
    let (input, _) = many0(alt((ws, line_comment, multi_line_comment)))(input)?;
    Ok((input, ""))
}

/// Remove whitespace and comments partially.
/// There still might be some whitespace left after running this parser.
pub fn some_whitespace(input: &str) -> IResult<&str, &str> {
    let (input, consumed) = alt((ws, line_comment, multi_line_comment))(input)?;
    Ok((input, consumed))
}

fn ws(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}

fn line_comment(input: &str) -> IResult<&str, &str> {
    delimited(
        tag(LINE_COMMENT_START),
        take_until(LINE_COMMENT_END),
        tag(LINE_COMMENT_END),
    )(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, &str> {
    delimited(
        tag(MULTI_LINE_COMMENT_START),
        take_until(MULTI_LINE_COMMENT_END),
        tag(MULTI_LINE_COMMENT_END),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_no_ws() {
        assert!(ws("").is_err());
    }

    #[test]
    fn ws_only_ws() {
        let input = " \n   \n ";
        assert_eq!(ws(input), Ok(("", input)));
    }

    #[test]
    fn ws_removes_whitespace() {
        assert_eq!(ws(" \n   \na"), Ok(("a", " \n   \n")));
    }

    #[test]
    fn line_comment_removed() {
        assert_eq!(line_comment("// comment\na"), Ok(("a", " comment")));
    }

    #[test]
    fn multi_line_comment_removed() {
        assert_eq!(multi_line_comment("/* \n \n */ a"), Ok((" a", " \n \n ")));
    }

    #[test]
    fn wsc_whitespace_and_multiple_comments() {
        assert_eq!(wsc("\n //a\n /*b*/\nc"), Ok(("c", "")));
    }

    #[test]
    fn wsc_no_whitespace_or_comments() {
        assert_eq!(wsc("a// comment\n"), Ok(("a// comment\n", "")));
    }
}
