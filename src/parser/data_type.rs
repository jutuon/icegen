
use core::fmt;

use nom::{
    bytes::complete::{tag},
    combinator::{opt},
    IResult
};

use super::{identifier::identifier, utils::comma_separated1, whitespace::wsc, Identifier};

#[derive(Debug, PartialEq, Clone)]
pub struct DataType {
    pub(in crate::parser) name: Identifier,
    pub nullable: bool,
    pub type_args: Vec<DataType>,
}

impl DataType {
    pub fn to_nullable(&self) -> DataType {
        DataType {
            name: self.name.clone(),
            nullable: true,
            type_args: self.type_args.clone(),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let generics = if self.type_args.is_empty() {
            "".to_string()
        } else {
            format!("<{}>", self.type_args.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "))
        };

        if self.nullable {
            write!(f, "{}{}?", self.name, generics)
        } else {
            write!(f, "{}{}", self.name, generics)
        }
    }
}

pub fn data_type(input: &str) -> IResult<&str, DataType> {
    let (input, _) = wsc(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = wsc(input)?;
    let (input, type_args) = opt(generics)(input)?;
    let (input, _) = wsc(input)?;
    let (input, nullable) = opt(tag("?"))(input)?;
    let (input, _) = wsc(input)?;

    Ok((
        input,
        DataType {
            name,
            nullable: nullable.is_some(),
            type_args: type_args.unwrap_or_default(),
        }
    ))
}

pub fn generics(input: &str) -> IResult<&str, Vec<DataType>> {
    let (input, _) = tag("<")(input)?;
    let (input, _) = wsc(input)?;
    let (input, types) = comma_separated1(data_type, input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag(">")(input)?;

    Ok((input, types))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dtype(name: &str, nullable: bool, type_args: impl AsRef<[DataType]>) -> DataType {
        DataType {
            name: Identifier { name: name.to_string() },
            nullable,
            type_args: type_args.as_ref().to_vec(),
        }
    }

    #[test]
    fn data_type_non_nullable() {
        assert_eq!(
            data_type("Test "),
            Ok(("", dtype("Test", false, [])))
        );
    }

    #[test]
    fn data_type_nullable() {
        assert_eq!(
            data_type("Test? "),
            Ok(("", dtype("Test", true, [])))
        );
    }

    #[test]
    fn data_type_single_arg() {
        let arg1 = dtype("A", false, []);
        assert_eq!(
            data_type("Test < A > "),
            Ok(("", dtype("Test", false, [arg1])))
        );
    }

    #[test]
    fn data_type_multiple_type_args() {
        let arg1 = dtype("A", false, []);
        let arg2 = dtype("B", false, []);
        assert_eq!(
            data_type("Test < A , B > "),
            Ok(("", dtype("Test", false, [arg1, arg2])))
        );
    }

    #[test]
    fn data_type_multiple_type_args_where_args_also_have_type_args() {
        let a1 = dtype("A1", false, []);
        let a2 = dtype("A2", false, []);

        let b1 = dtype("B1", false, []);
        let b2 = dtype("B2", false, []);

        let arg1 = dtype("A", false, [a1, a2]);
        let arg2 = dtype("B", false, [b1, b2]);

        assert_eq!(
            data_type("Test < A < A1, A2 > , B < B1, B2 > > "),
            Ok(("", dtype("Test", false, [arg1, arg2])))
        );
    }
}
