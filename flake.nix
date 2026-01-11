{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      nixpkgs,
      crane,
      rust-overlay,
      ...
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      lib = pkgs.lib;
      makeRustToolchain = p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain makeRustToolchain;
    in
    {
      devShells.${system}.default = craneLib.devShell {
      };

      packages.x86_64-linux =
        let
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./.)
              (lib.fileset.fromSource ./assets/font)
            ];
          };
          strictDeps = true;

          craneLibDefault = craneLib;
          craneLibMusl = (crane.mkLib pkgs).overrideToolchain (
            p:
            (makeRustToolchain p).override {
              targets = [ "x86_64-unknown-linux-musl" ];
            }
          );
        in
        {
          default = craneLibDefault.buildPackage {
            inherit src strictDeps;
          };

          static = craneLibMusl.buildPackage {
            inherit src strictDeps;
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          };
        };
    };
}
