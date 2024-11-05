use super::super::set_wallpaper;
use crate::engine::common::hsv_to_rgb;
use crate::errors::{Result, WallrusError};
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::Rng;
use std::path::Path;

pub fn generate_gradient_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();

    let base_hue = rng.gen_range(0..360);
    let start_color = hsv_to_rgb(base_hue, 0.7, 0.8);
    let end_color = hsv_to_rgb((base_hue + 30) % 360, 0.7, 0.8);

    for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = start_color.0 as f32
            + (end_color.0 as f32 - start_color.0 as f32) * (x as f32 / width as f32);
        let g = start_color.1 as f32
            + (end_color.1 as f32 - start_color.1 as f32) * (x as f32 / width as f32);
        let b = start_color.2 as f32
            + (end_color.2 as f32 - start_color.2 as f32) * (x as f32 / width as f32);
        *pixel = Rgba([r as u8, g as u8, b as u8, 255]);
    }

    imgbuf.save(file_path).map_err(|e| {
        WallrusError::ImageProcessing(format!("Failed to save gradient image: {}", e))
    })?;

    println!("Gradient wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path)?;
    Ok(())
}
