# NixOS VM test for cosmic-notifications-ng module
# Run with: nixos-rebuild build-vm -I nixos-config=./nix/test.nix

{ config, pkgs, lib, ... }:

{
  imports = [
    ./module.nix
  ];

  # Basic system configuration for testing
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "cosmic-notifications-test";

  # Enable COSMIC desktop
  services.desktopManager.cosmic.enable = true;
  services.displayManager.cosmic-greeter.enable = true;

  # Enable cosmic-notifications-ng
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

  # Test user
  users.users.testuser = {
    isNormalUser = true;
    password = "test";
    extraGroups = [ "wheel" ];
  };

  # Useful tools for testing
  environment.systemPackages = with pkgs; [
    libnotify
    dbus
  ];

  # Enable DBus for testing
  services.dbus.enable = true;

  # Minimum system configuration
  system.stateVersion = "24.11";

  # VM configuration
  virtualisation.vmVariant = {
    virtualisation = {
      memorySize = 2048;
      cores = 2;
      graphics = true;
    };
  };
}
