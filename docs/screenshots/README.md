# Screenshots

This directory contains screenshots demonstrating COSMIC Notifications features.

## Needed Screenshots

The following screenshots are needed to complete the documentation:

### 1. Basic Notification (`basic-notification.png`)
- Simple text notification
- Shows app icon, summary, and body text
- Default styling with normal urgency

**How to capture:**
```bash
notify-send "Welcome" "This is a basic notification"
# Take screenshot of notification
```

### 2. Rich Notification with Image (`rich-notification-image.png`)
- Notification displaying an embedded image
- Shows image thumbnail (64x64 inline)
- Full notification with app icon, summary, body, and image

**How to capture:**
```bash
notify-send -i /usr/share/pixmaps/debian-logo.png "System Update" "New updates are available"
# Take screenshot of notification
```

### 3. Notification with Action Buttons (`notification-actions.png`)
- Notification showing multiple action buttons
- Demonstrates button styling
- Shows button hover state (if possible)

**How to capture:**
```bash
# Requires D-Bus call (notify-send doesn't support multiple actions)
dbus-send --session --print-reply \
  --dest=org.freedesktop.Notifications \
  /org/freedesktop/Notifications \
  org.freedesktop.Notifications.Notify \
  string:"Email" uint32:0 string:"mail-unread" \
  string:"New Email" \
  string:"You have a new message from Alice" \
  array:string:"default","Open","delete","Delete","reply","Reply" \
  dict:string:variant:"category",variant:string:"email.arrived" \
  int32:10000
# Take screenshot of notification
```

### 4. Progress Notification (`notification-progress.png`)
- Shows progress bar at ~50%
- Download or upload notification
- Progress bar styling

**How to capture:**
```bash
dbus-send --session --print-reply \
  --dest=org.freedesktop.Notifications \
  /org/freedesktop/Notifications \
  org.freedesktop.Notifications.Notify \
  string:"Downloads" uint32:0 string:"folder-download" \
  string:"Downloading File" \
  string:"large-file.iso (512 MB)" \
  array:string: \
  dict:string:variant:"value",variant:int32:50 \
  int32:10000
# Take screenshot of notification
```

### 5. Animated GIF Notification (`notification-animated.gif`)
- Screen recording of animated GIF in notification
- Shows animation playing
- Should be actual GIF or short video

**How to capture:**
```bash
notify-send -i /path/to/animated.gif "Animation Test" "This notification contains an animated GIF"
# Record screen for 3-5 seconds to show animation
```

### 6. Urgency Levels Comparison (`urgency-levels.png`)
- Three notifications side-by-side or stacked
- Shows Low, Normal, and Critical urgency
- Demonstrates color differences

**How to capture:**
```bash
notify-send -u low "Low Priority" "This can wait"
sleep 1
notify-send -u normal "Normal" "Standard notification"
sleep 1
notify-send -u critical "Critical Alert" "Immediate attention required!"
# Take screenshot showing all three
```

### 7. Notification with Clickable Links (`notification-links.png`)
- Shows notification with link in body text
- Link should be visually distinct (underlined or colored)
- Cursor hovering over link (optional)

**How to capture:**
```bash
notify-send "Update Available" "Visit https://github.com/pop-os/cosmic for more information"
# Take screenshot, optionally with cursor over link
```

### 8. Multiple Notifications Stacked (`notification-stack.png`)
- 3-4 notifications displayed simultaneously
- Shows stacking behavior
- Different types of notifications

**How to capture:**
```bash
notify-send -i mail-unread "Email" "New message from Bob"
notify-send -i network-wireless "Network" "Connected to WiFi"
notify-send -i battery-low "Battery" "Battery at 15%"
notify-send -i dialog-information "Info" "System update available"
# Take screenshot of all notifications
```

## Screenshot Guidelines

### Technical Requirements
- **Resolution:** Minimum 1920x1080
- **Format:** PNG for static images, GIF for animations
- **DPI:** 96 DPI (standard)
- **Color space:** sRGB

### Capture Guidelines
- Use a clean desktop background (solid color or subtle pattern)
- Ensure good contrast between notification and background
- Capture full notification with slight padding around edges
- Use default COSMIC theme for consistency
- Avoid including personal information in screenshots

### Tools Recommended
- **Static screenshots:** GNOME Screenshot, Flameshot, or built-in screenshot tool
- **Animations:** Peek (GIF recorder) or SimpleScreenRecorder
- **Editing:** GIMP or Krita for any needed adjustments

## Integration into Documentation

Once captured, screenshots should be:

1. Added to this directory with descriptive filenames
2. Referenced in README.md:
   ```markdown
   ![Basic Notification](docs/screenshots/basic-notification.png)
   ```
3. Referenced in TESTING.md where relevant
4. Included in release announcements

## Future Screenshots

Consider adding:
- Dark theme variants
- Different notification layouts (portrait vs landscape)
- Accessibility features (high contrast, large text)
- Multi-monitor setups
- Notification history/center (when implemented)

## Contributing Screenshots

If you'd like to contribute screenshots:

1. Follow the guidelines above
2. Capture high-quality screenshots showing the feature clearly
3. Submit via pull request with descriptive commit message
4. Include a brief description of your setup (theme, resolution, etc.)

Thank you for helping improve the documentation!
