
pub mod mixin;
pub mod abstract_class;
pub mod impl_class;

use anyhow::{anyhow, Result};


use crate::{parser::{ClassDefinition, ClassItem, FactoryConstructor, Identifier, NamedParameter, PrivateConstructor}};

pub struct ValidatedClass {
    name: Identifier,
    private_constructor: Option<PrivateConstructor>,
    factory_constructor: FactoryConstructor,
}

impl ValidatedClass {
    pub fn private_constructor_exists(&self) -> bool {
        self.private_constructor.is_some()
    }

    pub fn private_constructor_is_const(&self) -> bool {
        self.private_constructor.as_ref().map(|v| v.is_const).unwrap_or_default()
    }

    pub fn factory_constructor_params(&self) -> &[NamedParameter] {
        &self.factory_constructor.params
    }

    pub fn factory_constructor_is_const(&self) -> bool {
        self.factory_constructor.is_const
    }

    pub fn nullable_named_parameter_exists(&self) -> bool {
        self.factory_constructor.params.iter().any(|param| param.parameter_type.nullable)
    }
}

impl ValidatedClass {
    pub fn validate(class_info: &ClassDefinition) -> Result<ValidatedClass> {
        let mut private_constructor: Option<PrivateConstructor> = None;
        let mut factory_constructor: Option<FactoryConstructor> = None;

        for item in &class_info.item_info.items {
            match item {
                ClassItem::FactoryConstructor(constructor) => {
                    if factory_constructor.is_some() {
                        return Err(anyhow!("Multiple factory constructors found for class {}", class_info.name));
                    } else {
                        factory_constructor = Some(constructor.clone());
                    }

                    Self::validate_factory_constructor(constructor, class_info)?;
                }
                ClassItem::PrivateConstructor( constructor ) =>
                    if private_constructor.is_some() {
                        return Err(anyhow!("Multiple private constructors found for class {}", class_info.name));
                    } else {
                        private_constructor = Some(constructor.clone());
                    }
            }
        }

        let factory_constructor = factory_constructor
            .ok_or(anyhow!("No factory constructor found for class {}", class_info.name))?;

        let validated = ValidatedClass {
            name: class_info.name.clone(),
            private_constructor,
            factory_constructor,
        };

        Ok(validated)
    }

    fn validate_factory_constructor(constructor: &FactoryConstructor, class_info: &ClassDefinition) -> Result<()> {
        if constructor.params.is_empty() {
            return Err(anyhow!(
                "Factory constructor in class {} has no named parameters",
                class_info.name
            ));
        }

        for param in &constructor.params {
            let mut default_annotation_found = false;
            for annotation in &param.annotations {
                if !annotation.is_default_annotation() {
                    continue;
                }
                if default_annotation_found {
                    return Err(anyhow!(
                        "Multiple @Default annotations found for parameter {} in class {}",
                        param.name,
                        class_info.name
                    ));
                }
                default_annotation_found = true;
            }
        }

        Ok(())
    }
}
