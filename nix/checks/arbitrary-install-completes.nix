{ lib, ... }:
{
  perSystem =
    { pkgs, self', ... }:
    {
      checks.arbitraryInstallCompletes = pkgs.stdenv.mkDerivation {
        name = "bun2nix-exec-test";

        outputHash = "sha256-pQpattmS9VmO3ZIQUFn66az8GSmB4IvYhTTCFn6SUmo=";
        outputHashAlgo = "sha256";
        outputHashMode = "recursive";

        src = ./arbitrary-install-completes/test-project;

        nativeBuildInputs = with pkgs; [
          nix
          cacert
          git
        ];

        installPhase = ''
          mkdir -p "$out"
          PWD="$(pwd)"

          export NIX_STATE_DIR=$PWD/nix-state
          export NIX_STORE_DIR=$PWD/nix-store
          export NIX_PROFILES_DIR=$PWD/nix-profiles
          export NIX_CONF_DIR=$PWD/nix-conf
          export HOME=$PWD/home
          mkdir -p $NIX_STATE_DIR $NIX_STORE_DIR $NIX_PROFILES_DIR $NIX_CONF_DIR $HOME

          ${lib.getExe self'.packages.bun2nix}
        '';
      };
    };
}
