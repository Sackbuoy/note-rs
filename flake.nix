{
  description = "Rust Flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-darwin"];
    forAllSystems = nixpkgs.lib.genAttrs systems;
    pkgsFor = system: nixpkgs.legacyPackages.${system};
  in {
    packages = forAllSystems (system: let
      pkgs = pkgsFor system;
      manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
    in {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = manifest.name;
        version = manifest.version;
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
      };
    });

    devShells = forAllSystems (system: let
      pkgs = pkgsFor system;
    in {
      default = pkgs.mkShell {
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
    });

    apps = forAllSystems (system: let
      pkgs = pkgsFor system;
    in {
      default = {
        type = "app";
        program = toString (pkgs.writeShellScript "run" ''
          export RUSTUP_HOME="$PWD/.rustup"
          export CARGO_HOME="$PWD/.cargo"
          ${pkgs.cargo}/bin/cargo run "$@"
        '');
      };
    });
  };
}
