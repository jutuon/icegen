
use nom::{
    bytes::{complete::{tag}}, combinator::{opt}, sequence::{delimited, preceded, tuple}, IResult
};

use crate::parser::{identifier::{identifier, Identifier}, keyword::{const_keyword, factory_keyword}, whitespace::wsc};

use super::named_parameters::{named_parameters0, NamedParameter};

/// Constructor like `ClassName._();`
#[derive(Debug, PartialEq, Clone)]
pub struct PrivateConstructor {
    pub is_const: bool,
}


pub fn private_constructor<'a>(class_name: &Identifier, input: &'a str) -> IResult<&'a str, PrivateConstructor> {
    let (input, is_const) = opt(const_keyword)(input)?;
    let (input, _) = wsc(input)?;
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

    Ok((input, PrivateConstructor { is_const: is_const.is_some() }))
}

#[derive(Debug, PartialEq, Clone)]
pub struct FactoryConstructor {
    pub params: Vec<NamedParameter>,
    pub is_const: bool,
}


pub fn factory_constructor<'a>(class_name: &Identifier, input: &'a str) -> IResult<&'a str, FactoryConstructor> {
    let (input, is_const) = opt(const_keyword)(input)?;
    let (input, _) = wsc(input)?;
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
        is_const: is_const.is_some(),
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

    fn p(is_const: bool) -> PrivateConstructor {
        PrivateConstructor { is_const }
    }

    fn f(params: impl AsRef<[NamedParameter]>) -> FactoryConstructor {
        FactoryConstructor {
            params: params.as_ref().to_vec(),
            is_const: false,
        }
    }

    fn f_with_is_const(params: impl AsRef<[NamedParameter]>, is_const: bool) -> FactoryConstructor {
        FactoryConstructor {
            params: params.as_ref().to_vec(),
            is_const,
        }
    }

    #[test]
    fn private_constructor_parsed_correctly() {
        assert_eq!(
            private_constructor(
                &identifier("A"),
                "A . _ ( ) ; ",
            ),
            Ok((" ", p(false)))
        );
    }

    #[test]
    fn private_constructor_with_const_parsed_correctly() {
        assert_eq!(
            private_constructor(
                &identifier("A"),
                "const A . _ ( ) ; ",
            ),
            Ok((" ", p(true)))
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
                f([])
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
                f([named_parameter("B", "b"),])
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
                f([
                    named_parameter("B", "b"),
                    named_parameter("C", "c"),
                ])
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
                f([
                    named_parameter("B", "b"),
                ])
            ))
        );
    }

    #[test]
    fn factory_constructor_with_const() {
        assert_eq!(
            factory_constructor(
                &identifier("A"),
                "const factory A ( {  } ) = _ ;"
            ),
            Ok((
                "",
                f_with_is_const([], true)
            ))
        );
    }
}
