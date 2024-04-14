use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, utils::indent_lines};

pub fn generate_impl_class_constructor(class: &ValidatedClass) -> Result<String> {
    let super_constructor_invocation = if class.private_constructor_exists {
        " : super._()"
    } else {
        ""
    };

    let factory = formatdoc!("
        _${}Impl({{
        {}
        }}){};",
        class.name,
        indent_lines("  ", generate_impl_class_field_params(class)),
        super_constructor_invocation,
    );

    Ok(factory)
}

fn generate_impl_class_field_params(class: &ValidatedClass) -> String {
    let mut field_getters = String::new();

    for field in &class.factory_constructor_params {
        let required = if field.required {
            "required "
        } else {
            ""
        };

        let default_value = if let Some(default) = field.default_annotation() {
            format!(" = {}", default)
        } else {
            "".to_string()
        };

        field_getters.push_str(&format!(
            "{}this.{}{},\n",
            required,
            field.name,
            default_value,
        ));
    }

    field_getters
}
