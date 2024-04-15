//! Update generated code files if needed



use std::fs;
use anyhow::{anyhow, Result};

use crate::codegen::{generate_data_class_file, GENERATED_FILE_HEADER};
use crate::config::{ArgsConfig};
use crate::file_finder::{DartFile, ParsedDartFiles, FREEZED_GENERATED_CODE_FILE_EXTENSION_WITHOUT_LEADING_DOT};


pub fn update_generated_code_for_parsed_files(config: &ArgsConfig, files: ParsedDartFiles) -> Result<()> {
    for file in files.code_files {
        if !file.parsed_file.contains_freezed_annotated_class() {
            continue;
        }

        let generated_code = generate_data_class_file(&file)?;
        update_generated_code_if_needed(config, file, generated_code)?;
    }

    Ok(())
}

pub fn update_generated_code_if_needed(
    config: &ArgsConfig,
    file: DartFile,
    new_generated_code: String,
) -> Result<()> {
    let generated_code_path = file.path.with_extension(
        FREEZED_GENERATED_CODE_FILE_EXTENSION_WITHOUT_LEADING_DOT
    );

    if generated_code_path.exists() {
        let current_generated_code = fs::read_to_string(&generated_code_path)?;

        if current_generated_code == new_generated_code {
            return Ok(());
        }

        if !config.force && !current_generated_code.starts_with(GENERATED_FILE_HEADER) {
            return Err(anyhow!(
                "Generated code file {:?} does not start with the expected header",
                generated_code_path
            ));
        }
    }

    fs::write(generated_code_path, new_generated_code)?;

    Ok(())
}
