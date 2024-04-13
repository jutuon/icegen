use crate::parser::Identifier;

const WRONG_CONSTRUCTOR_EXCEPTION_MESSAGE: &str = "A class was instantiated with a private constructor. Please instantiate the class with a factory constructor instead.";
pub const WRONG_CONSTRUCTOR_EXCEPTION_IDENTIFIER: &str = "_privateConstructorCalledException";

pub fn generate_final_variable_for_exception() -> String {
    format!(
        "final {} = Exception('{}');",
        WRONG_CONSTRUCTOR_EXCEPTION_IDENTIFIER,
        WRONG_CONSTRUCTOR_EXCEPTION_MESSAGE,
    )
}
