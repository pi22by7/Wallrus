use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Download a new wallpaper from Unsplash
    Download {
        /// Search keyword for the wallpaper
        #[arg(long)]
        keyword: Option<String>,

        /// Collection ID to search within
        #[arg(long)]
        collection: Option<String>,

        /// Artist username to filter by
        #[arg(long)]
        artist: Option<String>,
    },

    /// Start a slideshow of wallpapers
    Slideshow {
        /// Interval between wallpaper changes in seconds
        #[arg(long, default_value_t = 5)]
        interval: u64,
    },

    /// Generate a new wallpaper
    Generate {
        /// Width of the generated wallpaper
        #[arg(long, default_value_t = 1920)]
        width: u32,

        /// Height of the generated wallpaper
        #[arg(long, default_value_t = 1080)]
        height: u32,
    },
}
