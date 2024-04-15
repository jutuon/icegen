use indoc::formatdoc;

use crate::codegen::{data_class::ValidatedClass, utils::indent_lines};

pub fn generate_impl_class_hash_code(class: &ValidatedClass) -> String {
    let mut fields = String::new();

    for field in class.factory_constructor_params() {
        fields.push_str(&format!("{},\n", field.name,));
    }

    let hash_code = formatdoc!(
        "
        @override
        int get hashCode => Object.hash(
          runtimeType,
        {}
        );",
        indent_lines("  ", fields),
    );

    hash_code
}
