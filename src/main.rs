
pub mod config;
pub mod parser;
pub mod file_finder;

fn main() {
    let config = config::get_config();

    let parsed_files = file_finder::parse_freezed_classes_from_dart_files(&config.code_dir).unwrap();
    dbg!(parsed_files);
}
