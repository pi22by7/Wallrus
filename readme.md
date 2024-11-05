## Command Overview

```
wallrus <command> [options]

Commands:
  download    Download wallpapers
  slideshow   Start wallpaper slideshow
  generate    Generate custom wallpaper
```

### Download Command

```
wallrus download [options]

Options:
  --keyword <keyword>       Download wallpapers matching keyword
  --collection <id>         Download from specific collection
  --artist <username>       Download from specific artist
```

Examples:

```bash
# Download nature wallpapers
wallrus download --keyword nature

# Download from collection and artist
wallrus download --collection 123 --artist johndoe
```

### Slideshow Command

```
wallrus slideshow [options]

Options:
  --interval <seconds>      Time between wallpaper changes (default: 60)
```

Example:

```bash
# Start slideshow with 10 second intervals
wallrus slideshow --interval 10
```

### Generate Command

```
wallrus generate [options]

Options:
  --width <pixels>         Output width in pixels
  --height <pixels>        Output height in pixels
```

Example:

```bash
# Generate 2560x1440 wallpaper
wallrus generate --width 2560 --height 1440
```

## Help Commands

Get help for any command:

```bash
wallrus --help                # General help
wallrus <command> --help      # Command-specific help
```
