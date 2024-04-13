use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::utils::indent_lines;

use super::ValidatedClass;

pub fn generate_abstract_class(class: &ValidatedClass) -> Result<String> {
    let class_modifier = if class.private_constructor_exists {
        "extends"
    } else {
        "implements"
    };

    let private_constructor = if class.private_constructor_exists {
        format!(
            "  _{}._() : super._();\n",
            class.name,
        )
    } else {
        "".to_string()
    };

    let abstract_class = formatdoc!("
        /// @nodoc
        abstract class _{} {} {} {{
        {}
        {}

        {}
        }}",
        class.name,
        class_modifier,
        class.name,
        indent_lines("  ", generate_abstract_class_factory(class)?),
        private_constructor,
        indent_lines("  ", generate_abstract_class_overridden_items(class)),
    );

    Ok(abstract_class)
}

fn generate_abstract_class_factory(class: &ValidatedClass) -> Result<String> {
    let factory = formatdoc!("
        factory _{}({{
        {}
        }}) = _${}Impl;",
        class.name,
        indent_lines("  ", generate_abstract_class_field_params(class)),
        class.name,
    );

    Ok(factory)
}

pub fn generate_abstract_class_field_params(class: &ValidatedClass) -> String {
    let mut field_getters = String::new();

    for field in &class.factory_constructor_params {
        let required = if field.required {
            "required "
        } else {
            ""
        };

        field_getters.push_str(&format!(
            "{}{} {},\n",
            required,
            field.parameter_type.name,
            field.name,
        ));
    }

    field_getters
}

fn generate_abstract_class_overridden_items(class: &ValidatedClass) -> String {
    let mut items = String::new();

    for field in &class.factory_constructor_params {
        items.push_str(&formatdoc!("
            @override
            {} get {};\n",
            field.parameter_type.name,
            field.name,
        ));
    }

    items
}
