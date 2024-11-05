use super::super::set_wallpaper;
use crate::engine::common::{hsv_to_rgb, WallpaperConfig};
use crate::errors::{Result, WallrusError};
use image::{ImageBuffer, Rgba, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::path::Path;

struct RandomWalker {
    x: f32,
    y: f32,
    angle: f32,
    hue: f32,
    speed: f32,
}

impl RandomWalker {
    fn new(x: u32, y: u32, hue: f32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            angle: rand::thread_rng().gen_range(0.0..std::f32::consts::TAU),
            hue,
            speed: rand::thread_rng().gen_range(0.5..2.0),
        }
    }

    fn step(&mut self, rng: &mut impl Rng, step_size: u32) {
        // Smoother movement with sine wave variation
        let time = self.angle * 0.1;
        let noise = (time.sin() * 0.3 + rng.gen_range(-0.2..0.2)) * self.speed;
        self.angle += noise;

        // Update position with varying step sizes
        let actual_step = step_size as f32 * self.speed;
        self.x += actual_step * self.angle.cos();
        self.y += actual_step * self.angle.sin();

        // Gradually change hue
        self.hue += 0.1;
        if self.hue >= 360.0 {
            self.hue -= 360.0;
        }
    }

    fn draw(
        &self,
        imgbuf: &mut RgbaImage,
        width: u32,
        height: u32,
        base_color: (u8, u8, u8),
        thickness: u32,
    ) {
        let x = self.x.round() as i32;
        let y = self.y.round() as i32;

        // Calculate distance from center for opacity
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_distance = ((center_x.powi(2) + center_y.powi(2)) as f32).sqrt();
        let distance = (((self.x - center_x).powi(2) + (self.y - center_y).powi(2)) as f32).sqrt();
        let opacity = ((1.0 - distance / max_distance) * 255.0) as u8;

        // Create gradient effect based on position
        let color = hsv_to_rgb(self.hue as i32, 0.7, 0.8);

        // Blend with base color
        let blend_factor = 0.7;
        let final_color = (
            ((color.0 as f32 * blend_factor + base_color.0 as f32 * (1.0 - blend_factor)) as u8),
            ((color.1 as f32 * blend_factor + base_color.1 as f32 * (1.0 - blend_factor)) as u8),
            ((color.2 as f32 * blend_factor + base_color.2 as f32 * (1.0 - blend_factor)) as u8),
        );

        // Draw with gradient thickness
        let gradient_thickness = (thickness as f32 * (1.0 - distance / max_distance)) as u32;
        for dx in -(gradient_thickness as i32) / 2..=gradient_thickness as i32 / 2 {
            for dy in -(gradient_thickness as i32) / 2..=gradient_thickness as i32 / 2 {
                if x + dx < 0 || x + dx >= width as i32 || y + dy < 0 || y + dy >= height as i32 {
                    continue;
                }

                let px = (x + dx) as u32;
                let py = (y + dy) as u32;

                // Calculate distance from center of the brush for smooth edges
                let brush_distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
                let brush_opacity =
                    ((1.0 - brush_distance / gradient_thickness as f32) * opacity as f32) as u8;

                let current_pixel = imgbuf.get_pixel(px, py);
                let new_pixel = Rgba([
                    blend_colors(current_pixel[0], final_color.0, brush_opacity),
                    blend_colors(current_pixel[1], final_color.1, brush_opacity),
                    blend_colors(current_pixel[2], final_color.2, brush_opacity),
                    255,
                ]);

                imgbuf.put_pixel(px, py, new_pixel);
            }
        }
    }
}

fn blend_colors(c1: u8, c2: u8, alpha: u8) -> u8 {
    let a = alpha as f32 / 255.0;
    ((c1 as f32) * (1.0 - a) + (c2 as f32) * a) as u8
}

