{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-SBKjxhC6zHTu0SyJwxLlQHItzMzYZ71VCWQC2hOzpRY=";
        };
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.default = (pkgs.makeRustPlatform {cargo = toolchain; rustc = toolchain;}).buildRustPackage {
          pname = "lobars";
          version = "0.1.2";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock; 
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            toolchain
          ];
        };
    });
}
