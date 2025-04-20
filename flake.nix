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
      packages = bun2nix;

      defaultApp = {
        type = "app";
        program = "${bun2nix.bin}/bin/bun2nix";
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
          rust-analyzer
          cargo
          rustfmt
          clippy
          mold

          # Database
          sqlx-cli
          sqlite

          # SSL
          pkg-config
          openssl

          # Javascript dependencies
          bun
        ];

        env = with pkgs; {
          RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
          LD_LIBRARY_PATH = lib.makeLibraryPath [openssl];
          DATABASE_URL = "sqlite://.cache/bun2nix";
        };

        shellHook = ''
          mkdir .cache
          touch .cache/bun2nix
        '';
      };
    });
}
