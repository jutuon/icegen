//! Find and parse Dart files from a directory

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::parser::ParsedFile;

const DART_FILE_EXTENSION: &str = ".dart";
const FREEZED_GENERATED_CODE_FILE_EXTENSION: &str = ".freezed.dart";
pub const FREEZED_GENERATED_CODE_FILE_EXTENSION_WITHOUT_LEADING_DOT: &str = "freezed.dart";

#[derive(Debug)]
pub struct DartFile {
    pub path: PathBuf,
    pub parsed_file: ParsedFile,
}

/// Parsed Dart files in a directory and its subdirectories.
///
/// This does not contain Dart files ending with `.freezed.dart`.
#[derive(Debug)]
pub struct ParsedDartFiles {
    pub code_files: Vec<DartFile>,
}

pub fn parse_freezed_classes_from_dart_files(
    code_dir: impl AsRef<Path>,
) -> Result<ParsedDartFiles> {
    let mut parsed_files = Vec::<DartFile>::new();

    handle_one_code_dir(code_dir, &mut parsed_files)?;

    Ok(ParsedDartFiles {
        code_files: parsed_files,
    })
}

fn handle_one_code_dir(
    code_dir: impl AsRef<Path>,
    parsing_results: &mut Vec<DartFile>,
) -> Result<()> {
    for entry in fs::read_dir(code_dir)? {
        let entry = entry?;

        if entry.path().is_dir() {
            handle_one_code_dir(entry.path(), parsing_results)?;
            continue;
        }

        if !entry.path().is_file() {
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.ends_with(FREEZED_GENERATED_CODE_FILE_EXTENSION) {
            continue;
        }

        if !file_name.ends_with(DART_FILE_EXTENSION) {
            continue;
        }

        if let Some(parsed_classes) = handle_dart_file(entry.path())? {
            parsing_results.push(parsed_classes);
        }
    }

    Ok(())
}

fn handle_dart_file(dart_code_file: impl AsRef<Path>) -> Result<Option<DartFile>> {
    let path = dart_code_file.as_ref();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read Dart code file at {:?}", path))?;

    let file = ParsedFile::parse_dart_file(&contents)?;

    Ok(Some(DartFile {
        path: path.to_owned(),
        parsed_file: file,
    }))
}
