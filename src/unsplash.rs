use crate::errors::{Result, WallrusError};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::path::Path;
use tokio::io::AsyncWriteExt;

use crate::engine::set_wallpaper;
use crate::utils::{generate_unique_filename, is_valid_file};

const UNSPLASH_SEARCH_URL: &str = "https://api.unsplash.com/search/photos";

/// Fetches an image URL from Unsplash based on the given criteria.
pub async fn fetch_unsplash_image_url(
    access_key: &str,
    query: Option<&str>,
    collection: Option<&str>,
    artist: Option<&str>,
) -> Result<String> {
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
        .map_err(|e| WallrusError::Config(e.to_string()))?;
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, header_value);

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| WallrusError::Network(e))?
        .json::<Value>()
        .await
        .map_err(|e| WallrusError::Network(e))?;

    Ok(res["results"][0]["urls"]["full"]
        .as_str()
        .ok_or_else(|| WallrusError::Config("No image URL found".to_string()))?
        .to_string())
}

/// Downloads the image from the given URL and saves it to the specified file path.
pub async fn download_image(image_url: &str, file_path: &str) -> Result<()> {
    let response = reqwest::get(image_url)
        .await
        .map_err(|e| WallrusError::Network(e))?;
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = tokio::fs::File::create(file_path)
        .await
        .map_err(|e| WallrusError::Io(e))?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| WallrusError::Network(e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| WallrusError::Io(e))?;
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}

pub async fn download_and_set_wallpaper(
    access_key: &str,
    query: Option<&str>,
    collection: Option<&str>,
    artist: Option<&str>,
    image_path: &str,
) -> Result<()> {
    println!("Fetching image URL from Unsplash... ");
    let image_url = fetch_unsplash_image_url(access_key, query, collection, artist).await?;

    let file_name = generate_unique_filename(image_path, "jpg");
    println!("Downloading image to {}... ", &file_name);
    download_image(&image_url, &file_name).await?;

    if !is_valid_file(&file_name) {
        return Err(WallrusError::ImageProcessing(
            "Downloaded file is not valid".to_string(),
        ));
    }

    let path = Path::new(&file_name);
    set_wallpaper(path)?;

    Ok(())
}
