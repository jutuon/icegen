use indoc::formatdoc;

use crate::codegen::data_class::ValidatedClass;

pub fn generate_impl_class_overridden_fields(class: &ValidatedClass) -> String {
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
