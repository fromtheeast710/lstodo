{
  description = "LsTodo's Nix Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rust = pkgs.rust-bin.stable.latest.minimal.override {
          extensions = ["rust-src"];
        };

        builder = {
          lib,
          rustPlatform,
        }: let
          toml = (lib.importTOML ./Cargo.toml).package;
        in
          rustPlatform.buildRustPackage {
            inherit (toml) version;

            pname = toml.name;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            meta.mainProgram = "lstodo";
          };
      in
        with pkgs; {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [(import inputs.rust-overlay)];
          };

          packages.default = callPackage builder {};

          devShells.default = mkShell {
            packages = [
              rust
              rust-analyzer-unwrapped
              cargo-expand
              rust-bin.nightly."2024-04-07".rustfmt
            ];

            env.RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
          };
        };
    };
}
