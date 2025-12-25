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
        cargoLeptos =
          {
            cargoArtifacts,
            cargoLeptosExtraArgs ? "",
            cargoExtraArgs ? "",
          }@origArgs:
          let
            args = builtins.removeAttrs origArgs [
              "cargoLeptosExtraArgs"
              "cargoExtraArgs"
            ];
          in
          craneLib.mkCargoDerivation (
            args
            // {
              inherit cargoArtifacts;

              pnameSuffix = "-leptos";

              buildPhaseCargoCommand = "cargo leptos ${cargoExtraArgs} ${cargoLeptosExtraArgs}";

              nativeBuildInputs = (args.nativeBuildInputs or [ ]) ++ [
                cargo-leptos.packages.${pkgs.stdenv.hostPlatform.system}.default
              ];
            }
          );

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        # NB: we don't need to overlay our custom toolchain for the *entire*
        # pkgs (which would require rebuidling anything else which uses rust).
        # Instead, we just want to update the scope that crane will use by appending
        # our specific toolchain there.
        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.nightly.latest.default.override {
            targets = [
              "x86_64-unknown-linux-gnu"
              "wasm32-unknown-unknown"
            ];
          }
        );

        unfilteredRoot = ./.;
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (
              file:
              lib.any file.hasExt [
                "html"
                "scss"
              ]
            ) unfilteredRoot)
            # Example of a folder for images, icons, etc
            (lib.fileset.maybeMissing ./public)
          ];
        };

        puppy-support =
          {
            extra ? "",
          }:
          craneLib.buildPackage {
            inherit src;
            strictDeps = true;
            doCheck = false;
            cargoBuildCommand = "cargo leptos build -vv";
            cargoExtraArgs = "";

            nativeBuildInputs =
              with pkgs;
              [
                perl
                pkg-config
                cargo-leptos.packages.${pkgs.stdenv.hostPlatform.system}.default
              ]
              ++ lib.optionals stdenv.buildPlatform.isDarwin [
                libiconv
              ];

            buildInputs = [
              pkgs.openssl
            ];
          };
      in
      {
        packages.default = puppy-support {
          extra = "--debug";
        };
      }
    );
}
