use std::{path::PathBuf, str::FromStr};

use clap::{arg, Parser};

#[derive(Parser)]
#[command(author, version, about)]
pub struct ArgsConfig {
    /// Directory where Dart code files are located.
    #[arg(long, value_name = "DIR")]
    pub code_dir: PathBuf,
}

pub fn get_config() -> ArgsConfig {
    ArgsConfig::parse()
}
