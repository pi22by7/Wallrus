use dotenv::dotenv;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use winapi::um::winuser::{
    SystemParametersInfoA, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

const UNSPLASH_SEARCH_URL: &str = "https://api.unsplash.com/search/photos";
const USE_EXISTING_IMAGE: bool = false;

async fn fetch_unsplash_image_url(
    access_key: &str,
    query: Option<&str>,
    collection: Option<&str>,
    artist: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let mut url = UNSPLASH_SEARCH_URL.to_string();

    // Append query parameters
    if let Some(query) = query {
        url.push_str("?query=");
        url.push_str(query);
    }
    if let Some(collection) = collection {
        url.push_str("&collections=");
        url.push_str(collection);
    }
    if let Some(artist) = artist {
        url.push_str("&username=");
        url.push_str(artist);
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
    println!("Enter search keyword (or press enter to skip):");
    let keyword = stdin.lock().lines().next().unwrap().unwrap_or_default();

    println!("Enter collection ID (or press enter to skip):");
    let collection = stdin.lock().lines().next().unwrap().unwrap_or_default();

    println!("Enter artist username (or press enter to skip):");
    let artist = stdin.lock().lines().next().unwrap().unwrap_or_default();

    // converts String to &str
    let keyword_option = keyword.as_str().into();
    let collection_option = collection.as_str().into();
    let artist_option = artist.as_str().into();

    if !Path::new(&image_path).exists() || !USE_EXISTING_IMAGE {
        match fetch_unsplash_image_url(
            &unsplash_access_key,
            keyword_option,
            collection_option,
            artist_option,
        )
        .await
        {
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
