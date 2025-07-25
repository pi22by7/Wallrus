#!/bin/bash
set -e

# Wallrus Installation Script
# Supports Linux and macOS

REPO="pi22by7/wallrus"
BINARY_NAME="wallrus"
INSTALL_DIR="/usr/local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    
    case $OS in
        linux)
            case $ARCH in
                x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
                aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
                *) print_error "Unsupported architecture: $ARCH" ;;
            esac
            ARCHIVE_EXT="tar.gz"
            ;;
        darwin)
            case $ARCH in
                x86_64) TARGET="x86_64-apple-darwin" ;;
                arm64) TARGET="aarch64-apple-darwin" ;;
                *) print_error "Unsupported architecture: $ARCH" ;;
            esac
            ARCHIVE_EXT="tar.gz"
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            ;;
    esac
    
    print_status "Detected platform: $TARGET"
}

# Get latest release version
get_latest_version() {
    print_status "Fetching latest release..."
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
    if [ -z "$VERSION" ]; then
        print_error "Failed to fetch latest version"
    fi
    print_status "Latest version: $VERSION"
}

# Download and install
install_wallrus() {
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME-$TARGET.$ARCHIVE_EXT"
    TEMP_DIR=$(mktemp -d)
    
    print_status "Downloading from $DOWNLOAD_URL..."
    
    cd "$TEMP_DIR"
    curl -sL "$DOWNLOAD_URL" -o "$BINARY_NAME-$TARGET.$ARCHIVE_EXT"
    
    if [ "$ARCHIVE_EXT" = "tar.gz" ]; then
        tar -xzf "$BINARY_NAME-$TARGET.$ARCHIVE_EXT"
    fi
    
    # Check if we need sudo for installation
    if [ ! -w "$INSTALL_DIR" ]; then
        print_status "Installing to $INSTALL_DIR (requires sudo)..."
        sudo install -m 755 "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        print_status "Installing to $INSTALL_DIR..."
        install -m 755 "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    fi
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
    
    print_status "Installation complete!"
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        print_status "Wallrus installed successfully:"
        "$BINARY_NAME" --version
    else
        print_error "Installation failed - $BINARY_NAME not found in PATH"
    fi
}

# Main installation flow
main() {
    print_status "Starting Wallrus installation..."
    
    # Check dependencies
    if ! command -v curl >/dev/null 2>&1; then
        print_error "curl is required but not installed"
    fi
    
    detect_platform
    get_latest_version
    install_wallrus
    verify_installation
    
    echo
    print_status "Next steps:"
    echo "1. Set up your Unsplash API key: export UNSPLASH_ACCESS_KEY=your_key"
    echo "2. Set wallpaper directory: export IMAGE_PATH=/path/to/wallpapers"
    echo "3. Try: wallrus download --keyword nature"
    echo
    echo "For Hyprland users: Native Wayland protocol support included (no external tools required!)"
}

main "$@"