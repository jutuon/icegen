use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, utils::indent_lines};


pub fn generate_impl_class_equality_operator(class: &ValidatedClass) -> String {
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
            (other.runtimeType == runtimeType &&
              other is _${}Impl &&
        {}
          );
        }}",
        class.name,
        indent_lines("      ", equality_checks),
    );

    equality
}
