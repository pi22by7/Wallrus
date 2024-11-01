// utils.rs

use chrono::prelude::Utc;
use std::path::Path;

/// Generates a unique filename based on the current timestamp.
/// This is useful for saving files that should not overwrite each other.
pub fn generate_unique_filename(base_path: &str, extension: &str) -> String {
    let now = Utc::now();
    let date_str = now.format("%Y%m%d%H%M%S").to_string();

    format!("{}Wallrus-{}.{}", base_path, date_str, extension)
}

/// Converts an `Option<String>` to an `Option<&str>`.
#[allow(dead_code)]
pub fn str_option_to_slice(option: &Option<String>) -> Option<&str> {
    option.as_deref()
}

/// Validates if a path points to a valid file.
/// Returns true if the path exists and is a file.
pub fn is_valid_file(path: &str) -> bool {
    Path::new(path).is_file()
}
