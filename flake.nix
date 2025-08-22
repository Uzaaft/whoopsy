{
  description = "A very basic rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    # Rust overlay
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ] (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      # For `nix build` & `nix run`:
      packages = {
        inherit (pkgs) rust-toolchain;
      };

      devShell = pkgs.mkShell {
        packages = [];

        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          pkg-config
        ];
      };
    });
}
