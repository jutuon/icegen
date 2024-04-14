use indoc::formatdoc;

use crate::codegen::data_class::ValidatedClass;

pub fn generate_impl_class_to_string(class: &ValidatedClass) -> String {
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
