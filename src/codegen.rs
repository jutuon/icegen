//! Generate Dart data classes

use anyhow::Result;

use crate::{
    file_finder::DartFile,
    parser::{ClassDefinition, ParsedFile, TopLevelItems},
};

use self::data_class::ValidatedClass;

mod data_class;
mod part_of;
mod utils;

pub const GENERATED_FILE_HEADER: &str = "// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
";

pub const GENERATOR_INFO_TEXT: &str =
    "// **************************************************************************
// Generated with Icegen
// **************************************************************************";

struct StringEditor {
    content: String,
}

impl StringEditor {
    fn new(content: String) -> Self {
        Self { content }
    }

    fn add_paragraph(&mut self, paragraph: impl AsRef<str>) {
        self.content
            .push_str(&format!("\n{}\n", paragraph.as_ref()));
    }

    fn trim_end_and_add_final_newline(self) -> String {
        format!("{}\n", self.content.trim_end())
    }

    fn append(&mut self, editor: &StringEditor) {
        self.content.push_str(&editor.content);
    }
}

struct ValidatedFile {
    pub flutter_foundation_import_exists: bool,
}

impl ValidatedFile {
    pub fn validate(parsed_file: &ParsedFile) -> Result<Self> {
        let mut flutter_foundation_import_exists = false;

        for item in &parsed_file.items {
            if let TopLevelItems::Import(import) = item {
                if import.is_flutter_foundation_import() {
                    flutter_foundation_import_exists = true;
                }
            }
        }

        Ok(ValidatedFile {
            flutter_foundation_import_exists,
        })
    }
}

pub fn generate_data_class_file(file: &DartFile) -> Result<String> {
    let validated = ValidatedFile::validate(&file.parsed_file)?;

    let mut nullable_named_parameter_exists = false;
    let mut class_specific_code = StringEditor::new(String::new());

    for item in &file.parsed_file.items {
        if let TopLevelItems::Class(class) = item {
            if !class.contains_freezed_annotation() {
                continue;
            }

            generate_data_class(
                &validated,
                class,
                &mut class_specific_code,
                &mut nullable_named_parameter_exists,
            )?;
        }
    }

    let mut editor = StringEditor::new(GENERATED_FILE_HEADER.to_string());

    editor.add_paragraph(part_of::generate_part_of_statement(file)?);
    editor.add_paragraph(GENERATOR_INFO_TEXT);

    if nullable_named_parameter_exists {
        editor.add_paragraph(data_class::impl_class::generate_detect_default_class_and_constant());
    }

    editor.append(&class_specific_code);

    Ok(editor.trim_end_and_add_final_newline())
}

fn generate_data_class(
    file: &ValidatedFile,
    class: &ClassDefinition,
    editor: &mut StringEditor,
    nullable_named_paramter_exists: &mut bool,
) -> Result<()> {
    let validated = ValidatedClass::validate(class)?;

    editor.add_paragraph(data_class::mixin::generate_mixin(&validated)?);
    editor.add_paragraph(data_class::abstract_class::generate_abstract_class(
        &validated,
    )?);
    editor.add_paragraph(data_class::impl_class::generate_impl_class(
        file, &validated,
    )?);

    *nullable_named_paramter_exists =
        *nullable_named_paramter_exists || validated.nullable_named_parameter_exists();

    Ok(())
}
