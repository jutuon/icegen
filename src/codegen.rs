//! Generate Dart data classes

use anyhow::Result;

use crate::{file_finder::DartFile, parser::{ClassDefinition, ParsedFile, TopLevelItems}};

mod wrong_constructor_exception;
mod part_of;

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

pub fn generate_data_class_file(file: &DartFile) -> Result<String> {
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

                generate_data_class(file, class, &mut editor)?;
            }
            _ => (),
        }
    }

    Ok(editor.trim_end_and_add_final_newline())
}

fn generate_data_class(file: &DartFile, class: &ClassDefinition, editor: &mut StringEditor) -> Result<()> {
    Ok(())
}
