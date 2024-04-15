use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::utils::indent_lines;

use super::ValidatedClass;

pub fn generate_abstract_class(class: &ValidatedClass) -> Result<String> {
    let class_modifier = if class.private_constructor_exists() {
        "extends"
    } else {
        "implements"
    };

    let private_constructor = if class.private_constructor_exists() {
        let const_keyword = if class.private_constructor_is_const() {
            "const "
        } else {
            ""
        };

        format!(
            "\n  {}_{}._() : super._();",
            const_keyword,
            class.name,
        )
    } else {
        "".to_string()
    };

    let abstract_class = formatdoc!("
        /// @nodoc
        abstract class _{} {} {} {{
        {}{}
        }}",
        class.name,
        class_modifier,
        class.name,
        indent_lines("  ", generate_abstract_class_factory(class)?),
        private_constructor,
    );

    Ok(abstract_class)
}

fn generate_abstract_class_factory(class: &ValidatedClass) -> Result<String> {
    let const_keyword = if class.factory_constructor_is_const() {
        "const "
    } else {
        ""
    };

    let factory = formatdoc!("
        {}factory _{}({{
        {}
        }}) = _${}Impl;",
        const_keyword,
        class.name,
        indent_lines("  ", generate_abstract_class_field_params(class)),
        class.name,
    );

    Ok(factory)
}

fn generate_abstract_class_field_params(class: &ValidatedClass) -> String {
    let mut field_getters = String::new();

    for field in class.factory_constructor_params() {
        let required = if field.required {
            "required "
        } else {
            ""
        };

        field_getters.push_str(&format!(
            "{}{} {},\n",
            required,
            field.parameter_type,
            field.name,
        ));
    }

    field_getters
}
