use core::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::alpha1,
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub(in crate::parser) name: String,
}

impl Identifier {
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn identifier(input: &str) -> IResult<&str, Identifier> {
    // Check first character
    let _ = alt((alpha1, tag("_"), tag("$")))(input)?;
    let (input, identifier) = take_while(is_valid_identifier)(input)?;

    Ok((
        input,
        Identifier {
            name: identifier.to_string(),
        },
    ))
}

fn is_valid_identifier(input: char) -> bool {
    input.is_alphanumeric() || input == '_' || input == '$'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_empty_fails() {
        assert!(identifier("").is_err());
    }

    #[test]
    fn identifier_begins_with_number_fails() {
        assert!(identifier("1test").is_err());
    }

    #[test]
    fn identifier_ends_with_parenthesis() {
        assert_eq!(
            identifier("test$_()"),
            Ok((
                "()",
                Identifier {
                    name: "test$_".to_string()
                }
            ))
        );
    }

    #[test]
    fn identifier_ends_with_comma() {
        assert_eq!(
            identifier("test$_,"),
            Ok((
                ",",
                Identifier {
                    name: "test$_".to_string()
                }
            ))
        );
    }

    #[test]
    fn identifier_input_continues() {
        assert_eq!(
            identifier("test$_ a"),
            Ok((
                " a",
                Identifier {
                    name: "test$_".to_string()
                }
            ))
        );
    }

    #[test]
    fn identifier_input_ends() {
        assert_eq!(
            identifier("test$_"),
            Ok((
                "",
                Identifier {
                    name: "test$_".to_string()
                }
            ))
        );
    }

    #[test]
    fn identifier_first_character_underscore() {
        assert_eq!(
            identifier("_a"),
            Ok((
                "",
                Identifier {
                    name: "_a".to_string()
                }
            ))
        );
    }

    #[test]
    fn identifier_first_character_dollar() {
        assert_eq!(
            identifier("$a"),
            Ok((
                "",
                Identifier {
                    name: "$a".to_string()
                }
            ))
        );
    }
}
