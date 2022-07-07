{
  description = "Nix shell for Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells = {
          default = with pkgs; stdenv.mkDerivation {
            name = "devshell";
            buildInputs = [
              # Build tools
              cargo-readme

              rustup

              cmake
              clang
              pkg-config
              openssl
              nodejs
            ] ++ (lib.optionals stdenv.isDarwin ([
              libiconv
            ] ++ (with darwin.apple_sdk.frameworks; [
              DiskArbitration
              Foundation
            ])));
          };
        };
      });
}
