use chrono::prelude::Utc;
use std::path::Path;

/// Generates a unique filename based on the current timestamp.
/// This is useful for saving files that should not overwrite each other.
pub fn generate_unique_filename(base_path: &str, extension: &str) -> String {
    let now = Utc::now();
    let date_str = now.format("%Y%m%d%H%M%S").to_string();

    // Ensure base_path ends with a directory separator
    let base_path = if base_path.ends_with('/') {
        base_path.to_string()
    } else {
        format!("{}/", base_path)
    };

    format!("{}Wallrus-{}.{}", base_path, date_str, extension)
}

/// Validates if a path points to a valid file.
/// Returns true if the path exists and is a file.
pub fn is_valid_file(path: &str) -> bool {
    Path::new(path).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_generate_unique_filename() {
        let result = generate_unique_filename("/tmp/", "jpg");
        assert!(result.starts_with("/tmp/Wallrus-"));
        assert!(result.ends_with(".jpg"));
    }

    #[test]
    fn test_is_valid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        assert!(is_valid_file(file_path.to_str().unwrap()));
        assert!(!is_valid_file("/nonexistent/file.txt"));
    }
}
