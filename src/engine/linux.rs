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
    // Method 1: Try native Wayland layer shell (preferred)
    #[cfg(feature = "wayland")] 
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            if let Ok(()) = crate::engine::wayland::set_wayland_wallpaper(image_path) {
                return Ok(());
            }
            // If native fails, fall back to external tools
            eprintln!("[DEBUG] Native Wayland failed, trying external tools...");
        }
    }

    let image_path_str = image_path.to_str().ok_or_else(|| {
        WallrusError::Config("Invalid image path".into())
    })?;

    // Method 2: Detect and try available external tools (fallback)
    let available_tools = detect_wallpaper_tools();
    eprintln!("[DEBUG] Detected wallpaper tools: {:?}", available_tools);
    
    for tool in &available_tools {
        eprintln!("[DEBUG] Trying tool: {}", tool);
        match tool.as_str() {
            "hyprpaper" => {
                if try_hyprpaper(image_path_str).is_ok() {
                    return Ok(());
                }
            }
            "swww" => {
                if try_swww(image_path_str).is_ok() {
                    return Ok(());
                }
            }
            "swaybg" => {
                if try_swaybg(image_path_str).is_ok() {
                    return Ok(());
                }
            }
            _ => continue,
        }
    }

    // Generate appropriate error message
    #[cfg(feature = "wayland")]
    let error_msg = format!(
        "Wallpaper setting failed. Native Wayland support failed and no external tools available. Detected tools: {:?}",
        available_tools
    );

    #[cfg(not(feature = "wayland"))]
    let error_msg = format!(
        "No wallpaper utilities available for Hyprland. Please install one of: hyprpaper, swww, or swaybg, or compile with --features wayland for native support. Detected tools: {:?}",
        available_tools
    );

    Err(WallrusError::Config(error_msg))
}

fn detect_wallpaper_tools() -> Vec<String> {
    let mut tools = Vec::new();
    
    // Check for hyprpaper (via hyprctl)
    if Command::new("hyprctl").arg("--help").output().is_ok() {
        tools.push("hyprpaper".to_string());
    }
    
    // Check for swww
    if Command::new("swww").arg("--version").output().is_ok() {
        tools.push("swww".to_string());
    }
    
    // Check for swaybg
    if Command::new("swaybg").arg("--version").output().is_ok() {
        tools.push("swaybg".to_string());
    }
    
    tools
}

fn try_hyprpaper(image_path_str: &str) -> Result<()> {
    // First preload the image (required by hyprpaper)
    let _preload = Command::new("hyprctl")
        .args(["hyprpaper", "preload", image_path_str])
        .output();
    
    // Then set as wallpaper
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &format!(",{}", image_path_str)])
        .output()
        .map_err(WallrusError::Io)?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(WallrusError::Config("hyprpaper failed".into()))
    }
}

fn try_swww(image_path_str: &str) -> Result<()> {
    let output = Command::new("swww")
        .args(["img", image_path_str])
        .output()
        .map_err(WallrusError::Io)?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(WallrusError::Config("swww failed".into()))
    }
}

fn try_swaybg(image_path_str: &str) -> Result<()> {
    // Kill existing swaybg instances first
    let _ = Command::new("pkill").arg("swaybg").output();
    
    // Start new swaybg instance
    let mut child = Command::new("swaybg")
        .args(["-i", image_path_str])
        .spawn()
        .map_err(WallrusError::Io)?;
    
    // Give it a moment to start
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    match child.try_wait().map_err(WallrusError::Io)? {
        Some(_) => Err(WallrusError::Config("swaybg failed to start".into())),
        None => Ok(()), // Still running, which is good
    }
}
