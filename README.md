# Wallrus 🦭

A fast, cross-platform wallpaper management tool written in Rust. Download wallpapers from Unsplash, generate procedural wallpapers, and run automated slideshows across Linux, macOS, and Windows.

## Features

🎨 **Multiple Wallpaper Sources**
- Download from Unsplash API with keyword/artist/collection filters
- Generate procedural wallpapers (gradients, random walks, scatter plots)
- Automated slideshow from local image directories

✨ **Cross-Platform Support**
- **Linux**: GNOME, KDE, XFCE, Hyprland, and other desktop environments
- **macOS**: Native system integration
- **Windows**: Native system integration

⚡ **Fast & Lightweight**
- Written in Rust for performance and reliability
- Minimal resource usage
- Single binary with no external dependencies

## Installation

### Quick Install (Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/yourusername/wallrus/main/install.sh | sh
```

### Package Managers

#### Cargo (Rust)
```bash
cargo install wallrus
```

#### Arch Linux (AUR)
```bash
yay -S wallrus
# or
paru -S wallrus
```

#### Homebrew (macOS)
```bash
brew install wallrus
```

#### From GitHub Releases
Download pre-built binaries from [Releases](https://github.com/yourusername/wallrus/releases).

### Build from Source

```bash
git clone https://github.com/yourusername/wallrus.git
cd wallrus
cargo build --release
```

The binary will be available at `target/release/wallrus`.

### Prerequisites

- For Unsplash downloads: [Unsplash API access key](https://unsplash.com/developers)

## Configuration

### Environment Variables

Create a `.env` file in the project directory or set environment variables:

```bash
# Required for downloading from Unsplash
UNSPLASH_ACCESS_KEY=your_unsplash_access_key_here

# Directory to save wallpapers (will be created if it doesn't exist)
IMAGE_PATH=/path/to/wallpaper/directory
```

### Hyprland Setup

Wallrus automatically detects Hyprland and tries wallpaper utilities in this order:

1. **hyprpaper** (preferred) - `hyprctl hyprpaper wallpaper`
2. **swww** (fallback) - `swww img`
3. **swaybg** (basic) - `swaybg -i`

Make sure at least one of these utilities is installed:

```bash
# Arch Linux
sudo pacman -S hyprpaper swww swaybg

# Or install manually from their respective repositories
```

## Usage

### Download Wallpapers

```bash
# Download a random nature wallpaper
wallrus download

# Search by keyword
wallrus download --keyword "mountains"

# Download from specific artist
wallrus download --artist "johndoe"

# Download from collection
wallrus download --collection "123456"
```

### Generate Wallpapers

```bash
# Generate a random procedural wallpaper
wallrus generate

# Specify dimensions (defaults to 1920x1080)
wallrus generate --width 2560 --height 1440
```

### Slideshow

```bash
# Start slideshow from directory (changes every 60 seconds)
wallrus slideshow /path/to/images

# Custom interval (in seconds)
wallrus slideshow /path/to/images --interval 30
```

## Supported Desktop Environments

| Environment | Status | Method |
|-------------|--------|---------|
| GNOME/Unity | ✅ | `gsettings` |
| KDE Plasma | ✅ | `qdbus` |
| XFCE | ✅ | `xfconf-query` |
| Hyprland | ✅ | `hyprctl`/`swww`/`swaybg` |
| macOS | ✅ | Native APIs |
| Windows | ✅ | Native APIs |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development

```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Unsplash](https://unsplash.com/) for providing the wallpaper API
- The Rust community for excellent crates and tools
- Hyprland and wlroots communities for Wayland compositor development