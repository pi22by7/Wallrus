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
            "Hyprland" => set_hyprland_wallpaper(image_path),
            _ => Err(WallrusError::Config(format!(
                "Unsupported desktop environment: {}",
                desktop
            ))),
        }
    } else {
        // Check if we're running under Hyprland even without XDG_CURRENT_DESKTOP
        if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            set_hyprland_wallpaper(image_path)
        } else {
            Err(WallrusError::Config(
                "Could not detect desktop environment".into(),
            ))
        }
    }
}

fn set_gnome_wallpaper(image_path: &Path) -> Result<()> {
    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &format!("file://{}", image_path.display()),
        ])
        .output()
        .map_err(WallrusError::Io)?;
    Ok(())
}

fn set_kde_wallpaper(image_path: &Path) -> Result<()> {
    // KDE Plasma 5
    Command::new("qdbus")
        .args([
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
        .map_err(WallrusError::Io)?;
    Ok(())
}

fn set_xfce_wallpaper(image_path: &Path) -> Result<()> {
    Command::new("xfconf-query")
        .args([
            "-c",
            "xfce4-desktop",
            "-p",
            "/backdrop/screen0/monitor0/workspace0/last-image",
            "-s",
            image_path.to_str().unwrap(),
        ])
        .output()
        .map_err(WallrusError::Io)?;
    Ok(())
}

fn set_hyprland_wallpaper(image_path: &Path) -> Result<()> {
    let image_path_str = image_path.to_str().ok_or_else(|| {
        WallrusError::Config("Invalid image path".into())
    })?;

    // Method 1: Try hyprpaper via hyprctl (preferred)
    if let Ok(output) = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &format!(",{}", image_path_str)])
        .output()
    {
        if output.status.success() {
            return Ok(());
        }
    }

    // Method 2: Try swww (fallback)
    if let Ok(output) = Command::new("swww")
        .args(["img", image_path_str])
        .output()
    {
        if output.status.success() {
            return Ok(());
        }
    }

    // Method 3: Try swaybg (basic fallback)
    if let Ok(mut child) = Command::new("swaybg")
        .args(["-i", image_path_str])
        .spawn()
    {
        // swaybg runs as a daemon, so we don't wait for it to finish
        // Just check if it started successfully
        std::thread::sleep(std::time::Duration::from_millis(100));
        if let Ok(Some(_)) = child.try_wait() {
            return Err(WallrusError::Config(
                "swaybg failed to start".into()
            ));
        }
        return Ok(());
    }

    Err(WallrusError::Config(
        "No supported Hyprland wallpaper utility found. Please install hyprpaper, swww, or swaybg".into()
    ))
}
