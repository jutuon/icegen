#[allow(clippy::collapsible_else_if)]

pub mod codegen;
pub mod config;
pub mod file_finder;
pub mod file_writer;
pub mod parser;

fn main() {
    let config = config::get_config();

    let parsed_files = match file_finder::parse_freezed_classes_from_dart_files(&config.code_dir) {
        Ok(parsed_files) => parsed_files,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match file_writer::update_generated_code_for_parsed_files(&config, parsed_files) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
