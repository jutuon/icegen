use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, utils::indent_lines};

pub fn generate_impl_class_constructor(class: &ValidatedClass) -> Result<String> {
    let const_keyword = if class.factory_constructor_is_const() {
        "const "
    } else {
        ""
    };

    let super_constructor_invocation = if class.private_constructor_exists() {
        " : super._()"
    } else {
        ""
    };

    let factory = formatdoc!(
        "
        {}{}_${}Impl({{
        {}
        }}){};",
        const_values_for_field_value_defaults(class),
        const_keyword,
        class.name,
        indent_lines("  ", generate_impl_class_field_params(class)),
        super_constructor_invocation,
    );

    Ok(factory)
}

fn generate_impl_class_field_params(class: &ValidatedClass) -> String {
    let mut field_getters = String::new();

    for field in class.factory_constructor_params() {
        let required = if field.required { "required " } else { "" };

        let default_value = if field.default_annotation().is_some() {
            format!(" = _{}DefaultValue", field.name)
        } else {
            "".to_string()
        };

        field_getters.push_str(&format!(
            "{}this.{}{},\n",
            required, field.name, default_value,
        ));
    }

    field_getters
}

fn const_values_for_field_value_defaults(class: &ValidatedClass) -> String {
    let mut code = String::new();

    for field in class.factory_constructor_params() {
        let default_value = if let Some(default) = field.default_annotation() {
            default
        } else {
            continue;
        };

        code.push_str(&format!(
            "static const {} _{}DefaultValue = {};\n",
            field.parameter_type, field.name, default_value,
        ));
    }

    if !code.is_empty() {
        code.push('\n');
    }

    code
}
