use crate::errors::{Result, WallrusError};
use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(image_path: &Path) -> Result<()> {
    if !image_path.exists() {
        return Err(WallrusError::Config(format!(
            "Wallpaper file does not exist: {:?}",
            image_path
        )));
    }

    // Use osascript to set wallpaper
    let script = format!(
        "tell application \"Finder\" to set desktop picture to POSIX file \"{}\"",
        image_path.display()
    );

    Command::new("osascript")
        .args(&["-e", &script])
        .output()
        .map_err(|e| WallrusError::Io(e))?;

    Ok(())
}
