use super::super::set_wallpaper;
use crate::engine::common::{hsv_to_rgb, WallpaperConfig};
use crate::errors::{Result, WallrusError};
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::Rng;
use std::path::Path;

struct RandomWalker {
    x: f32,
    y: f32,
    angle: f32,
}

impl RandomWalker {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            angle: 0.0,
        }
    }

    fn step(&mut self, rng: &mut impl Rng, step_size: u32) {
        // Perlin noise could be used here for smoother movement
        self.angle += rng.gen_range(-0.5..0.5);
        self.x += step_size as f32 * self.angle.cos();
        self.y += step_size as f32 * self.angle.sin();
    }

    fn draw(
        &self,
        imgbuf: &mut RgbaImage,
        width: u32,
        height: u32,
        color: (u8, u8, u8),
        thickness: u32,
    ) {
        let x = self.x.round() as i32;
        let y = self.y.round() as i32;

        for dx in -(thickness as i32) / 2..=thickness as i32 / 2 {
            for dy in -(thickness as i32) / 2..=thickness as i32 / 2 {
                let px = (x + dx).clamp(0, width as i32 - 1) as u32;
                let py = (y + dy).clamp(0, height as i32 - 1) as u32;
                imgbuf.put_pixel(px, py, Rgba([color.0, color.1, color.2, 255]));
            }
        }
    }
}

pub fn generate_random_walk_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    let config = WallpaperConfig::default();
    let (width, height) = (config.width, config.height); // Use width and height

    // Generate colors with better contrast
    let base_hue = rng.gen_range(0..360);
    let background_color = hsv_to_rgb(base_hue, config.saturation * 0.5, config.value);
    let line_color = hsv_to_rgb((base_hue + 180) % 360, config.saturation, config.value); // Complementary color

    // Fill background
    imgbuf.pixels_mut().for_each(|pixel| {
        *pixel = Rgba([
            background_color.0,
            background_color.1,
            background_color.2,
            255,
        ]);
    });

    let mut walker = RandomWalker::new(width / 2, height / 2);

    for _ in 0..config.num_steps {
        walker.step(&mut rng, config.step_size);
        walker.draw(
            &mut imgbuf,
            width,
            height,
            line_color,
            config.line_thickness,
        );
    }

    imgbuf
        .save(file_path)
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to save image: {}", e)))?;

    println!("Random walk wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path)?;
    Ok(())
}
