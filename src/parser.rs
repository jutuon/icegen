//! Parse Dart files

use nom::{branch::alt, bytes::complete::take, multi::many0, IResult, Parser};
use anyhow::Result;

use self::{class::{class, ClassDefinition}, import::{import_statement, ImportStatement}, utils::item_parser, whitespace::wsc};

mod whitespace;
mod keyword;
mod literal;
mod identifier;
mod annotation;
mod import;
mod class;
mod utils;

#[derive(Debug, PartialEq)]
enum TopLevelItems {
    Import(ImportStatement),
    Class(ClassDefinition),
}

#[derive(Debug, PartialEq)]
pub struct ParsedFile {
    items: Vec<TopLevelItems>,
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
        .map(|(input, items)|
            (input, ParsedFile { items })
        )
}

#[cfg(test)]
mod tests {
    use self::{annotation::Annotation, class::items::ClassItemInfo, identifier::Identifier};

    use super::*;

    fn i(path: &str) -> TopLevelItems {
        TopLevelItems::Import(ImportStatement {
            path: path.to_string(),
        })
    }

    fn c(name: &str) -> TopLevelItems {
        TopLevelItems::Class(ClassDefinition {
            annotations: vec![],
            name: Identifier { name: name.to_string() },
            mixin_types: vec![],
            item_info: ClassItemInfo {
                items: vec![],
            },
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
            items: vec![
                i("a"),
                i("b"),
                c("B"),
                c("C"),
            ]
        };

        assert_eq!(
            parse_file_contents("

            import 'a';
            import 'b';

            test//class A {}
            class B {}
            sealed class C {}

            "),
            Ok(("", wanted))
        );
    }
}
