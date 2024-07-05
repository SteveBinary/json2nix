{ pkgs, ... }:

let
  manifest = (pkgs.lib.importTOML ./json2nix-cli/Cargo.toml).package;
in
{
  cli = pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;

    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };
}
