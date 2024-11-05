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

    // Try different desktop environments
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        match desktop.as_str() {
            "GNOME" | "Unity" | "GNOME-Classic" => set_gnome_wallpaper(image_path),
            "KDE" => set_kde_wallpaper(image_path),
            "XFCE" => set_xfce_wallpaper(image_path),
            _ => Err(WallrusError::Config(format!(
                "Unsupported desktop environment: {}",
                desktop
            ))),
        }
    } else {
        Err(WallrusError::Config(
            "Could not detect desktop environment".into(),
        ))
    }
}

fn set_gnome_wallpaper(image_path: &Path) -> Result<()> {
    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &format!("file://{}", image_path.display()),
        ])
        .output()
        .map_err(|e| WallrusError::Io(e))?;
    Ok(())
}

fn set_kde_wallpaper(image_path: &Path) -> Result<()> {
    // KDE Plasma 5
    Command::new("qdbus")
        .args(&[
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &format!(
                "var allDesktops = desktops();
                for (i=0;i<allDesktops.length;i++) {{
                    d = allDesktops[i];
                    d.wallpaperPlugin = 'org.kde.image';
                    d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');
                    d.writeConfig('Image', 'file://{}')",
                image_path.display()
            ),
        ])
        .output()
        .map_err(|e| WallrusError::Io(e))?;
    Ok(())
}

fn set_xfce_wallpaper(image_path: &Path) -> Result<()> {
    Command::new("xfconf-query")
        .args(&[
            "-c",
            "xfce4-desktop",
            "-p",
            "/backdrop/screen0/monitor0/workspace0/last-image",
            "-s",
            image_path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| WallrusError::Io(e))?;
    Ok(())
}
