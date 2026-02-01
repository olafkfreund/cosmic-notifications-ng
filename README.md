# Cosmic Notifications

Layer Shell notifications daemon which integrates with COSMIC, featuring rich notification support including images, action buttons, progress indicators, and animated content.

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

# Building

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