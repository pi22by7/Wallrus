// utils.rs

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a unique filename based on the current timestamp.
/// This is useful for saving files that should not overwrite each other.
pub fn generate_unique_filename(base_path: &str, extension: &str) -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    format!(
        "{}Wallrus-{}.{}",
        base_path,
        since_the_epoch.as_secs(),
        extension
    )
}

/// Converts an `Option<String>` to an `Option<&str>`.
pub fn str_option_to_slice(option: &Option<String>) -> Option<&str> {
    option.as_deref()
}

/// Validates if a path points to a valid file.
/// Returns true if the path exists and is a file.
pub fn is_valid_file(path: &str) -> bool {
    Path::new(path).is_file()
}
