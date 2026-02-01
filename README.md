# Cosmic Notifications NG

Enhanced Layer Shell notifications daemon for the COSMIC desktop environment, featuring **rich notification support** including images, action buttons, progress indicators, clickable URLs, and animated content.

## What Makes This Different

This is an enhanced fork of the standard COSMIC notifications daemon with significant improvements:

| Feature | Standard COSMIC | COSMIC Notifications NG |
|---------|-----------------|------------------------|
| **Image Support** | Basic icon display | Full image-path/image-data hints, auto-resizing, preview images |
| **Image Scaling** | Fixed sizes | Dynamic scaling up to 128x128 with proper aspect ratio |
| **Animated Images** | Not supported | GIF, APNG, WebP with frame timing (100 frames/30s limits) |
| **Clickable URLs** | Not supported | Auto-detection with secure http/https handling |
| **Progress Bars** | Basic | Smooth animated progress with value hints |
| **HTML Sanitization** | Limited | Full ammonia-based XSS protection |
| **Action Buttons** | Basic | Themed buttons with proper DBus signals |
| **Configuration** | Limited | Extensive TOML-based configuration |
| **NixOS Module** | Not provided | Full NixOS module with overlay support |

### Key Enhancements

1. **YouTube/Media Previews**: When apps send notifications with image hints, preview images display at proper size (128x128)
2. **Clickable Links**: URLs in notification body text become clickable buttons
3. **Better Image Handling**: Fixed image-path parsing to work with absolute file paths, not just file:// URLs
4. **Proper Icon Scaling**: Icons from `icon::from_raster_pixels()` now scale correctly using `.size()` method

## Features

### Rich Notification Support

COSMIC Notifications implements the full [freedesktop.org Notification Specification](https://specifications.freedesktop.org/notification-spec/) with enhanced features:

- **Image Support**
  - Display images from file paths (`image-path` hint)
  - Display images from raw data (`image-data`, `icon_data` hints)
  - Automatic image resizing (max 128x128, configurable)
  - Support for PNG, JPEG, and other common formats

- **Animated Images**
  - GIF, APNG, and WebP animation support
  - Smooth playback with proper frame timing
  - Memory-safe limits (100 frames max, 30s max duration)
  - Can be disabled via configuration

- **Action Buttons**
  - Multiple action buttons per notification
  - Default action support (click anywhere on notification)
  - Proper DBus ActionInvoked signal emission
  - Themed button styling

- **Progress Indicators**
  - Progress bar widget for download/upload notifications
  - Smooth animation on value updates
  - Supports `value` hint (0-100)

- **Clickable Links**
  - Automatic URL detection in notification body
  - Click to open in default browser
  - Security: only http:// and https:// URLs are clickable
  - Can be disabled via configuration

- **Urgency Levels**
  - Low, Normal, and Critical urgency styling
  - Different colors per urgency level
  - Visual distinction for important notifications

- **Category Support**
  - Category hints for notification types (email, IM, system, etc.)
  - Category-specific icons
  - Proper styling per category

- **HTML Sanitization**
  - Safe rendering of basic HTML tags (b, i, u, a)
  - Automatic removal of dangerous tags (script, iframe, etc.)
  - Protection against XSS attacks

### Configuration

Configure notification behavior via COSMIC Settings or directly in configuration files:

```toml
# Show images in notifications (default: true)
show_images = true

# Show action buttons (default: true)
show_actions = true

# Maximum image size in pixels (default: 128)
max_image_size = 128

# Enable clickable links (default: true)
enable_links = true

# Enable animated images (default: true)
enable_animations = true
```

### Performance

- **Target:** 30 FPS for animations
- **Memory:** < 100MB with multiple rich notifications
- **Efficiency:** Hardware-accelerated rendering via iced/wgpu

## Usage Examples

### Basic Notification

```bash
notify-send "Hello" "This is a basic notification"
```

### Notification with Icon

```bash
notify-send -i dialog-information "Information" "This notification has an icon"
```

### Notification with Urgency

```bash
# Low urgency (subtle styling)
notify-send -u low "Low Priority" "This can wait"

# Critical urgency (prominent styling)
notify-send -u critical "Alert" "This is important!"
```

### Progress Notification (via DBus)

```bash
dbus-send --session --print-reply \
  --dest=org.freedesktop.Notifications \
  /org/freedesktop/Notifications \
  org.freedesktop.Notifications.Notify \
  string:"MyApp" uint32:0 string:"" \
  string:"Download Progress" \
  string:"Downloading file..." \
  array:string: \
  dict:string:variant:"value",variant:int32:75 \
  int32:5000
```

### Notification with Actions (via DBus)

```bash
dbus-send --session --print-reply \
  --dest=org.freedesktop.Notifications \
  /org/freedesktop/Notifications \
  org.freedesktop.Notifications.Notify \
  string:"MyApp" uint32:0 string:"dialog-question" \
  string:"Confirmation" \
  string:"Do you want to proceed?" \
  array:string:"yes","Yes","no","No" \
  dict:string:variant: \
  int32:5000
```

### Testing

Run the test suite to verify all features:

```bash
./scripts/test_rich_notifications.sh
```

For detailed testing instructions with real applications (Firefox, Thunderbird, Spotify, etc.), see [TESTING.md](TESTING.md).

## NixOS Installation

### Flake-based Installation (Recommended)

Add this flake to your `flake.nix` inputs:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    cosmic-notifications-ng.url = "github:olafkfreund/cosmic-notifications-ng";
  };

  outputs = { self, nixpkgs, cosmic-notifications-ng, ... }: {
    nixosConfigurations.your-hostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        # Use the overlay to replace system cosmic-notifications
        { nixpkgs.overlays = [ cosmic-notifications-ng.overlays.default ]; }

        # Or use the NixOS module for more control
        cosmic-notifications-ng.nixosModules.default
        {
          services.cosmic-notifications-ng = {
            enable = true;
            settings = {
              show_images = true;
              show_actions = true;
              max_image_size = 128;
              enable_links = true;
              enable_animations = true;
            };
          };
        }
      ];
    };
  };
}
```

### Module Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enable` | bool | `false` | Enable the notification daemon |
| `package` | package | `pkgs.cosmic-notifications-ng` | Package to use |
| `replaceSystemPackage` | bool | `true` | Replace system cosmic-notifications via overlay |
| `settings.show_images` | bool | `true` | Show images in notifications |
| `settings.show_actions` | bool | `true` | Show action buttons |
| `settings.max_image_size` | int | `128` | Maximum image size in pixels |
| `settings.enable_links` | bool | `true` | Make URLs clickable |
| `settings.enable_animations` | bool | `true` | Enable GIF/APNG/WebP animations |

