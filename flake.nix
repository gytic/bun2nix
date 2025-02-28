{
  description = "Create nix expressions from bun lockfiles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    pre-commit-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      bun2nix = pkgs.callPackage ./default.nix {};
    in {
      defaultPackage = bun2nix;

      defaultApp = {
        type = "app";
        program = "${bun2nix}/bin/bun2nix";
      };

      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            cargo-check.enable = true;
            clippy.enable = true;
            rustfmt.enable = true;
          };
        };
      };

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
