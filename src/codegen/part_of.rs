
use anyhow::{anyhow, Result};

use crate::{file_finder::DartFile};

pub fn generate_part_of_statement(file: &DartFile) -> Result<String> {
    let file_name = file.path.file_name()
        .ok_or(anyhow!("Could not get file name"))?
        .to_str()
        .ok_or(anyhow!("Could not convert file name to string"))?;

    if file_name.contains(['\'', '$']) {
        // Prevent code injection and string formatting using file name
        return Err(anyhow!("File name '{}' contains invalid characters", file_name));
    }

    Ok(format!(
        "part of '{}';",
        file_name,
    ))
}
