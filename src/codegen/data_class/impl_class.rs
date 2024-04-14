
use anyhow::Result;
use indoc::formatdoc;

use crate::codegen::utils::indent_lines;

use super::ValidatedClass;

mod constructor;
mod fields;
mod to_string;
mod equality;
mod hash_code;

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
        indent_lines("  ", constructor::generate_impl_class_constructor(class)?),
        indent_lines("  ", fields::generate_impl_class_overridden_fields(class)),
        indent_lines("  ", to_string::generate_impl_class_to_string(class)),
        indent_lines("  ", equality::generate_impl_class_equality_operator(class)),
        indent_lines("  ", hash_code::generate_impl_class_hash_code(class)),
    );

    Ok(impl_class)
}
