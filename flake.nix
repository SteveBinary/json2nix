{
  description = "Nix flake for json2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [ (import rust-overlay) ];
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system overlays; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = import ./shell.nix { inherit pkgs; };
      });
      packages = forEachSupportedSystem ({ pkgs }:
        let
          package = import ./default.nix { inherit pkgs; };
        in
        rec {
          app = package.app;
          default = app;
        }
      );
    };
}
