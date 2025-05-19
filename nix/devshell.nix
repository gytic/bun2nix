{ pkgs, ... }:
pkgs.mkShell (
  {
    packages = with pkgs; [
      # Rust dependencies
      rustc
      cargo
      rustfmt
      clippy

      # Database
      sqlx-cli
      sqlite

      # SSL
      pkg-config
      openssl

      # Docs
      mdbook

      # Javascript dependencies
      bun
    ];

    env = with pkgs; {
      LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
      DATABASE_URL = "sqlite://.cache/bun2nix";
    };

    shellHook = ''
      mkdir -p .cache
      touch .cache/bun2nix
    '';
  }
  # Mold does not support MacOS
  // (
    with pkgs;
    lib.optionalAttrs (!stdenv.isDarwin) {
      packages = [
        mold
      ];

      env = {
        RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
      };
    }
  )
)
