{
  description = "LsTodo's Nix Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default ];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./Toolchain.toml;
  in with pkgs; {
    devShells.${system}.default = mkShell {
      packages = [
        toolchain
        rust-analyzer-unwrapped
        cargo-expand
      ];
      RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
    };
  };
}