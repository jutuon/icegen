
pub mod constructor;
pub mod items;
pub mod named_parameters;

use nom::{
    branch::alt, bytes::complete::{is_not, tag, take, take_until, take_while}, character::complete::{multispace0, multispace1}, combinator::{map, opt}, multi::{count, many0}, sequence::{delimited, pair, preceded, tuple}, IResult
};

use self::items::{class_item_info, ClassItemInfo};

use super::{annotation::{annotations0, Annotation}, identifier::{identifier, Identifier}, keyword::{class_keyword, import_keyword, with_keyword}, literal::string_literal, utils::comma_separated1, whitespace::wsc};

#[derive(Debug, PartialEq)]
pub struct ClassDefinition {
    pub annotations: Vec<Annotation>,
    pub name: Identifier,
    pub mixin_types: Vec<Identifier>,
    pub item_info: ClassItemInfo,
}

impl ClassDefinition {
    pub fn contains_freezed_annotation(&self) -> bool {
        self.annotations.iter().any(|annotation| {
            annotation.is_freezed_annotation()
        })
    }
}

pub fn class(input: &str) -> IResult<&str, ClassDefinition> {
    let (input, annotations) = annotations0(input)?;
    let (input, _) = wsc(input)?;
    let (input, _) = class_keyword(input)?;
    let (input, _) = wsc(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = wsc(input)?;

    let (input, mixin_types) = parse_mixin_types(input)?;
    let (input, _) = wsc(input)?;

    let (input, item_info) = delimited(
        tag("{"),
        |input| class_item_info(&name, input),
        tag("}"),
    )(input)?;

    Ok((input, ClassDefinition { annotations, name, mixin_types, item_info }))
}

pub fn parse_mixin_types(input: &str) -> IResult<&str, Vec<Identifier>> {
    let (input, _) = wsc(input)?;
    let (input, with_detected) = opt(with_keyword)(input)?;
    if with_detected.is_some() {
        let (input, _) = wsc(input)?;
        let (input, identifiers) = comma_separated1(identifier, input)?;
        Ok((input, identifiers))
    } else {
        Ok((input, vec![]))
    }
}

#[cfg(test)]
mod tests {
    use tests::constructor::{FactoryConstructor, PrivateConstructor};

    use crate::parser::data_type::DataType;

    use self::{items::ClassItem, named_parameters::NamedParameter};

    use super::*;

    fn c(name: &str) -> ClassDefinition {
        ClassDefinition {
            annotations: vec![],
            name: Identifier { name: name.to_string() },
            mixin_types: vec![],
            item_info: ClassItemInfo {
                items: vec![],
            },
        }
    }

    fn annotated_c(annotations: &[&str], name: &str) -> ClassDefinition {
        let annotations = annotations.iter()
            .map(|a| Annotation {
                name: Identifier { name: a.to_string() },
                parameters: "".to_string(),
            }).collect();
        ClassDefinition {
            annotations,
            name: Identifier { name: name.to_string() },
            mixin_types: vec![],
            item_info: ClassItemInfo {
                items: vec![],
            },
        }
    }

    fn c_with_mixins(name: &str, with_types: &[&str]) -> ClassDefinition {
        let with_types = with_types.iter().map(|a| Identifier { name: a.to_string() } ).collect();
        ClassDefinition {
            annotations: vec![],
            name: Identifier { name: name.to_string() },
            mixin_types: with_types,
            item_info: ClassItemInfo {
                items: vec![],
            },
        }
    }

    fn c_with_items(name: &str, items: Vec<ClassItem>) -> ClassDefinition {
        ClassDefinition {
            annotations: vec![],
            name: Identifier { name: name.to_string() },
            mixin_types: vec![],
            item_info: ClassItemInfo {
                items,
            },
        }
    }

    fn named_parameter(
        class_name: &str,
        name: &str
    ) -> NamedParameter {
        NamedParameter {
            annotations: vec![],
            required: false,
            parameter_type: DataType {
                name: Identifier { name: class_name.to_string() },
                nullable: false,
                type_args: vec![],
            },
            name: Identifier { name: name.to_string() },
        }
    }

    fn factory_constructor(params: Vec<NamedParameter>) -> ClassItem {
        ClassItem::FactoryConstructor(
            FactoryConstructor { params, is_const: false }
        )
    }

    #[test]
    fn class_with_inner_scopes() {
        assert_eq!(
            class(
                "class A { { } }"
            ),
            // TOOD: Should class parsing be more strict?
            Ok((" }", c("A")))
        );
    }

    #[test]
    fn class_with_annotation() {
        assert_eq!(
            class(
                "@a class A {}"
            ),
            Ok(("", annotated_c(&["a"], "A")))
        );
    }

    #[test]
    fn class_with_multiple_annotations() {
        assert_eq!(
            class(
                "@a @b @c class A {}"
            ),
            Ok(("", annotated_c(&["a", "b", "c"], "A")))
        );
    }

    #[test]
    fn class_with_something_unsupported_after_class_name_makes_failure() {
        assert!(class("class A a {}").is_err());
    }

    #[test]
    fn class_and_with_keyword_and_no_type_makes_failure() {
        assert!(class("class A with {}").is_err());
    }

    #[test]
    fn class_and_with_keyword_and_one_type() {
        assert_eq!(
            class(
                "class A with B {}"
            ),
            Ok(("", c_with_mixins("A", &["B"])))
        );
    }

    #[test]
    fn class_and_with_keyword_and_two_types() {
        assert_eq!(
            class(
                "class A with B, C {}"
            ),
            Ok(("", c_with_mixins("A", &["B", "C"])))
        );
    }

    #[test]
    fn class_and_private_constructor() {
        assert_eq!(
            class(
                "class A { A._(); }"
            ),
            Ok((
                "",
                c_with_items(
                    "A",
                    vec![ClassItem::PrivateConstructor(PrivateConstructor { is_const: false })],
                )
            ))
        );
    }

    #[test]
    fn class_and_getter_and_private_constructor() {
        assert_eq!(
            class(
                "class A {
                    int get number => 1;
                    A._();
                }"
            ),
            Ok((
                "",
                c_with_items(
                    "A",
                    vec![
                        ClassItem::PrivateConstructor(PrivateConstructor { is_const: false }),
                    ],
                )
            ))
        );
    }

    #[test]
    fn class_and_factory_constructors() {
        assert_eq!(
            class(
                "class A {
                    factory A({
                        B b,
                    }) = _ ;
                }"
            ),
            Ok((
                "",
                c_with_items(
                    "A",
                    vec![
                        factory_constructor(
                            vec![
                                named_parameter("B", "b"),
                            ]
                        ),
                    ],
                )
            ))
        );
    }
}
