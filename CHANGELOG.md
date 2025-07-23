# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-07-23

### Added

- **Hyprland Support**: Added full compatibility with Hyprland compositor
  - Supports hyprpaper via `hyprctl` commands (preferred method)
  - Falls back to swww for animated wallpapers
  - Basic fallback to swaybg for simple static wallpapers
- Hyprland detection via `XDG_CURRENT_DESKTOP` and `HYPRLAND_INSTANCE_SIGNATURE`
- Multi-method wallpaper setting approach for maximum compatibility

### Changed

- **Package renamed** from `wallchange` to `wallrus`
- Improved filename generation to avoid concatenation issues
- Enhanced error handling and fallback mechanisms
- Better wallpaper utility detection and usage

### Fixed

- Fixed filename generation creating `wallrusWallrus-` pattern
- Resolved all Rust compiler warnings and clippy suggestions
- Fixed module inception warning in config module
- Cleaned up redundant closures and unnecessary type casts
- Improved range checking with modern Rust patterns

### Technical Improvements

- Applied 24+ clippy automatic fixes for better code quality
- Optimized error handling with direct function references
- Removed unnecessary borrows and type conversions
- Enhanced code readability and maintainability

## [0.1.0] - Initial Release

### Added

- Wallpaper downloading from Unsplash API
- Support for GNOME, KDE, and XFCE desktop environments
- Slideshow functionality with configurable intervals
- Procedural wallpaper generation (gradient, random walk, scatter plot)
- Cross-platform support (Linux, macOS, Windows)
- CLI interface with subcommands for different operations
