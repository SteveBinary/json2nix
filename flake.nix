{
  description = "Nix flake for json2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        packages = import ./packages.nix { inherit pkgs; };
      in
      {
        formatter = pkgs.nixfmt;
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            packages.cli
            packages.web
          ];

          buildInputs = with pkgs; [
            rust-analyzer
            rustfmt
            clippy
          ];

          env = {
            TRUNK_SKIP_VERSION_CHECK = "true";
          };
        };
        packages = {
          cli = packages.cli;
          web = packages.web;
          default = packages.cli;
        };
      }
    );
}
