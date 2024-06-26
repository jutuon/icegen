use nom::{
    bytes::complete::{tag, take_until},
    IResult,
};

use super::{keyword::import_keyword, literal::string_literal, whitespace::wsc};

#[derive(Debug, PartialEq)]
pub struct ImportStatement {
    pub path: String,
}

impl ImportStatement {
    pub fn is_flutter_foundation_import(&self) -> bool {
        self.path == "package:flutter/foundation.dart"
    }
}

pub fn import_statement(input: &str) -> IResult<&str, ImportStatement> {
    let (input, _) = import_keyword(input)?;
    let (input, _) = wsc(input)?;
    let (input, path) = string_literal(input)?;
    let (input, _) = take_until(";")(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input, ImportStatement { path }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_statement_other_than_import_statement() {
        assert!(import_statement("a").is_err());
    }

    #[test]
    fn import_statement_parsed_correctly() {
        assert_eq!(
            import_statement("import // \n'hello' \n   ;a"),
            Ok((
                "a",
                ImportStatement {
                    path: "hello".to_string()
                }
            ))
        );
    }
}
