use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, ValidatedFile};

pub fn generate_impl_class_to_string(file: &ValidatedFile, class: &ValidatedClass) -> String {
    let mut fields = String::new();
    for field in class.factory_constructor_params() {
        fields.push_str(&format!(
            "{}: ${}, ",
            field.name,
            field.name,
        ));
    }
    fields.pop();
    fields.pop();

    let to_string_params = if file.flutter_foundation_import_exists {
        "{DiagnosticLevel minLevel = DiagnosticLevel.info}"
    } else {
        ""
    };

    let hash_code = formatdoc!("
        @override
        String toString({}) {{
          return '{}({})';
        }}",
        to_string_params,
        class.name,
        fields,
    );

    hash_code
}
