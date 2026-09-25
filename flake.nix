{
  description = "ThatProject";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [ (import rust-overlay ) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolChain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolChain
            pkgs.cargo-watch
            pkgs.cargo-edit
            pkgs.pkg-config
          ];

          shellHook = ''
            echo "ThatProject dev shell\
              rustc: $(rustc --version)\
            ";
          '';
        };
      }
    );

    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = cargoToml.package.name;
      version = cargoToml.package.version;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
}