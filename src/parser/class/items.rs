
use nom::{
    branch::alt, bytes::complete::{is_not, tag, take_until, take_while}, character::complete::{multispace0, multispace1}, combinator::{map, opt}, multi::many0, sequence::{delimited, pair, preceded, tuple}, IResult, Parser
};

use crate::parser::{identifier::Identifier, utils::item_parser};

use super::constructor::{factory_constructor, private_constructor, FactoryConstructor, PrivateConstructor};

#[derive(Debug, PartialEq)]
pub struct ClassItemInfo {
    pub items: Vec<ClassItem>,
}

#[derive(Debug, PartialEq)]
pub enum ClassItem {
    PrivateConstructor(PrivateConstructor),
    FactoryConstructor(FactoryConstructor),
}

pub fn class_item_info<'a>(class_name: &Identifier, input: &'a str) -> IResult<&'a str, ClassItemInfo> {
    item_parser(
        |input| input.starts_with("}"),
        alt((
            (|input| private_constructor(class_name, input)).map(ClassItem::PrivateConstructor),
            (|input| factory_constructor(class_name, input)).map(ClassItem::FactoryConstructor),
        )),
        input,
    )
        .map(|(input, items)|
            (input, ClassItemInfo { items })
        )
}
