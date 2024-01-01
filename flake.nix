{
  description = "A basic flake with a shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system: 
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in rec {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.automake
          pkgs.bashInteractive
          pkgs.cmake
          pkgs.openssl_3
        ];
        buildInputs = [
          pkgs.rust-analyzer
          pkgs.rust-bin.stable.latest.default
        ];
      };
    });
}
