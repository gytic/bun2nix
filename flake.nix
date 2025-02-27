{
  description = "Create nix expressions from bun lockfiles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          # Rust dependencies
          rustc
          cargo
          rustfmt
          clippy
          mold

          # Javascript dependencies
          bun
        ];

        env = {
          RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
        };
      };
    });
}
