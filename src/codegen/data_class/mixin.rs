use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::wrong_constructor_exception::WRONG_CONSTRUCTOR_EXCEPTION_IDENTIFIER;

use super::ValidatedClass;

fn generate_field_getters_for_mixin(class: &ValidatedClass) -> String {
    let mut field_getters = String::new();

    for field in &class.factory_constructor_params {
        field_getters.push_str(&format!(
            "  {} get {} => throw {};\n",
            field.parameter_type,
            field.name,
            WRONG_CONSTRUCTOR_EXCEPTION_IDENTIFIER,
        ));
    }

    field_getters
}

pub fn generate_mixin(class: &ValidatedClass) -> Result<String> {
    let mixin = formatdoc!("
        /// @nodoc
        mixin _${} {{
        {}
        }}",
        class.name,
        generate_field_getters_for_mixin(class),
    );

    Ok(mixin)
}
