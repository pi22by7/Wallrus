// wallpaper.rs

use image::{ImageBuffer, Rgba};
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

/// Generates a gradient wallpaper and saves it to the given file path.
pub fn generate_wallpaper(
    width: u32,
    height: u32,
    start_color: (u8, u8, u8, u8),
    end_color: (u8, u8, u8, u8),
    file_path: &str,
) {
    let file_path = generate_unique_filename(file_path, "jpg");
    let file_path = Path::new(&file_path);

    let (start_r, start_g, start_b, start_a) = start_color;
    let (end_r, end_g, end_b, end_a) = end_color;

    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = start_r as f32 + (end_r as f32 - start_r as f32) * (x as f32 / width as f32);
        let g = start_g as f32 + (end_g as f32 - start_g as f32) * (x as f32 / width as f32);
        let b = start_b as f32 + (end_b as f32 - start_b as f32) * (x as f32 / width as f32);
        let a = start_a as f32 + (end_a as f32 - start_a as f32) * (x as f32 / width as f32);
        *pixel = Rgba([r as u8, g as u8, b as u8, a as u8]);
    }

    imgbuf.save(file_path).unwrap();
    println!("Wallpaper generated at {:?}", file_path);

    set_wallpaper(file_path);
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
