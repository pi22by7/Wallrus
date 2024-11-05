mod cli;
mod config;
mod engine;
mod errors;
mod unsplash;
mod utils;

use clap::Parser;
use std::time::Duration;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load and validate configuration
    let config = Config::load()?;

    match cli.command {
        Commands::Download {
            keyword,
            collection,
            artist,
        } => {
            println!("Downloading wallpaper...");
            unsplash::download_and_set_wallpaper(
                &config.unsplash_access_key,
                keyword.as_deref(),
                collection.as_deref(),
                artist.as_deref(),
                &config.image_path,
            )
            .await?;
        }
        Commands::Slideshow { interval } => {
            println!("Starting slideshow...");
            engine::create_slideshow(&config.image_path, Duration::from_secs(interval))?;
        }
        Commands::Generate { width, height } => {
            println!("Generating wallpaper...");
            engine::generate_wallpaper(width, height, &config.image_path)?;
        }
    }

    Ok(())
}
