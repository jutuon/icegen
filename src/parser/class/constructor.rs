
use nom::{
    branch::alt, bytes::{complete::{is_not, tag, take_until, take_while}}, character::complete::{multispace0, multispace1}, combinator::{map, opt}, multi::many0, sequence::{delimited, pair, preceded, tuple}, IResult
};

use crate::parser::{identifier::{identifier, Identifier}, keyword::factory_keyword, whitespace::wsc};

use super::named_parameters::{named_parameters0, NamedParameter};

/// Constructor like `ClassName._();`
#[derive(Debug, PartialEq)]
pub struct PrivateConstructor;


pub fn private_constructor<'a>(class_name: &Identifier, input: &'a str) -> IResult<&'a str, PrivateConstructor> {
    let (input, _) = tag(class_name.name.as_bytes())(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag("_")(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input, PrivateConstructor))
}

#[derive(Debug, PartialEq)]
pub struct FactoryConstructor {
    pub params: Vec<NamedParameter>,
}


pub fn factory_constructor<'a>(class_name: &Identifier, input: &'a str) -> IResult<&'a str, FactoryConstructor> {
    let (input, _) = factory_keyword(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag(class_name.name.as_bytes())(input)?;
    let (input, _) = wsc(input)?;

    let (input, params) = delimited(
        tag("("),
        delimited(
            preceded(wsc, tag("{")),
            preceded(wsc, named_parameters0),
            preceded(tuple((wsc, tag("}"))), wsc),
        ),
        tag(")"),
    )(input)?;

    let (input, _) = wsc(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = identifier(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = tag(";")(input)?;


    Ok((input, FactoryConstructor {
        params,
    }))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{data_type::DataType, identifier::Identifier};

    fn identifier(name: &str) -> Identifier {
        Identifier { name: name.to_string() }
    }

    fn named_parameter(
        class_name: &str,
        name: &str
    ) -> NamedParameter {
        NamedParameter {
            annotations: vec![],
            required: false,
            parameter_type: DataType {
                name: identifier(class_name),
                nullable: false,
                type_args: vec![],
            },
            name: Identifier { name: name.to_string() },
        }
    }

    #[test]
    fn private_constructor_parsed_correctly() {
        assert_eq!(
            private_constructor(
                &identifier("A"),
                "A . _ ( ) ; ",
            ),
            Ok((" ", PrivateConstructor))
        );
    }


    #[test]
    fn factory_constructor_no_params() {
        assert_eq!(
            factory_constructor(
                &identifier("A"),
                "factory A ( {  } ) = _ ;"
            ),
            Ok((
                "",
                FactoryConstructor { params: vec![] }
            ))
        );
    }

    #[test]
    fn factory_constructor_one_parameter() {
        assert_eq!(
            factory_constructor(
                &identifier("A"),
                "factory A ( { B b } ) = _ ;"
            ),
            Ok((
                "",
                FactoryConstructor { params: vec![
                    named_parameter("B", "b"),
                ] }
            ))
        );
    }

    #[test]
    fn factory_constructor_two_parameters() {
        assert_eq!(
            factory_constructor(
                &identifier("A"),
                "factory A ({ B b, C c }) = _ ;"
            ),
            Ok((
                "",
                FactoryConstructor { params: vec![
                    named_parameter("B", "b"),
                    named_parameter("C", "c"),
                ] }
            ))
        );
    }

    #[test]
    fn factory_constructor_multiline() {
        assert_eq!(
            factory_constructor(
                &identifier("A"),
                "factory A({
                    B b  ,
                }) = _ ;"
            ),
            Ok((
                "",
                FactoryConstructor { params: vec![
                    named_parameter("B", "b"),
                ] }
            ))
        );
    }
}
