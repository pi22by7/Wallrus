// Add serde and serde_json to your Cargo.toml dependencies
use dotenv::dotenv;
use reqwest;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use winapi::um::winuser::{
    SystemParametersInfoA, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

const UNSPLASH_URL: &str = "https://api.unsplash.com/photos/random?client_id=";
const USE_EXISTING_IMAGE: bool = false;

async fn fetch_unsplash_image_url(access_key: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}{}", UNSPLASH_URL, access_key);
    let res = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(res["urls"]["full"].as_str().unwrap_or_default().to_string())
}

async fn download_image(image_url: &str) -> Result<Vec<u8>, io::Error> {
    let response = reqwest::get(image_url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map_or(false, |v| v.to_str().unwrap_or("").starts_with("image/"))
    {
        Ok(response
            .bytes()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .to_vec())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Response is not an image",
        ))
    }
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
            // SystemParametersInfoA failed
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

    if !Path::new(&image_path).exists() || !USE_EXISTING_IMAGE {
        match fetch_unsplash_image_url(&unsplash_access_key).await {
            Ok(image_url) => match download_image(&image_url).await {
                Ok(image_data) => match File::create(image_path.clone()) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(&image_data) {
                            println!("Failed to write image data: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Failed to create file: {}", e);
                        return;
                    }
                },
                Err(e) => {
                    println!("Error downloading image: {}", e);
                    return;
                }
            },
            Err(e) => {
                println!("Error fetching image URL: {}", e);
                return;
            }
        }
    }

    set_wallpaper(&image_path);
    println!("Wallpaper updated!");
}
