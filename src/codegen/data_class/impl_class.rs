
use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::utils::indent_lines;

use super::{abstract_class::generate_abstract_class_field_params, ValidatedClass};

pub fn generate_impl_class(class: &ValidatedClass) -> Result<String> {
    let class_modifier = if class.private_constructor_exists {
        "extends"
    } else {
        "implements"
    };

    let impl_class = formatdoc!("
        /// @nodoc
        class _${}Impl {} _{} {{
        {}

        {}

        {}

        {}

        {}
        }}",
        class.name,
        class_modifier,
        class.name,
        indent_lines("  ", generate_impl_class_constructor(class)?),
        indent_lines("  ", generate_impl_class_overridden_items(class)),
        indent_lines("  ", generate_impl_class_to_string(class)),
        indent_lines("  ", generate_impl_class_equality_operator(class)),
        indent_lines("  ", generate_impl_class_hash_code(class)),
    );

    Ok(impl_class)
}

fn generate_impl_class_constructor(class: &ValidatedClass) -> Result<String> {
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

pub fn generate_impl_class_field_params(class: &ValidatedClass) -> String {
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

fn generate_impl_class_overridden_items(class: &ValidatedClass) -> String {
    let mut items = String::new();

    for field in &class.factory_constructor_params {
        items.push_str(&formatdoc!("
            @override
            final {} {};\n",
            field.parameter_type,
            field.name,
        ));
    }

    items
}

fn generate_impl_class_to_string(class: &ValidatedClass) -> String {
    let mut fields = String::new();

    for field in class.factory_constructor_params.iter() {
        fields.push_str(&format!(
            "{}: ${},",
            field.name,
            field.name,
        ));
    }
    fields.pop();

    let hash_code = formatdoc!("
        @override
        String toString() {{
          return '{}({})';
        }}",
        class.name,
        fields,
    );

    hash_code
}

fn generate_impl_class_equality_operator(class: &ValidatedClass) -> String {
    let mut equality_checks = String::new();

    for (i, field) in class.factory_constructor_params.iter().enumerate() {
        let field_count = class.factory_constructor_params.len();
        let and_operator = if i == field_count - 1 {
            ""
        } else {
            " &&"
        };
        equality_checks.push_str(&formatdoc!("
            (identical(other.{}, {}) ||
              other.{} == {}){}\n",
            field.name,
            field.name,
            field.name,
            field.name,
            and_operator,
        ));
    }

    let equality = formatdoc!("
        @override
        bool operator ==(Object other) {{
          return identical(this, other) ||
            (other is _${}Impl &&
        {}
          );
        }}",
        class.name,
        indent_lines("    ", equality_checks),
    );

    equality
}

fn generate_impl_class_hash_code(class: &ValidatedClass) -> String {
    let mut fields = String::new();

    for field in class.factory_constructor_params.iter() {
        fields.push_str(&format!(
            "{},\n",
            field.name,
        ));
    }

    let hash_code = formatdoc!("
        @override
        int get hashCode => Object.hash(
          runtimeType,
        {}
        );",
        indent_lines("  ", fields),
    );

    hash_code
}
