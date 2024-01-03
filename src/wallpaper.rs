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

use crate::utils::generate_unique_filename;

/// Enum to specify the type of wallpaper to generate.
pub enum WallpaperType {
    Gradient,
    RandomWalk,
    RandomPlot,
}

/// Generates a wallpaper based on the specified type and saves it to the given file path.
pub fn generate_wallpaper(width: u32, height: u32, file_path: &str) {
    let file_path = generate_unique_filename(file_path, "jpg");
    let file_path = Path::new(&file_path);

    let wallpaper_type = match thread_rng().gen_range(0..3) {
        0 => WallpaperType::Gradient,
        // 1 => WallpaperType::RandomPLot,
        _ => WallpaperType::RandomWalk,
    };

    match wallpaper_type {
        WallpaperType::Gradient => generate_gradient_wallpaper(width, height, &file_path),
        WallpaperType::RandomWalk => generate_random_walk_wallpaper(width, height, &file_path),
        WallpaperType::RandomPlot => generate_random_plot_wallpaper(width, height, &file_path),
    }
}

fn generate_gradient_wallpaper(width: u32, height: u32, file_path: &Path) {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = thread_rng();

    // Generate a base hue for the gradient
    let base_hue = rng.gen_range(0..360);

    // Create start and end colors using analogous color scheme
    let start_color = hsv_to_rgb(base_hue, 0.7, 0.8);
    let end_color = hsv_to_rgb((base_hue + 30) % 360, 0.7, 0.8); // 30 degrees apart

    for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = start_color.0 as f32
            + (end_color.0 as f32 - start_color.0 as f32) * (x as f32 / width as f32);
        let g = start_color.1 as f32
            + (end_color.1 as f32 - start_color.1 as f32) * (x as f32 / width as f32);
        let b = start_color.2 as f32
            + (end_color.2 as f32 - start_color.2 as f32) * (x as f32 / width as f32);
        *pixel = Rgba([r as u8, g as u8, b as u8, 255]);
    }

    imgbuf.save(file_path).unwrap();
    println!("Gradient wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path);
}

fn generate_random_walk_wallpaper(width: u32, height: u32, file_path: &Path) {
    let mut imgbuf: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = thread_rng();

    // Generate a single random pastel background color
    let base_hue = rng.gen_range(0..360);

    // Create background and line colors using analogous color scheme
    let background_color = hsv_to_rgb(base_hue, 0.5, 1.0);
    let line_color = hsv_to_rgb((base_hue + 30) % 360, 0.7, 0.8);

    // Apply the background color uniformly
    for pixel in imgbuf.pixels_mut() {
        *pixel = Rgba([
            background_color.0,
            background_color.1,
            background_color.2,
            255,
        ]);
    }

    let (mut x, mut y) = (width / 2, height / 2);
    let step_size = 5;
    let line_thickness = 3;

    for _ in 0..5000 {
        let dx = rng.gen_range(-step_size..=step_size);
        let dy = rng.gen_range(-step_size..=step_size);
        x = ((x as i32 + dx).max(0).min(width as i32 - 1)) as u32;
        y = ((y as i32 + dy).max(0).min(height as i32 - 1)) as u32;

        // Draw thicker lines
        for i in 0..line_thickness {
            for j in 0..line_thickness {
                let xi = ((x as i32 + i - line_thickness / 2)
                    .max(0)
                    .min(width as i32 - 1)) as u32;
                let yj = ((y as i32 + j - line_thickness / 2)
                    .max(0)
                    .min(height as i32 - 1)) as u32;
                imgbuf.put_pixel(
                    xi,
                    yj,
                    Rgba([line_color.0, line_color.1, line_color.2, 255]),
                );
            }
        }
    }

    imgbuf.save(file_path).unwrap();
    println!("Random walk wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path)
}

fn generate_random_plot_wallpaper(width: u32, height: u32, file_path: &Path) {
    let root = BitMapBackend::new(file_path, (width, height)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Random Plot", ("sans-serif", 50).into_font())
        .build_cartesian_2d(0..width as i32, 0..height as i32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let mut rng = thread_rng();
    for _ in 0..1000 {
        chart
            .draw_series(PointSeries::of_element(
                (0..100).map(|x| (x, rng.gen_range(0..height as i32))),
                1,
                &RED,
                &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
            ))
            .unwrap();
    }

    println!("Random plot wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path);
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
pub fn set_wallpaper(image_path: &Path) {
    if !image_path.exists() {
        println!("Wallpaper file does not exist: {:?}", image_path);
        return;
    }

    let wide_path: Vec<u16> = OsStr::new(image_path)
        .encode_wide()
        .chain(std::iter::once(0)) // Append null terminator
        .collect();

    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide_path.as_ptr() as *const _ as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result == 0 {
            println!(
                "Failed to set wallpaper. Error code: {}",
                std::io::Error::last_os_error()
            );
        }
    }
}

/// Creates a slideshow from images in a specified directory, changing wallpaper at a given interval.
pub fn create_slideshow(image_directory: &str, interval: Duration) {
    let paths = fs::read_dir(image_directory).unwrap();
    for path in paths {
        let image_path = path.unwrap().path();
        set_wallpaper(&image_path);
        thread::sleep(interval);
    }
}
