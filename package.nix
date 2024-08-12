{ lib, rustPlatform }:
let
  toml = (lib.importTOML ./Cargo.toml).package;
in rustPlatform.buildRustPackage {
  inherit (toml) version;

  pname = toml.name;
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  meta.mainProgram = "lstodo";
}