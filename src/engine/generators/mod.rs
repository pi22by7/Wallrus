mod gradient;
mod random_plot;
mod random_walk;

pub use gradient::generate_gradient_wallpaper;
pub use random_plot::generate_random_plot_wallpaper;
pub use random_walk::generate_random_walk_wallpaper;

use super::WallpaperType;
use crate::errors::Result;
use crate::utils::generate_unique_filename;
use rand::{thread_rng, Rng};
use std::path::Path;

#[allow(dead_code)]
pub fn generate_wallpaper(width: u32, height: u32, file_path: &str) -> Result<()> {
    let file_path = generate_unique_filename(file_path, "jpg");
    let file_path = Path::new(&file_path);

    let wallpaper_type = match thread_rng().gen_range(0..3) {
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
