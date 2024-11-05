use std::{fs, path::Path, thread, time::Duration};

use rand::Rng;

use crate::{
    engine,
    errors::{Result, WallrusError},
    utils::generate_unique_filename,
};

use super::generators::{
    generate_gradient_wallpaper, generate_random_plot_wallpaper, generate_random_walk_wallpaper,
};

#[derive(Debug, Clone)]
pub struct WallpaperConfig {
    pub width: u32,
    pub height: u32,
    pub line_thickness: u32,
    pub step_size: u32,
    pub num_steps: u32,
    pub saturation: f32,
    pub value: f32,
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            line_thickness: 3,
            step_size: 5,
            num_steps: 5000,
            saturation: 0.7,
            value: 0.8,
        }
    }
}

impl WallpaperConfig {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(WallrusError::Config(
                "Width and height must be greater than 0".into(),
            ));
        }

        Ok(Self {
            width,
            height,
            ..Default::default()
        })
    }

    #[allow(dead_code)]
    pub fn with_line_thickness(mut self, thickness: u32) -> Self {
        self.line_thickness = thickness;
        self
    }

    #[allow(dead_code)]
    pub fn with_step_size(mut self, step_size: u32) -> Self {
        self.step_size = step_size;
        self
    }

    #[allow(dead_code)]
    pub fn with_num_steps(mut self, num_steps: u32) -> Self {
        self.num_steps = num_steps;
        self
    }
}

pub fn hsv_to_rgb(hue: i32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let c = value * saturation;
    let x = c * (1.0 - (((hue as f32 / 60.0) % 2.0) - 1.0).abs());
    let m = value - c;
    let (r_prime, g_prime, b_prime) = if hue >= 0 && hue < 60 {
        (c, x, 0.0)
    } else if hue >= 60 && hue < 120 {
        (x, c, 0.0)
    } else if hue >= 120 && hue < 180 {
        (0.0, c, x)
    } else if hue >= 180 && hue < 240 {
        (0.0, x, c)
    } else if hue >= 240 && hue < 300 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (
        ((r_prime + m) * 255.0).round() as u8,
        ((g_prime + m) * 255.0).round() as u8,
        ((b_prime + m) * 255.0).round() as u8,
    )
}

pub enum WallpaperType {
    Gradient,
    RandomWalk,
    RandomPlot,
}

/// Generates a wallpaper based on the specified type and saves it to the given file path.
pub fn generate_wallpaper(width: u32, height: u32, file_path: &str) -> Result<()> {
    let file_path = generate_unique_filename(file_path, "jpg");
    let file_path = Path::new(&file_path);

    let wallpaper_type = match rand::thread_rng().gen_range(0..3) {
        0 => WallpaperType::Gradient,
        1 => WallpaperType::RandomPlot,
        _ => WallpaperType::RandomWalk,
    };

    match wallpaper_type {
        WallpaperType::Gradient => generate_gradient_wallpaper(width, height, file_path)?,
        WallpaperType::RandomWalk => generate_random_walk_wallpaper(width, height, file_path)?,
        WallpaperType::RandomPlot => generate_random_plot_wallpaper(width, height, file_path)?,
    }

    Ok(())
}

/// Creates a slideshow from images in a specified directory, changing wallpaper at a given interval.
pub fn create_slideshow(image_directory: &str, interval: Duration) -> Result<()> {
    let paths = fs::read_dir(image_directory).map_err(|e| WallrusError::Io(e))?;

    let valid_paths: Vec<_> = paths
        .filter_map(|entry| {
            entry.ok().filter(|e| {
                let path = e.path();
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "jpg" || ext == "png")
                    .unwrap_or(false)
            })
        })
        .collect();

    if valid_paths.is_empty() {
        return Err(WallrusError::Config(
            "No valid images found in directory".into(),
        ));
    }

    println!("Starting slideshow with {} images", valid_paths.len());

    loop {
        for entry in &valid_paths {
            let image_path = entry.path();
            println!("Setting wallpaper: {:?}", image_path);
            engine::set_wallpaper(&image_path)?;
            thread::sleep(interval);
        }
    }
}
