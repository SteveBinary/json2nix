{ pkgs, ... }:

let
  cliManifest = (pkgs.lib.importTOML ./json2nix-cli/Cargo.toml).package;
in
{
  cli = pkgs.rustPlatform.buildRustPackage {
    pname = "json2nix";
    version = cliManifest.version;

    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };
}
