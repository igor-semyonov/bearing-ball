{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    devShells.${system}.default = pkgs.mkShell rec {
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      buildInputs = with pkgs; [
        udev
        alsa-lib-with-plugins
        vulkan-loader
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        # xorg.libXrandr # To use the x11 feature
        libxkbcommon
        wayland # To use the wayland feature
      ];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    };
  };
}
