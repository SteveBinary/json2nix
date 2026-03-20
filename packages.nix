{ pkgs }:

let
  rootManifest = pkgs.lib.importTOML ./Cargo.toml;
  cliManifest = pkgs.lib.importTOML ./json2nix-cli/Cargo.toml;
  webManifest = pkgs.lib.importTOML ./json2nix-web/Cargo.toml;
  cargoConfig = pkgs.lib.importTOML ./.cargo/config.toml;
  rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
{
  cli = pkgs.rustPlatform.buildRustPackage {
    pname = (builtins.head cliManifest.bin).name;
    version = rootManifest.workspace.package.version;

    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;

    nativeBuildInputs = [
      rustToolchain
    ];

    RUST_BACKTRACE = 1;

    cargoBuildFlags = "--package ${cliManifest.package.name}";
    cargoTestFlags = "--package ${cliManifest.package.name}";
  };

  web = pkgs.rustPlatform.buildRustPackage {
    pname = webManifest.package.name;
    version = rootManifest.workspace.package.version;

    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;

    nativeBuildInputs = with pkgs; [
      rustToolchain
      trunk

      # build tool which trunk would fail to install by itself
      binaryen
      wasm-bindgen-cli_0_2_114
    ];

    RUSTFLAGS = cargoConfig.build.rustflags;
    RUST_BACKTRACE = 1;

    doCheck = false; # the web app contains no tests

    buildPhase = ''
      cd json2nix-web
      trunk build \
        --release \
        --cargo-profile=release-for-web \
        --minify \
        --skip-version-check \
        --offline \
        --public-url "/json2nix" # hardcoded until there is a way to parametrize flakes, see https://github.com/NixOS/nix/issues/5663
    '';

    installPhase = ''
      mkdir -p $out
      mv dist/* $out
    '';
  };
}