### Quick Overlay Installation

For a simple drop-in replacement without configuration:

```nix
{
  nixpkgs.overlays = [
    (final: prev: {
      cosmic-notifications = cosmic-notifications-ng.packages.${prev.system}.default;
    })
  ];
}
```

After rebuilding (`sudo nixos-rebuild switch`), restart the COSMIC panel to load the new daemon.

### Verify Installation

```bash
# Check which binary is running
ls -la /proc/$(pgrep -f cosmic-notifications | head -1)/exe

# Send a test notification with image
gdbus call --session \
  --dest org.freedesktop.Notifications \
  --object-path /org/freedesktop/Notifications \
  --method org.freedesktop.Notifications.Notify \
  "TestApp" 0 "dialog-information" \
  "Test Notification" \
  "Visit https://github.com for more info" \
  "[]" "{'urgency': <byte 1>}" 5000
```

# Building from Source

Cosmic Notifications is set up to build a deb and a Nix flake, but it can be built using just.

Some Build Dependencies:
```
  cargo,
  just,
  intltool,
  appstream-util,
  desktop-file-utils,
  libxkbcommon-dev,
  pkg-config,
  desktop-file-utils,
```

## Build Commands

For a typical install from source, use `just` followed with `sudo just install`.
```sh
just
sudo just install
```

If you are packaging, run `just vendor` outside of your build chroot, then use `just build-vendored` inside the build-chroot. Then you can specify a custom root directory and prefix.
```sh
# Outside build chroot
just clean-dist
just vendor

# Inside build chroot
just build-vendored
sudo just rootdir=debian/cosmic-notifications prefix=/usr install
```

# Translators

Translation files may be found in the i18n directory. New translations may copy the English (en) localization of the project and rename `en` to the desired [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes). Translations may be submitted through GitHub as an issue or pull request. Submissions by email or other means are also acceptable; with the preferred name and email to associate with the changes.

# Debugging & Profiling

## Profiling async tasks with tokio-console

To debug issues with asynchronous code, install [tokio-console](https://github.com/tokio-rs/console) and run it within a separate terminal. Then kill the **cosmic-notifications** process a couple times in quick succession to prevent **cosmic-session** from spawning it again. Then you can start **cosmic-notifications** with **tokio-console** support either by running `just tokio-console` from this repository to test code changes, or `env TOKIO_CONSOLE=1 cosmic-notifications` to enable it with the installed version of **cosmic-notifications**.