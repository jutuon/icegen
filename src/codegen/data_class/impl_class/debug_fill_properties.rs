use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, utils::indent_lines, ValidatedFile};

pub fn generate_debug_fill_properties(file: &ValidatedFile, class: &ValidatedClass) -> String {
    if !file.flutter_foundation_import_exists {
        return "".to_string();
    }

    let mut fields = String::new();
    for field in class.factory_constructor_params() {
        fields.push_str(&format!(
            "..add(DiagnosticsProperty('{}', {}))\n",
            field.name, field.name,
        ));
    }
    fields.pop();

    let function = formatdoc!(
        "
        @override
        void debugFillProperties(DiagnosticPropertiesBuilder properties) {{
          super.debugFillProperties(properties);
          properties
            ..add(DiagnosticsProperty('type', '{}'))
        {};
        }}",
        class.name,
        indent_lines("    ", fields),
    );

    function
}
