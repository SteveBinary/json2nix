{ pkgs, ... }:

let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
{
  app = pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;

    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };
}
