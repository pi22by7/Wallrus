// config.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

use crate::errors::{Result, WallrusError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub unsplash_access_key: String,
    pub image_path: String,
    #[serde(default = "default_image_quality")]
    pub image_quality: u8,
    #[serde(default = "default_slideshow_interval")]
    pub slideshow_interval: u64,
}

fn default_image_quality() -> u8 {
    80
}

fn default_slideshow_interval() -> u64 {
    5
}

impl Config {
    pub fn load() -> Result<Self> {
        let unsplash_access_key = env::var("UNSPLASH_ACCESS_KEY").map_err(|_| {
            WallrusError::Config("UNSPLASH_ACCESS_KEY not found in environment".into())
        })?;

        let image_path = env::var("IMAGE_PATH")
            .map_err(|_| WallrusError::Config("IMAGE_PATH not found in environment".into()))?;

        let config = Self {
            unsplash_access_key,
            image_path,
            image_quality: default_image_quality(),
            slideshow_interval: default_slideshow_interval(),
        };

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.unsplash_access_key.is_empty() {
            return Err(WallrusError::Config("Unsplash access key is empty".into()));
        }
        if !Path::new(&self.image_path).exists() {
            return Err(WallrusError::Config("Image path does not exist".into()));
        }
        if self.image_quality > 100 {
            return Err(WallrusError::Config(
                "Image quality must be between 0 and 100".into(),
            ));
        }
        Ok(())
    }
}
