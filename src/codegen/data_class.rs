
pub mod mixin;
pub mod abstract_class;
pub mod impl_class;

use anyhow::{anyhow, Result};
use indoc::{formatdoc, indoc};

use crate::{codegen::wrong_constructor_exception::WRONG_CONSTRUCTOR_EXCEPTION_IDENTIFIER, file_finder::DartFile, parser::{ClassDefinition, ClassItem, FactoryConstructor, Identifier, NamedParameter, ParsedFile, TopLevelItems}};

pub struct ValidatedClass {
    name: Identifier,
    private_constructor_exists: bool,
    factory_constructor_params: Vec<NamedParameter>,
}

impl ValidatedClass {
    pub fn validate(class_info: &ClassDefinition) -> Result<ValidatedClass> {
        let mut private_constructor_exists = false;
        let mut factory_constructor_params: Option<Vec<NamedParameter>> = None;

        for item in &class_info.item_info.items {
            match item {
                ClassItem::FactoryConstructor(constructor) =>
                    if factory_constructor_params.is_some() {
                        return Err(anyhow!("Multiple factory constructors found for class {}", class_info.name));
                    } else {
                        factory_constructor_params = Some(constructor.params.clone());
                    }
                ClassItem::PrivateConstructor(_) =>
                    if private_constructor_exists {
                        return Err(anyhow!("Multiple private constructors found for class {}", class_info.name));
                    } else {
                        private_constructor_exists = true;
                    }
            }
        }

        let factory_constructor_params = factory_constructor_params
            .ok_or(anyhow!("No factory constructor found for class {}", class_info.name))?;

        let validated = ValidatedClass {
            name: class_info.name.clone(),
            private_constructor_exists,
            factory_constructor_params,
        };

        Ok(validated)
    }
}
