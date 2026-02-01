# Example NixOS configuration using cosmic-notifications-ng
# This demonstrates various configuration patterns

{ config, pkgs, ... }:

{
  # Import the module (when using the flake, this is handled automatically)
  # imports = [ ./module.nix ];

  # Enable COSMIC desktop environment
  services.desktopManager.cosmic.enable = true;

  # Basic configuration - enables cosmic-notifications-ng with defaults
  services.cosmic-notifications-ng = {
    enable = true;
  };

  # Full configuration example with all options
  # services.cosmic-notifications-ng = {
  #   enable = true;
  #
  #   # Use a custom package or override
  #   package = pkgs.cosmic-notifications-ng;
  #
  #   # Replace system cosmic-notifications (default: true)
  #   replaceSystemPackage = true;
  #
  #   settings = {
  #     # Image configuration
  #     show_images = true;
  #     max_image_size = 128;
  #     enable_animations = true;
  #
  #     # Interaction configuration
  #     show_actions = true;
  #     enable_links = true;
  #   };
  # };

  # Privacy-focused configuration
  # services.cosmic-notifications-ng = {
  #   enable = true;
  #   settings = {
  #     show_images = false;
  #     enable_links = false;
  #     enable_animations = false;
  #     show_actions = true;
  #   };
  # };

  # Performance-optimized configuration
  # services.cosmic-notifications-ng = {
  #   enable = true;
  #   settings = {
  #     max_image_size = 64;
  #     enable_animations = false;
  #     show_images = true;
  #     show_actions = true;
  #     enable_links = true;
  #   };
  # };

  # Minimal configuration (notifications with actions only)
  # services.cosmic-notifications-ng = {
  #   enable = true;
  #   settings = {
  #     show_images = false;
  #     enable_animations = false;
  #     enable_links = false;
  #     show_actions = true;
  #   };
  # };

  # Side-by-side installation (not replacing system package)
  # services.cosmic-notifications-ng = {
  #   enable = true;
  #   replaceSystemPackage = false;
  # };
}
