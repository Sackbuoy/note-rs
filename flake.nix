{
 description = "Rust Flake";
 inputs = {
   nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
 };

 outputs = { self, nixpkgs, flake-utils, ... }:
   let
     system = "x86_64-linux";
     pkgs = nixpkgs.legacyPackages.${system};
     manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
   in {
   defaultPackage.${system} = pkgs.rustPlatform.buildRustPackage rec {
     pname = manifest.name;
     version = manifest.version;
     cargoLock.lockFile = ./Cargo.lock;
     src = pkgs.lib.cleanSource ./.;
   };

   devShell.${system} = pkgs.mkShell { 
     buildInputs = [
       pkgs.rustup
     ];
     shellHook = ''
       echo "Loaded Dev Environment"
       export RUSTUP_HOME="$PWD/.rustup"
       export CARGO_HOME="$PWD/.cargo"
       rustup toolchain install 1.83.0
       rustup default 1.83.0
       rustup component add rust-analyzer rustfmt clippy
     '';
   };
 };
}
