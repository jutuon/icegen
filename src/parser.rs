//! Parse Dart files

use anyhow::Result;
use nom::{branch::alt, IResult, Parser};

use self::{class::class, import::import_statement, utils::item_parser};

mod annotation;
mod class;
mod data_type;
mod identifier;
mod import;
mod keyword;
mod literal;
mod utils;
mod whitespace;

pub use self::annotation::Annotation;
pub use self::class::constructor::FactoryConstructor;
pub use self::class::constructor::PrivateConstructor;
pub use self::class::items::ClassItem;
pub use self::class::items::ClassItemInfo;
pub use self::class::named_parameters::NamedParameter;
pub use self::class::ClassDefinition;
pub use self::identifier::Identifier;
pub use self::import::ImportStatement;

#[derive(Debug, PartialEq)]
pub enum TopLevelItems {
    Import(ImportStatement),
    Class(ClassDefinition),
}

#[derive(Debug, PartialEq)]
pub struct ParsedFile {
    pub items: Vec<TopLevelItems>,
}

impl ParsedFile {
    pub fn contains_freezed_annotated_class(&self) -> bool {
        self.items.iter().any(|item| {
            if let TopLevelItems::Class(class) = item {
                class.contains_freezed_annotation()
            } else {
                false
            }
        })
    }
}

impl ParsedFile {
    pub fn parse_dart_file(input: &str) -> Result<ParsedFile> {
        let (_, parsed_file) = parse_file_contents(input).map_err(|e| e.to_owned())?;
        Ok(parsed_file)
    }
}

pub fn parse_file_contents(input: &str) -> IResult<&str, ParsedFile> {
    item_parser(
        |input| input.is_empty(),
        alt((
            import_statement.map(TopLevelItems::Import),
            class.map(TopLevelItems::Class),
        )),
        input,
    )
    .map(|(input, items)| (input, ParsedFile { items }))
}

#[cfg(test)]
mod tests {
    use self::{class::items::ClassItemInfo, identifier::Identifier};

    use super::*;

    fn i(path: &str) -> TopLevelItems {
        TopLevelItems::Import(ImportStatement {
            path: path.to_string(),
        })
    }

    fn c(name: &str) -> TopLevelItems {
        TopLevelItems::Class(ClassDefinition {
            annotations: vec![],
            name: Identifier {
                name: name.to_string(),
            },
            mixin_types: vec![],
            item_info: ClassItemInfo { items: vec![] },
        })
    }

    #[test]
    fn empty_file() {
        assert_eq!(
            parse_file_contents(""),
            Ok(("", ParsedFile { items: vec![] }))
        );
    }

    #[test]
    fn multiple_classes_and_imports_and_unknown_content() {
        let wanted = ParsedFile {
            items: vec![i("a"), i("b"), c("B"), c("C")],
        };

        assert_eq!(
            parse_file_contents(
                "

            import 'a';
            import 'b';

            test//class A {}
            class B {}
            sealed class C {}

            "
            ),
            Ok(("", wanted))
        );
    }
}
