// unsplash.rs

use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::utils::{generate_unique_filename, is_valid_file};
use crate::wallpaper::set_wallpaper;

const UNSPLASH_SEARCH_URL: &str = "https://api.unsplash.com/search/photos";

/// Fetches an image URL from Unsplash based on the given criteria.
pub async fn fetch_unsplash_image_url(
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

/// Downloads the image from the given URL and saves it to the specified file path.
pub async fn download_image(image_url: &str, file_path: &str) -> Result<(), io::Error> {
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
        File::create(file_path)?.write_all(&image_data)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Response is not an image",
        ));
    }

    Ok(())
}

pub async fn download_and_set_wallpaper(
    access_key: &str,
    query: Option<&str>,
    collection: Option<&str>,
    artist: Option<&str>,
    image_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the image URL from Unsplash
    let image_url = fetch_unsplash_image_url(access_key, query, collection, artist).await?;

    // Define the path where the image will be saved
    let file_name = generate_unique_filename(&image_path, "jpg");
    print!("Downloading image to {}... ", &file_name);
    // Download the image
    download_image(&image_url, &file_name).await?;

    if !is_valid_file(&file_name) {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Downloaded file is not valid",
        )));
    }

    // Set the image as wallpaper (using a function from wallpaper.rs)
    let path = Path::new(&file_name);
    set_wallpaper(path);

    Ok(())
}
