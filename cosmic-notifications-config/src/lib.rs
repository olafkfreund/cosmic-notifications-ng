use cosmic_config::{CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};

pub const ID: &str = "com.system76.CosmicNotifications";

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Anchor {
    #[default]
    Top,
    Bottom,
    Right,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, CosmicConfigEntry)]
#[version = 2]
pub struct NotificationsConfig {
    pub do_not_disturb: bool,
    pub anchor: Anchor,
    /// The maximum number of notifications that can be displayed at once.
    pub max_notifications: u32,
    /// The maximum number of notifications that can be displayed per app if not urgent and constrained by `max_notifications`.
    pub max_per_app: u32,
    /// Max time in milliseconds a critical notification can be displayed before being removed.
    pub max_timeout_urgent: Option<u32>,
    /// Max time in milliseconds a normal notification can be displayed before being removed.
    pub max_timeout_normal: Option<u32>,
    /// Max time in milliseconds a low priority notification can be displayed before being removed.
    pub max_timeout_low: Option<u32>,

    // Rich notification configuration options
    /// Whether to display images in notifications (default: true)
    #[serde(default = "default_true")]
    pub show_images: bool,
    /// Whether to display action buttons in notifications (default: true)
    #[serde(default = "default_true")]
    pub show_actions: bool,
    /// Maximum width/height for notification images in pixels (default: 128, range: 32-256)
    #[serde(default = "default_max_image_size")]
    pub max_image_size: u32,
    /// Whether links in notification body are clickable (default: true)
    #[serde(default = "default_true")]
    pub enable_links: bool,
    /// Whether animated images (GIFs) play and card animations are enabled (default: true)
    #[serde(default = "default_true")]
    pub enable_animations: bool,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            do_not_disturb: false,
            anchor: Anchor::default(),
            max_notifications: 3,
            max_per_app: 2,
            max_timeout_urgent: None,
            max_timeout_normal: Some(5000),
            max_timeout_low: Some(3000),
            show_images: default_true(),
            show_actions: default_true(),
            max_image_size: default_max_image_size(),
            enable_links: default_true(),
            enable_animations: default_true(),
        }
    }
}

// Default value helpers for serde
const fn default_true() -> bool {
    true
}

const fn default_max_image_size() -> u32 {
    128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = NotificationsConfig::default();

        // Test original fields
        assert!(!config.do_not_disturb);
        assert_eq!(config.max_notifications, 3);
        assert_eq!(config.max_per_app, 2);
        assert_eq!(config.max_timeout_normal, Some(5000));
        assert_eq!(config.max_timeout_low, Some(3000));
        assert_eq!(config.max_timeout_urgent, None);

        // Test new rich notification fields
        assert!(config.show_images);
        assert!(config.show_actions);
        assert_eq!(config.max_image_size, 128);
        assert!(config.enable_links);
        assert!(config.enable_animations);
    }

    #[test]
    fn test_config_serialization() {
        let config = NotificationsConfig::default();
        let json = serde_json::to_string(&config).unwrap();

        // Should serialize all fields
        assert!(json.contains("show_images"));
        assert!(json.contains("show_actions"));
        assert!(json.contains("max_image_size"));
        assert!(json.contains("enable_links"));
        assert!(json.contains("enable_animations"));
    }

    #[test]
    fn test_config_deserialization_with_defaults() {
        // Simulate old config file (version 1) without rich notification fields
        let old_config_json = r#"{
            "do_not_disturb": false,
            "anchor": "Top",
            "max_notifications": 3,
            "max_per_app": 2,
            "max_timeout_urgent": null,
            "max_timeout_normal": 5000,
            "max_timeout_low": 3000
        }"#;

        let config: NotificationsConfig = serde_json::from_str(old_config_json).unwrap();

        // Old fields should deserialize correctly
        assert!(!config.do_not_disturb);
        assert_eq!(config.max_notifications, 3);

        // New fields should use defaults
        assert!(config.show_images);
        assert!(config.show_actions);
        assert_eq!(config.max_image_size, 128);
        assert!(config.enable_links);
        assert!(config.enable_animations);
    }

    #[test]
    fn test_config_deserialization_full() {
        // Config with all fields including rich notification options
        let full_config_json = r#"{
            "do_not_disturb": true,
            "anchor": "Bottom",
            "max_notifications": 5,
            "max_per_app": 3,
            "max_timeout_urgent": null,
            "max_timeout_normal": 6000,
            "max_timeout_low": 4000,
            "show_images": false,
            "show_actions": false,
            "max_image_size": 64,
            "enable_links": false,
            "enable_animations": false
        }"#;

        let config: NotificationsConfig = serde_json::from_str(full_config_json).unwrap();

        // All fields should deserialize correctly
        assert!(config.do_not_disturb);
        assert_eq!(config.max_notifications, 5);
        assert_eq!(config.max_per_app, 3);
        assert!(!config.show_images);
        assert!(!config.show_actions);
        assert_eq!(config.max_image_size, 64);
        assert!(!config.enable_links);
        assert!(!config.enable_animations);
    }

    #[test]
    fn test_max_image_size_range() {
        // Test various max_image_size values
        let test_cases = vec![
            (32, 32),   // Minimum valid
            (128, 128), // Default
            (256, 256), // Maximum valid
            (16, 16),   // Below minimum (should be handled by RichCardConfig::from_notifications_config)
            (512, 512), // Above maximum (should be handled by RichCardConfig::from_notifications_config)
        ];

        for (input, expected) in test_cases {
            let json = format!(r#"{{
                "do_not_disturb": false,
                "anchor": "Top",
                "max_notifications": 3,
                "max_per_app": 2,
                "max_timeout_urgent": null,
                "max_timeout_normal": 5000,
                "max_timeout_low": 3000,
                "max_image_size": {}
            }}"#, input);

            let config: NotificationsConfig = serde_json::from_str(&json).unwrap();
            assert_eq!(config.max_image_size, expected, "max_image_size should be {}", expected);
        }
    }

    #[test]
    fn test_default_helpers() {
        assert_eq!(default_true(), true);
        assert_eq!(default_max_image_size(), 128);
    }
}
