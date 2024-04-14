//! Generate Dart data classes

use anyhow::Result;

use crate::{file_finder::DartFile, parser::{ClassDefinition, ParsedFile, TopLevelItems}};

use self::data_class::ValidatedClass;

mod wrong_constructor_exception;
mod part_of;
mod data_class;
mod utils;

pub const GENERATED_FILE_HEADER: &str = "//
// AUTOMATICALLY GENERATED CODE
//
// Generated with icegen
// ---------------------------------------------------------------------
";

struct StringEditor {
    content: String,
}

impl StringEditor {
    fn new(content: String) -> Self {
        Self { content }
    }

    fn add_paragraph(&mut self, paragraph: impl AsRef<str>) {
        self.content.push_str(&format!("\n{}\n", paragraph.as_ref()));
    }

    fn trim_end_and_add_final_newline(self) -> String {
        format!("{}\n", self.content.trim_end())
    }
}

struct ValidatedFile {
    pub flutter_foundation_import_exists: bool,
}

impl ValidatedFile {
    pub fn validate(parsed_file: &ParsedFile) -> Result<Self> {
        let mut flutter_foundation_import_exists = false;

        for item in &parsed_file.items {
            match item {
                TopLevelItems::Import(import) => {
                    if import.is_flutter_foundation_import() {
                        flutter_foundation_import_exists = true;
                    }
                }
                _ => (),
            }
        }

        Ok(ValidatedFile {
            flutter_foundation_import_exists,
        })
    }
}

pub fn generate_data_class_file(file: &DartFile) -> Result<String> {
    let validated = ValidatedFile::validate(&file.parsed_file)?;

    let mut editor = StringEditor::new(format!("{}", GENERATED_FILE_HEADER));

    editor.add_paragraph(part_of::generate_part_of_statement(file)?);
    editor.add_paragraph(
        wrong_constructor_exception::generate_final_variable_for_exception()
    );

    for item in &file.parsed_file.items {
        match item {
            TopLevelItems::Class(class) => {
                if !class.contains_freezed_annotation() {
                    continue;
                }

                generate_data_class(&validated, class, &mut editor)?;
            }
            _ => (),
        }
    }

    Ok(editor.trim_end_and_add_final_newline())
}

fn generate_data_class(file: &ValidatedFile, class: &ClassDefinition, editor: &mut StringEditor) -> Result<()> {
    let validated = ValidatedClass::validate(class)?;

    editor.add_paragraph(data_class::mixin::generate_mixin(&validated)?);
    editor.add_paragraph(data_class::abstract_class::generate_abstract_class(&validated)?);
    editor.add_paragraph(data_class::impl_class::generate_impl_class(file, &validated)?);

    Ok(())
}
