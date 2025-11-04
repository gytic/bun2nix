{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.extractPackage = mkOption {
      description = ''
        Generic package extraction script for use in fetchBunDeps.

        If the package is a tarball, extract it,
        otherwise make a copy of the input directory in $out
      '';
      type = types.package;
    };
  };

  config.perSystem =
    {
      pkgs,
      ...
    }:
    {
      fetchBunDeps.extractPackage = pkgs.writeShellApplication {
        name = "extract-bun-package";
        runtimeInputs = [
          pkgs.libarchive
        ];
        text = ''
          throw_usage () {
              echo "Unexpected number of args"
              echo "Usage <pkg> <out>"
              exit 1
          }

          if [ "$#" -ne 2 ]; then
            throw_usage
          fi

          pkg="$1"
          out="$2"

          mkdir -p "$out"

          if [[ "$pkg" = *.tgz ]]; then
            bsdtar --extract \
              --file "$pkg" \
              --directory "$out" \
              --strip-components=1 \
              --no-same-owner \
              --no-same-permissions
          else
            cp -r "$pkg/." "$out"
          fi

          chmod -R u+rwx "$out"
        '';
      };
    };
}
