

use nom::{
    branch::alt, bytes::complete::{tag, take_until}, character::complete::{multispace0, multispace1}, combinator::{map, opt}, multi::many0, sequence::{delimited, pair, preceded, tuple}, IResult
};

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Import,
    Class,
    Factory,
    Required,
    With,
}

pub fn keyword(input: &str) -> IResult<&str, Keyword> {
    alt((
        import_keyword,
        class_keyword,
        factory_keyword,
    ))(input)
}

// TODO: Keyword which ends in a comment does not work

pub fn import_keyword(input: &str) -> IResult<&str, Keyword> {
    tag("import ")(input).map(|(input, _)| (input, Keyword::Import))
}

pub fn class_keyword(input: &str) -> IResult<&str, Keyword> {
    tag("class ")(input).map(|(input, _)| (input, Keyword::Class))
}

pub fn factory_keyword(input: &str) -> IResult<&str, Keyword> {
    tag("factory ")(input).map(|(input, _)| (input, Keyword::Factory))
}

pub fn required_keyword(input: &str) -> IResult<&str, Keyword> {
    tag("required ")(input).map(|(input, _)| (input, Keyword::Required))
}

pub fn with_keyword(input: &str) -> IResult<&str, Keyword> {
    tag("with ")(input).map(|(input, _)| (input, Keyword::With))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_keyword_empty() {
        assert!(import_keyword("").is_err());
    }

    #[test]
    fn import_keyword_parses_correctly() {
        assert_eq!(
            import_keyword("import a"),
            Ok(("a", Keyword::Import))
        );
    }

    #[test]
    fn class_keyword_empty() {
        assert!(class_keyword("").is_err());
    }

    #[test]
    fn class_keyword_parses_correctly() {
        assert_eq!(
            class_keyword("class a"),
            Ok(("a", Keyword::Class))
        );
    }
}
