{
  description = "Rust Flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
    in {
    defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
      pname = manifest.name;
      version = manifest.version;
      cargoLock.lockFile = ./Cargo.lock;
      src = pkgs.lib.cleanSource ./.;
    };

    devShell = pkgs.mkShell { 
      buildInputs =
      [
        pkgs.rust-analyzer # LSP Server
        pkgs.rustfmt       # Formatter
        pkgs.clippy        # Linter
      ];
      shellHook = ''
        echo "Loaded Dev Environment"
      '';
    };
  });
}
