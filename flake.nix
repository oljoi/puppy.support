{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    cargo-leptos.url = "github:oljoi/cargo-leptos";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      cargo-leptos,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
        inherit (cargoToml.package) name version;

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.nightly.latest.default.override {
            targets = [
              "x86_64-unknown-linux-gnu"
              "wasm32-unknown-unknown"
            ];
          }
        );

        craneBuild = rec {
          args = {
            src = lib.cleanSourceWith {
              src = self;
              filter =
                path: type:
                (lib.hasSuffix "\.html" path)
                || (lib.hasSuffix "\.scss" path)
                || (lib.hasSuffix "\.css" path)
                || (lib.hasSuffix "tailwind.config.js" path)
                || (lib.hasInfix "/assets/" path)
                || (lib.hasInfix "/css/" path)
                || (lib.hasInfix "/public/" path)
                || (craneLib.filterCargoSources path type);
            };
            pname = name;
            version = version;
            buildInputs = [
              cargo-leptos.packages.default
              pkgs.binaryen
              pkgs.wasm-bindgen-cli_0_2_105
              pkgs.dart-sass
            ];
          };
          cargoArtifacts = craneLib.buildDepsOnly args;
          buildArgs = args // {
            inherit cargoArtifacts;
            buildPhaseCargoCommand = "cargo leptos build --release -vvv";
            doNotPostBuildInstallCargoBinaries = true;
            nativeBuildInputs = [
              pkgs.makeWrapper
            ];
            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/release/${name} $out/bin/
              cp -r target/site $out/bin/
              wrapProgram $out/bin/${name} \
                --set LEPTOS_SITE_ROOT $out/bin/site
            '';
          };
          package = craneLib.buildPackage (buildArgs);

          check = craneLib.cargoClippy (
            args
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
            }
          );

          doc = craneLib.cargoDoc (
            args
            // {
              inherit cargoArtifacts;
            }
          );
        };
      in
      {
        packages.default = craneBuild.package;
      }
    );
}
