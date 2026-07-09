{
  description = "A cli AI anime girl app for those who live in terminal.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    naersk.inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    flake-parts,
    naersk,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {pkgs, ...}: let
        naersk' = pkgs.callPackage naersk {};
      in {
        packages.default = naersk'.buildPackage {
          pname = "cli-chan";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = with pkgs; [pkg-config];
          buildInputs = with pkgs; [openssl];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-analyzer
            clippy
            rustfmt
            pkg-config
            openssl
          ];
        };
      };
    };
}
