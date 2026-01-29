use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct ArgsConfig {
    /// Directory where Dart code files are located. If '.dart' files
    /// containing @freezed or @Freezed annotated classes are found
    /// from this directory or its subdirectories, files ending
    /// with '.freezed.dart' will be generated next to found files.
    #[arg(long, value_name = "DIR")]
    pub code_dir: PathBuf,

    /// Force update of an existing generated file even if it contains
    /// unknown file header.
    #[arg(long)]
    pub force: bool,
}

pub fn get_config() -> ArgsConfig {
    ArgsConfig::parse()
}
