
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

pub fn generate_impl_class(file: &ValidatedFile, class: &ValidatedClass) -> Result<String> {
    let class_modifier = if class.private_constructor_exists {
        "extends"
    } else {
        "implements"
    };

    let diagnosticable_tree_mixin = if file.flutter_foundation_import_exists {
        " with DiagnosticableTreeMixin"
    } else {
        ""
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
        class _${}Impl {} _{}{} {{
        {}

        {}

        {}{}

        {}

        {}
        }}",
        class.name,
        class_modifier,
        class.name,
        diagnosticable_tree_mixin,
        indent_lines("  ", constructor::generate_impl_class_constructor(class)?),
        indent_lines("  ", fields::generate_impl_class_overridden_fields(class)),
        indent_lines("  ", to_string::generate_impl_class_to_string(file, class)),
        debug_fill_properties,
        indent_lines("  ", equality::generate_impl_class_equality_operator(class)),
        indent_lines("  ", hash_code::generate_impl_class_hash_code(class)),
    );

    Ok(impl_class)
}
