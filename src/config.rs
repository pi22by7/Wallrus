// config.rs

use std::env;

/// Structure to hold configuration details.
pub struct Config {
    pub unsplash_access_key: String,
    pub image_path: String,
}

/// Loads environment variables and creates a `Config` instance.
pub fn load_config() -> Config {
    dotenv::dotenv().ok(); // Load .env file if present

    let unsplash_access_key = env::var("UNSPLASH_ACCESS_KEY")
        .expect("UNSPLASH_ACCESS_KEY not found in .env file or environment");
    let image_path =
        env::var("IMAGE_PATH").expect("IMAGE_PATH not found in .env file or environment");

    Config {
        unsplash_access_key,
        image_path,
    }
}
