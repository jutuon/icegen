use anyhow::Result;
use indoc::formatdoc;

use crate::{parser::Identifier};

use super::ValidatedClass;

pub fn generate_mixin(class: &ValidatedClass) -> Result<String> {
    validate_identifier(&class.name)?;
    let exception_identifier = format!("_privateConstructorError{}", class.name);
    let variable_for_exception = formatdoc!("
        final {} = UnsupportedError(
            'Private constructor {}._() was called. Please call factory constructor instead.');",
        exception_identifier,
        class.name,
    );

    let mixin = formatdoc!("
        {}

        /// @nodoc
        mixin _${} {{
        {}
        }}",
        variable_for_exception,
        class.name,
        generate_field_getters_for_mixin(class, exception_identifier),
    );

    Ok(mixin)
}

fn generate_field_getters_for_mixin(class: &ValidatedClass, exception_identifier: String) -> String {
    let mut field_getters = String::new();

    for field in &class.factory_constructor_params {
        field_getters.push_str(&format!(
            "  {} get {} => throw {};\n",
            field.parameter_type,
            field.name,
            exception_identifier,
        ));
    }

    field_getters
}

fn validate_identifier(name: &Identifier) -> Result<()> {
    // Prevent formatting using class name
    if name.as_str().contains('$') {
        return Err(anyhow::anyhow!(
            "Class name {} contains a dollar sign, which is not supported",
            name
        ));
    }

    Ok(())
}
