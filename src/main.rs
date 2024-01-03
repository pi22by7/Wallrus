// main.rs

mod config;
mod unsplash;
mod utils;
mod wallpaper;

use std::env;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() {
    config::load_config();

    let unsplash_access_key =
        env::var("UNSPLASH_ACCESS_KEY").expect("UNSPLASH_ACCESS_KEY not found in .env file");
    let image_path = env::var("IMAGE_PATH").expect("IMAGE_PATH not found in .env file");

    let stdin = io::stdin();
    println!("Choose an option: \n1. Download new wallpaper\n2. Start slideshow\n3. Generate one");
    let choice = stdin.lock().lines().next().unwrap().unwrap_or_default();

    match choice.as_str() {
        "1" => {
            let stdin = io::stdin();
            println!("Enter search keyword (or press enter to skip):");
            let keyword = stdin.lock().lines().next().unwrap().unwrap_or_default();

            println!("Enter collection ID (or press enter to skip):");
            let collection = stdin.lock().lines().next().unwrap().unwrap_or_default();

            println!("Enter artist username (or press enter to skip):");
            let artist = stdin.lock().lines().next().unwrap().unwrap_or_default();

            // Wrap the strings in Option and pass a reference to these options
            let keyword_option = if keyword.is_empty() {
                None
            } else {
                Some(keyword)
            };
            let collection_option = if collection.is_empty() {
                None
            } else {
                Some(collection)
            };
            let artist_option = if artist.is_empty() {
                None
            } else {
                Some(artist)
            };

            if let Err(e) = unsplash::download_and_set_wallpaper(
                &unsplash_access_key,
                utils::str_option_to_slice(&keyword_option),
                utils::str_option_to_slice(&collection_option),
                utils::str_option_to_slice(&artist_option),
                &image_path,
            )
            .await
            {
                println!("Error: {}", e);
            }
        }
        "2" => {
            wallpaper::create_slideshow(&image_path, Duration::from_secs(5));
        }
        "3" => {
            let file_path = Path::new(&image_path);
            wallpaper::generate_wallpaper(
                1920,
                1080,
                (255, 0, 0, 255),
                (0, 0, 255, 255),
                &image_path,
            );
            println!("Wallpaper generated at {:?}", file_path);
        }
        _ => println!("Invalid option."),
    }
}
