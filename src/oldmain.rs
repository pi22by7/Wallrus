use dotenv::dotenv;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread, time::Duration};
use winapi::um::winuser::{
    SystemParametersInfoA, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};
extern crate image;

use image::{ImageBuffer, Rgba};

const UNSPLASH_SEARCH_URL: &str = "https://api.unsplash.com/search/photos";
const USE_EXISTING_IMAGE: bool = false;

fn generate_wallpaper(
    width: u32,
    height: u32,
    start_color: (u8, u8, u8, u8),
    end_color: (u8, u8, u8, u8),
    file_path: &Path,
) {
    let (start_r, start_g, start_b, start_a) = start_color;
    let (end_r, end_g, end_b, end_a) = end_color;

    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = start_r as f32 + (end_r as f32 - start_r as f32) * (x as f32 / width as f32);
        let g = start_g as f32 + (end_g as f32 - start_g as f32) * (x as f32 / width as f32);
        let b = start_b as f32 + (end_b as f32 - start_b as f32) * (x as f32 / width as f32);
        let a = start_a as f32 + (end_a as f32 - start_a as f32) * (x as f32 / width as f32);
        *pixel = Rgba([r as u8, g as u8, b as u8, a as u8]);
    }

    imgbuf.save(file_path).unwrap();
}

fn create_slideshow(image_directory: &str, interval: Duration) {
    let paths = fs::read_dir(image_directory).unwrap();
    for path in paths {
        let image_path = path.unwrap().path();
        set_wallpaper(image_path.to_str().unwrap());
        thread::sleep(interval);
    }
}

async fn fetch_unsplash_image_url(
    access_key: &str,
    query: Option<&str>,
    collection: Option<&str>,
    artist: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let mut url = UNSPLASH_SEARCH_URL.to_string();

    url.push_str("?query=");
    url.push_str(query.unwrap_or("nature"));

    if let Some(collection) = collection {
        if !collection.is_empty() {
            url.push_str("&collections=");
            url.push_str(collection);
        }
    }

    if let Some(artist) = artist {
        if !artist.is_empty() {
            url.push_str("&username=");
            url.push_str(artist);
        }
    }

    let header_value = HeaderValue::from_str(&format!("Client-ID {}", access_key))
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, header_value);

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res["results"][0]["urls"]["full"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}

async fn download_image(image_url: &str, file_name: &str) -> Result<(), io::Error> {
    let response = reqwest::get(image_url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map_or(false, |v| v.to_str().unwrap_or("").starts_with("image/"))
    {
        let image_data = response
            .bytes()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .to_vec();
        File::create(file_name)?.write_all(&image_data)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Response is not an image",
        ));
    }

    Ok(())
}

fn set_wallpaper(image_path: &str) {
    unsafe {
        let result = SystemParametersInfoA(
            SPI_SETDESKWALLPAPER,
            0,
            image_path as *const _ as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result == 0 {
            // here if SystemParametersInfoA fails [not confirmed]
            println!(
                "Failed to set wallpaper. Error code: {}",
                io::Error::last_os_error()
            );
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let unsplash_access_key =
        env::var("UNSPLASH_ACCESS_KEY").expect("UNSPLASH_ACCESS_KEY not found in .env file");
    let image_path = env::var("IMAGE_PATH").expect("IMAGE_PATH not found in .env file");

    let stdin = io::stdin();

    println!("Choose an option: \n1. Download new wallpaper\n2. Start slideshow\n3. Generate one");
    let choice = stdin.lock().lines().next().unwrap().unwrap_or_default();

    match choice.as_str() {
        "1" => {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let unique_file_name =
                format!("{}Wallrus-{}.jpg", image_path, since_the_epoch.as_secs());

            println!("Enter search keyword (or press enter to skip):");
            let keyword = stdin.lock().lines().next().unwrap().unwrap_or_default();

            println!("Enter collection ID (or press enter to skip):");
            let collection = stdin.lock().lines().next().unwrap().unwrap_or_default();

            println!("Enter artist username (or press enter to skip):");
            let artist = stdin.lock().lines().next().unwrap().unwrap_or_default();

            // converts String to &str
            let keyword_option = if keyword.is_empty() {
                None
            } else {
                Some(keyword.as_str())
            };
            let collection_option = if collection.is_empty() {
                None
            } else {
                Some(collection.as_str())
            };
            let artist_option = if artist.is_empty() {
                None
            } else {
                Some(artist.as_str())
            };

            if !Path::new(&unique_file_name).exists() || !USE_EXISTING_IMAGE {
                match fetch_unsplash_image_url(
                    &unsplash_access_key,
                    keyword_option,
                    collection_option,
                    artist_option,
                )
                .await
                {
                    Ok(image_url) => match download_image(&image_url, &unique_file_name).await {
                        Ok(_) => println!("Image downloaded and saved as {}", unique_file_name),
                        Err(e) => println!("Error downloading image: {}", e),
                    },
                    Err(e) => println!("Error fetching image URL: {}", e),
                }
            }

            set_wallpaper(&unique_file_name);
            println!("Wallpaper updated!");
        }
        "2" => {
            let slideshow_interval = Duration::from_secs(5);
            create_slideshow(&image_path, slideshow_interval);
        }
        "3" => {
            let file_path = Path::new(&image_path).join("wallrus.jpg");
            generate_wallpaper(1920, 1080, (255, 0, 0, 255), (0, 0, 255, 255), &file_path);
            println!("Wallpaper generated at {:?}", file_path);
        }
        _ => println!("Invalid option."),
    }
}
