# Sourced from https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md#nix
{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    rustup

    lld
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXrandr xorg.libXi xorg.libXcursor
    libxkbcommon
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

