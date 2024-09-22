{
  inputs = {
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });

        cargoClippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "clippy";
          }
        );

        cargoDoc = craneLib.cargoDoc (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "doc";
          }
        );

        bevy_vr_controller = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "bevy_vr_controller";
          }
        );
      in
      {
        checks = {
          inherit bevy_vr_controller cargoClippy cargoDoc;
        };

        packages = {
          inherit bevy_vr_controller;
          default = bevy_vr_controller;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${localSystem};
          packages = with pkgs; [
            cargo-machete
            cargo-watch
            nodePackages.prettier
            rust-analyzer
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
