
pub mod config;
pub mod parser;
pub mod file_finder;
pub mod codegen;
pub mod file_writer;

fn main() {
    let config = config::get_config();

    let parsed_files = file_finder::parse_freezed_classes_from_dart_files(&config.code_dir).unwrap();
    file_writer::update_generated_code_for_parsed_files(parsed_files).unwrap();
}