pub fn generate_random_walk_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    let config = WallpaperConfig::default();

    // Setup progress bar
    let total_steps = config.num_steps * 5; // 5 walkers
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Progress update batch size to reduce overhead
    let update_frequency = (total_steps / 100).max(1) as u64; // Update roughly 100 times total
    let mut progress_counter = 0u64;

    // Generate base colors
    let base_hue = rng.gen_range(0..360);
    let background_color = hsv_to_rgb(base_hue, 0.3, 0.95);
    let accent_color = hsv_to_rgb((base_hue + 180) % 360, 0.7, 0.8);

    // Fill background with gradient (pre-allocate for better performance)
    let gradient_buffer: Vec<Rgba<u8>> = (0..width * height)
        .map(|i| {
            let x = (i % width) as f32;
            let y = (i / width) as f32;
            let gradient = (x / width as f32 * 0.5 + y / height as f32 * 0.5) as f32;
            Rgba([
                (background_color.0 as f32 * (1.0 - gradient)) as u8,
                (background_color.1 as f32 * (1.0 - gradient)) as u8,
                (background_color.2 as f32 * (1.0 - gradient)) as u8,
                255,
            ])
        })
        .collect();

    // Copy pre-calculated gradient to image buffer
    imgbuf
        .pixels_mut()
        .zip(gradient_buffer.iter())
        .for_each(|(pixel, &color)| {
            *pixel = color;
        });

    // Create multiple walkers
    let num_walkers = 5;
    let mut walkers: Vec<RandomWalker> = (0..num_walkers)
        .map(|i| {
            let angle = (i as f32 / num_walkers as f32) * std::f32::consts::TAU;
            let radius = (width.min(height) / 4) as f32;
            let x = (width as f32 / 2.0 + angle.cos() * radius) as u32;
            let y = (height as f32 / 2.0 + angle.sin() * radius) as u32;
            let hue = base_hue as f32 + (360.0 / num_walkers as f32) * i as f32;
            RandomWalker::new(x, y, hue)
        })
        .collect();

    // Animate walkers with multiple passes
    let passes = 3;
    for pass in 0..passes {
        let steps_per_pass = config.num_steps / passes as u32;
        let opacity_factor = 1.0 - (pass as f32 / passes as f32) * 0.3;

        for step in 0..steps_per_pass {
            let progress = step as f32 / steps_per_pass as f32;

            for walker in walkers.iter_mut() {
                let dynamic_step_size =
                    config.step_size as f32 * (1.0 + (progress * std::f32::consts::PI).sin() * 0.5);

                walker.step(&mut rng, dynamic_step_size as u32);

                // Boundary handling
                walker.x = walker.x.rem_euclid(width as f32);
                walker.y = walker.y.rem_euclid(height as f32);

                let dynamic_thickness = (config.line_thickness as f32
                    * walker.speed.powf(0.5)
                    * opacity_factor
                    * (1.0 + (progress * std::f32::consts::PI * 2.0).sin() * 0.3))
                    as u32;

                walker.draw(&mut imgbuf, width, height, accent_color, dynamic_thickness);

                // Update progress
                progress_counter += 1;
                if progress_counter % update_frequency == 0 {
                    pb.set_position(progress_counter);
                }
            }
        }
    }

    // Finish progress bar
    pb.finish_with_message("Applying final effects...");

    // Apply post-processing with simplified blur for better performance
    apply_fast_post_processing(&mut imgbuf, width, height)?;

    // Save the image
    pb.set_message("Saving image...");
    imgbuf
        .save(file_path)
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to save image: {}", e)))?;

    println!("Random walk wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path)?;
    Ok(())
}

fn apply_fast_post_processing(imgbuf: &mut RgbaImage, width: u32, height: u32) -> Result<()> {
    let blur_radius = ((width.min(height) as f32 * 0.01) as u32).min(5); // Limit blur radius for performance
    let kernel_size = (2 * blur_radius + 1) as usize;
    let kernel: Vec<f32> = (0..kernel_size)
        .map(|i| {
            let x = (i as f32 - blur_radius as f32) / blur_radius as f32;
            (-x * x / 2.0).exp()
        })
        .collect();

    // Horizontal blur pass
    let mut temp = imgbuf.clone();
    for y in 0..height {
        for x in 0..width {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut weight_sum = 0.0;

            for (i, &weight) in kernel.iter().enumerate() {
                let ox =
                    (x as i32 + i as i32 - blur_radius as i32).clamp(0, width as i32 - 1) as u32;
                let pixel = imgbuf.get_pixel(ox, y);
                r += pixel[0] as f32 * weight;
                g += pixel[1] as f32 * weight;
                b += pixel[2] as f32 * weight;
                weight_sum += weight;
            }

            temp.put_pixel(
                x,
                y,
                Rgba([
                    (r / weight_sum) as u8,
                    (g / weight_sum) as u8,
                    (b / weight_sum) as u8,
                    255,
                ]),
            );
        }
    }

    *imgbuf = temp;
    Ok(())
}
