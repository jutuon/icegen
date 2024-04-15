
use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::{utils::indent_lines, ValidatedFile};

use super::ValidatedClass;

mod constructor;
mod fields;
mod to_string;
mod equality;
mod hash_code;
mod debug_fill_properties;
mod copy_with;

pub use copy_with::generate_detect_default_class_and_constant;

pub fn generate_impl_class(file: &ValidatedFile, class: &ValidatedClass) -> Result<String> {
    let abstract_class_name = format!("_{}", class.name);
    let class_modifier = if class.private_constructor_exists() {
        if file.flutter_foundation_import_exists {
            format!("extends {} with DiagnosticableTreeMixin", abstract_class_name)
        } else {
            format!("extends {}", abstract_class_name)
        }
    } else {
        if file.flutter_foundation_import_exists {
            format!("with DiagnosticableTreeMixin implements {}", abstract_class_name)
        } else {
            format!("implements {}", abstract_class_name)
        }
    };

    let debug_fill_properties = if file.flutter_foundation_import_exists {
        format!(
            "\n\n{}",
            indent_lines("  ", debug_fill_properties::generate_debug_fill_properties(file, class))
        )
    } else {
        "".to_string()
    };

    let impl_class = formatdoc!("
        /// @nodoc
        class _${}Impl {} {{
        {}

        {}

        {}{}

        {}

        {}

        {}
        }}",
        class.name,
        class_modifier,
        indent_lines("  ", constructor::generate_impl_class_constructor(class)?),
        indent_lines("  ", fields::generate_impl_class_overridden_fields(class)),
        indent_lines("  ", to_string::generate_impl_class_to_string(file, class)),
        debug_fill_properties,
        indent_lines("  ", equality::generate_impl_class_equality_operator(class)),
        indent_lines("  ", hash_code::generate_impl_class_hash_code(class)),
        indent_lines("  ", copy_with::generate_impl_class_copy_with(class)),
    );

    Ok(impl_class)
}
