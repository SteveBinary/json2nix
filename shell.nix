{ pkgs, ... }:

let
  rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in

pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  nativeBuildInputs = [
    rustToolchain
  ];
  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
  ];
}
