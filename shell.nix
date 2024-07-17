{ pkgs, ... }:

let
  package = pkgs.callPackage ./package.nix {};
in
pkgs.mkShell {
  inputsFrom = [
    package.cli
    package.web
  ];

  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
  ];

  env = {
    TRUNK_SKIP_VERSION_CHECK = "true";
  };
}
