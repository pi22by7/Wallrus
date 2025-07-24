# Systemd Service for Wallrus

This directory contains systemd service files for running Wallrus as a background service.

## Slideshow Service

The `wallrus-slideshow.service` runs a wallpaper slideshow in the background.

### Installation

1. Copy the service file to the systemd user directory:
```bash
mkdir -p ~/.config/systemd/user
cp systemd/wallrus-slideshow.service ~/.config/systemd/user/
```

2. Create environment file (optional):
```bash
mkdir -p ~/.config/wallrus
cat > ~/.config/wallrus/env << EOF
UNSPLASH_ACCESS_KEY=your_api_key_here
IMAGE_PATH=/home/$USER/Pictures/Wallpapers
EOF
```

3. Reload systemd and enable the service:
```bash
systemctl --user daemon-reload
systemctl --user enable wallrus-slideshow.service
systemctl --user start wallrus-slideshow.service
```

### Configuration

Edit the service file to customize:
- `ExecStart`: Change the wallpaper directory and interval
- `Environment`: Add environment variables directly
- `User`: Change the user (defaults to %i which is the instance name)

### Usage

```bash
# Start slideshow service for current user
systemctl --user start wallrus-slideshow.service

# Enable auto-start on login
systemctl --user enable wallrus-slideshow.service

# Check status
systemctl --user status wallrus-slideshow.service

# View logs
journalctl --user -u wallrus-slideshow.service -f

# Stop service
systemctl --user stop wallrus-slideshow.service
```

### Multiple Users

To run for a specific user:
```bash
sudo systemctl enable wallrus-slideshow@username.service
sudo systemctl start wallrus-slideshow@username.service
```