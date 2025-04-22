{
  outputs = inputs: with inputs;
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = { pkgs, system, ... }: let
        workspace = nocargo.lib.${system}.mkRustPackageOrWorkspace {
      	         # NOTE: abstraction gore ^^^^^^^^^^^^^^^^^^^^^^^^
          src = ./.;
        };
      in {
        packages.default = workspace.release.lstodo.bin;

        devShells.default = with pkgs; mkShellNoCC {
          packages = [
            rust-analyzer-unwrapped
            rustfmt
          ];};
      };
    };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nocargo = {
      url = "github:oxalica/nocargo";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.registry-crates-io.follows = "registry-crates-io";
    };
    registry-crates-io = {
      url = "github:rust-lang/crates.io-index";
      flake = false;
    };
  };
}
