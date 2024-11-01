use image::{ImageBuffer, Rgba, RgbaImage};
use plotters::prelude::*;
use rand::{thread_rng, Rng};
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

use crate::errors::{Result, WallrusError};
use crate::utils::generate_unique_filename;

/// Enum to specify the type of wallpaper to generate.
pub enum WallpaperType {
    Gradient,
    RandomWalk,
    RandomPlot,
}
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

/// Generates a wallpaper based on the specified type and saves it to the given file path.

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

fn generate_gradient_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = thread_rng();

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

fn generate_random_walk_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = thread_rng();
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

fn generate_random_plot_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let root = BitMapBackend::new(file_path, (width, height)).into_drawing_area();
    let mut rng = thread_rng();

    let base_hue = rng.gen_range(0..360);
    let plot_color = hsv_to_rgb(base_hue, 0.7, 0.8);
    let background_color = hsv_to_rgb((base_hue + 30) % 360, 0.5, 1.0);

    root.fill(&RGBColor(
        background_color.0,
        background_color.1,
        background_color.2,
    ))
    .map_err(|e| WallrusError::ImageProcessing(format!("Failed to fill background: {}", e)))?;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..width as i32, 0..height as i32)
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to build chart: {}", e)))?;

    chart
        .configure_mesh()
        .disable_mesh()
        .disable_axes()
        .draw()
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to configure chart: {}", e)))?;

    for _ in 0..1000 {
        let data: Vec<(i32, i32)> = (0..100)
            .map(|_| {
                (
                    rng.gen_range(0..width as i32),
                    rng.gen_range(0..height as i32),
                )
            })
            .collect();

        chart
            .draw_series(PointSeries::of_element(
                data,
                1,
                &RGBColor(plot_color.0, plot_color.1, plot_color.2),
                &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
            ))
            .map_err(|e| WallrusError::ImageProcessing(format!("Failed to draw series: {}", e)))?;
    }

    root.present()
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to save plot: {}", e)))?;

    println!("Random plot wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path)?;
    Ok(())
}

fn hsv_to_rgb(hue: i32, saturation: f32, value: f32) -> (u8, u8, u8) {
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

/// Sets the given image as the desktop wallpaper.
pub fn set_wallpaper(image_path: &Path) -> Result<()> {
    if !image_path.exists() {
        return Err(WallrusError::Config(format!(
            "Wallpaper file does not exist: {:?}",
            image_path
        )));
    }

    let wide_path: Vec<u16> = OsStr::new(image_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide_path.as_ptr() as *const _ as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result == 0 {
            return Err(WallrusError::Io(std::io::Error::last_os_error()));
        }
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
            set_wallpaper(&image_path)?;
            thread::sleep(interval);
        }
    }
}
