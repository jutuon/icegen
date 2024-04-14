
use core::fmt;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till, take_until, take_until1, take_while, take_while1},
    character::{complete::{alpha1, multispace0, multispace1}, is_alphabetic},
    combinator::{map, not, opt}, multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult
};

use super::{identifier::identifier, Identifier};

#[derive(Debug, PartialEq, Clone)]
pub struct DataType {
    pub(in crate::parser) name: Identifier,
    pub nullable: bool,
}

impl DataType {
    pub fn to_nullable(&self) -> DataType {
        DataType {
            name: self.name.clone(),
            nullable: true,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.nullable {
            write!(f, "{}?", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub fn data_type(input: &str) -> IResult<&str, DataType> {
    let (input, name) = identifier(input)?;
    let (input, nullable) = opt(tag("?"))(input)?;

    Ok((input, DataType { name, nullable: nullable.is_some() }))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dtype(name: &str, nullable: bool) -> DataType {
        DataType {
            name: Identifier { name: name.to_string() },
            nullable,
        }
    }

    #[test]
    fn data_type_non_nullable() {
        assert_eq!(
            data_type("Test "),
            Ok((" ", dtype("Test", false)))
        );
    }

    #[test]
    fn data_type_nullable() {
        assert_eq!(
            data_type("Test? "),
            Ok((" ", dtype("Test", true)))
        );
    }
}
