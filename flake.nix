{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          system = system;
          overlays = overlays;
        };
        xorgLibs = with pkgs; [
            libx11
            libx11.dev
            libxcursor
            libxcursor.dev
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = xorgLibs;
          nativeBuildInputs = with pkgs; [
            rust-bin.stable.latest.default
          ];

          LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath xorgLibs}";
        };
      }
    );
}

