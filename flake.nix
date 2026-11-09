{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell rec {
      nativeBuildInputs = [pkgs.pkg-config];
      buildInputs = with pkgs; [
        udev
        alsa-lib
        vulkan-loader
        wayland
        libxkbcommon
        libx11
        libxcursor
        libxi
        libxrandr
        openssl
      ];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      shellHook = ''
        exec zsh
      '';
    };
  };
}
