use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, utils::indent_lines};

const DEFAULT_DETECTOR_VARIABLE: &str = "_detectDefaultValueInCopyWith";

pub fn generate_detect_default_class_and_constant() -> String {
    formatdoc!("
        class _DetectDefaultValueInCopyWith {{
          const _DetectDefaultValueInCopyWith();
        }}
        const _DetectDefaultValueInCopyWith {} = _DetectDefaultValueInCopyWith();",
        DEFAULT_DETECTOR_VARIABLE,
    )
}

pub fn generate_impl_class_copy_with(class: &ValidatedClass) -> String {
    formatdoc!("
        @override
        {} copyWith({{
        {}
        }}) => _${}Impl(
        {}
        );",
        class.name,
        indent_lines("  ", generate_field_params(class)),
        class.name,
        indent_lines("  ", generate_field_args(class)),
    )
}

fn generate_field_params(class: &ValidatedClass) -> String {
    let mut fields = String::new();

    for field in class.factory_constructor_params() {
        let default_value = if field.parameter_type.nullable {
            format!(" = {}", DEFAULT_DETECTOR_VARIABLE)
        } else {
            format!("")
        };

        fields.push_str(&format!(
            "Object? {}{},\n",
            field.name,
            default_value,
        ));
    }

    fields
}

fn generate_field_args(class: &ValidatedClass) -> String {
    let mut fields = String::new();

    for field in class.factory_constructor_params() {
        let arg = if field.parameter_type.nullable {
            format!(
                "{}: ({} == {} ? this.{} : {}) as {},\n",
                field.name,
                field.name,
                DEFAULT_DETECTOR_VARIABLE,
                field.name,
                field.name,
                field.parameter_type,
            )
        } else {
            format!(
                "{}: ({} ?? this.{}) as {},\n",
                field.name,
                field.name,
                field.name,
                field.parameter_type,
            )
        };

        fields.push_str(&arg);
    }

    fields
}
