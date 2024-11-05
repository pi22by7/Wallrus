mod cli;
mod config;
mod engine;
mod errors;
mod providers;
mod utils;

use crate::cli::Cli;
use crate::errors::Result;
use config::config::Config;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse_args();

    // Load and validate configuration
    let config = Config::load()?;

    match cli.command {
        cli::Commands::Download {
            keyword,
            collection,
            artist,
        } => {
            println!("Downloading wallpaper...");
            providers::unsplash::download_and_set_wallpaper(
                &config.unsplash_access_key,
                keyword.as_deref(),
                collection.as_deref(),
                artist.as_deref(),
                &config.image_path,
            )
            .await?;
        }
        cli::Commands::Slideshow { interval } => {
            println!("Starting slideshow...");
            engine::create_slideshow(&config.image_path, Duration::from_secs(interval))?;
        }
        cli::Commands::Generate { width, height } => {
            println!("Generating wallpaper...");
            engine::generate_wallpaper(width, height, &config.image_path)?;
        }
    }

    Ok(())
}
