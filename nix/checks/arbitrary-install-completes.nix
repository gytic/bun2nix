{ lib, ... }:
{
  perSystem =
    { pkgs, self', ... }:
    {
      checks.arbitraryInstallCompletes = pkgs.stdenv.mkDerivation {
        name = "bun2nix-exec-test";

        outputHash = "sha256-xOyFWDXEhcpw/36e888JC+1vwNm5o+O3JpZfTfSsg1I=";
        outputHashAlgo = "sha256";
        outputHashMode = "recursive";

        src = ./arbitrary-install-completes/test-project;

        nativeBuildInputs = with pkgs; [
          nix
          cacert
          git
        ];

        installPhase = ''
          PWD="$(pwd)"

          export NIX_STATE_DIR=$PWD/nix-state
          export NIX_STORE_DIR=$PWD/nix-store
          export NIX_PROFILES_DIR=$PWD/nix-profiles
          export NIX_CONF_DIR=$PWD/nix-conf
          export HOME=$PWD/home
          mkdir -p $NIX_STATE_DIR $NIX_STORE_DIR $NIX_PROFILES_DIR $NIX_CONF_DIR $HOME

          nix eval \
            --extra-experimental-features nix-command \
            --expr "$(${lib.getExe self'.packages.bun2nix})"

          echo ${self'.packages.bun2nix.version} > $out
        '';
      };
    };
}
