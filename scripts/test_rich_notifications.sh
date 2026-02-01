#!/usr/bin/env bash

# Test script for COSMIC Rich Notifications
# Tests various notification types to verify rich notification functionality

set -e

echo "=== COSMIC Rich Notifications Test Suite ==="
echo ""

# Check if notify-send is available
if ! command -v notify-send &> /dev/null; then
    echo "Error: notify-send not found. Please install libnotify-bin"
    exit 1
fi

echo "Testing basic notifications..."
sleep 1

# Test 1: Basic notification
echo "[1/12] Basic text notification"
notify-send "Basic Notification" "This is a simple notification with just text."
sleep 2

# Test 2: Notification with icon
echo "[2/12] Notification with icon"
notify-send -i dialog-information "Info Notification" "This notification has an info icon."
sleep 2

# Test 3: Low urgency notification
echo "[3/12] Low urgency notification"
notify-send -u low "Low Urgency" "This notification has low urgency (should be styled differently)."
sleep 2

# Test 4: Normal urgency notification
echo "[4/12] Normal urgency notification"
notify-send -u normal "Normal Urgency" "This notification has normal urgency (default)."
sleep 2

# Test 5: Critical urgency notification
echo "[5/12] Critical urgency notification"
notify-send -u critical "Critical Urgency" "This notification has critical urgency (should stand out)."
sleep 2

# Test 6: Notification with body text
echo "[6/12] Notification with body text"
notify-send "Summary Title" "This is the body text of the notification. It can be longer and contain more details about what happened."
sleep 2

# Test 7: Notification with action buttons (if supported)
echo "[7/12] Notification with action buttons"
notify-send "Action Test" "This notification should have action buttons." \
    --action="default=Open" \
    --action="dismiss=Dismiss" 2>/dev/null || \
    notify-send "Action Test" "This notification would have action buttons (not supported by notify-send, but will work via DBus)."
sleep 2

# Test 8: Notification with links in body
echo "[8/12] Notification with clickable links"
notify-send "Link Test" "Check out https://github.com/pop-os/cosmic-notifications for more info. Links should be clickable!"
sleep 2

# Test 9: Notification with category
echo "[9/12] Notification with category (email)"
notify-send -c email "New Email" "You have a new message from example@test.com"
sleep 2

# Test 10: Notification with category (im)
echo "[10/12] Notification with category (instant message)"
notify-send -c im.received "New Message" "Alice: Hey, are you there?"
sleep 2

# Test 11: Notification with image path (if test image exists)
echo "[11/12] Notification with image"
if [ -f "/usr/share/pixmaps/debian-logo.png" ]; then
    notify-send -i /usr/share/pixmaps/debian-logo.png "Image Test" "This notification should display an image."
elif [ -f "/usr/share/icons/hicolor/48x48/apps/firefox.png" ]; then
    notify-send -i /usr/share/icons/hicolor/48x48/apps/firefox.png "Image Test" "This notification should display an image."
else
    notify-send "Image Test" "No test image found, but images are supported via DBus hints."
fi
sleep 2

# Test 12: Transient notification
echo "[12/12] Transient notification"
notify-send -t 3000 "Transient" "This notification expires in 3 seconds."
sleep 4

echo ""
echo "=== Basic tests complete ==="
echo ""
echo "Advanced tests (require custom DBus calls):"
echo "- Progress bar notifications (use 'value' hint)"
echo "- Image data notifications (use 'image-data' hint)"
echo "- Animated GIF notifications (use 'image-path' with GIF file)"
echo "- Multiple action buttons (use 'actions' parameter)"
echo ""
echo "To test these features, use a DBus tool or application that supports"
echo "the full freedesktop notification specification."
echo ""
echo "Example DBus command for progress notification:"
echo 'dbus-send --session --print-reply --dest=org.freedesktop.Notifications \'
echo '  /org/freedesktop/Notifications org.freedesktop.Notifications.Notify \'
echo '  string:"TestApp" uint32:0 string:"" string:"Download Progress" \'
echo '  string:"Downloading file..." array:string: \'
echo '  dict:string:variant:"value",variant:int32:45 int32:5000'
echo ""
