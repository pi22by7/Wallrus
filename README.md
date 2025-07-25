# Wallrus 🦭

A fast, cross-platform wallpaper management tool with **native Wayland protocol support**. Download wallpapers from Unsplash, generate procedural wallpapers, and run automated slideshows across Linux, macOS, and Windows.

## Features

🎨 **Multiple Wallpaper Sources**
- Download from Unsplash API with keyword/artist/collection filters
- Generate procedural wallpapers (gradients, random walks, scatter plots)
- Automated slideshow from local image directories

🖥️ **Native Wayland Protocol Support**
- **Standalone wallpaper setting** - No external dependencies required
- **Direct wlr-layer-shell integration** using smithay-client-toolkit
- **Multi-monitor support** with per-output surface management
- **Smart fallback system** to external tools when needed

✨ **Cross-Platform Support**
- **Linux**: GNOME, KDE, XFCE, Hyprland, and other desktop environments
- **macOS**: Native system integration
- **Windows**: Native system integration

⚡ **Fast & Lightweight**
- Written in Rust for performance and reliability
- Minimal resource usage
- Native protocol implementation eliminates external tool dependencies

## Installation

### Quick Install (Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/pi22by7/wallrus/main/install.sh | sh
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
Download pre-built binaries from [Releases](https://github.com/pi22by7/wallrus/releases).

### Build from Source

```bash
git clone https://github.com/pi22by7/wallrus.git
cd wallrus
# Build with native Wayland support (default)
cargo build --release --features wayland
```

The binary will be available at `target/release/wallrus`.

#### Build Options

- **Default (Wayland enabled)**: `cargo build --release`
- **Wayland disabled**: `cargo build --release --no-default-features`

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

### Wayland/Hyprland Setup

Wallrus has **native Wayland protocol support** and automatically detects Hyprland. It tries methods in this order:

1. 🚀 **Native wlr-layer-shell protocol** (preferred) - Built-in, no dependencies
2. **hyprpaper** (fallback) - `hyprctl hyprpaper wallpaper`
3. **swww** (fallback) - `swww img`
4. **swaybg** (fallback) - `swaybg -i`

**No external tools required!** But you can optionally install fallbacks:

```bash
# Arch Linux
sudo pacman -S hyprpaper swww swaybg

# Or install manually from their respective repositories
```

#### Important Notes

- **Native mode**: Process stays alive to maintain wallpaper (layer surfaces require active client)
- **Daemon conflicts**: Stop `swww-daemon` before using native mode: `pkill swww-daemon`
- **Process management**: Use Ctrl+C to exit and remove wallpaper when using native mode

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
| **Hyprland** | ✅ | **Native wlr-layer-shell protocol** |
| Other Wayland | ✅ | **Native wlr-layer-shell protocol** |
| GNOME/Unity | ✅ | `gsettings` |
| KDE Plasma | ✅ | `qdbus` |
| XFCE | ✅ | `xfconf-query` |
| macOS | ✅ | Native APIs |
| Windows | ✅ | Native APIs |

### Wayland Protocol Features

- **🎯 Direct protocol integration** - No external dependencies
- **🖥️ Multi-monitor support** - Automatic output detection
- **⚡ High performance** - Direct buffer management
- **🔄 Smart fallbacks** - External tools when needed
- **🎨 Full image scaling** - Proper aspect ratio handling

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