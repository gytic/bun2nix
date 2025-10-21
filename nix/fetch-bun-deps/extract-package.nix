{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.extractPackage = mkOption {
      description = ''
        If the package is a tarball, extract it,
        otherwise make a copy of the directory.

        If `patchShebangs` is enabled patch all
        scripts to use bun as their executor.
      '';
      type = types.functionTo (types.functionTo (types.functionTo types.package));
    };
  };

  config.perSystem =
    { pkgs, config, ... }:
    {
      fetchBunDeps.extractPackage =
        {
          patchShebangs ? true,
          ...
        }:
        name: pkg:
        pkgs.runCommandLocal "extract-${name}"
          {
            nativeBuildInputs = [
              config.fetchBunDeps.bunWithFakeNode
              pkgs.libarchive
            ];
          }
          ''
            mkdir -p "$out"

            ${
              if (lib.hasSuffix ".tgz" pkg) then
                ''
                  bsdtar --extract \
                    --file "${pkg}" \
                    --directory "$out" \
                    --strip-components=1 \
                    --no-same-owner \
                    --no-same-permissions
                ''
              else
                ''
                  cp -r "${pkg}" "$out"
                ''
            }

            chmod -R a+X "$out"

            ${lib.optionalString patchShebangs ''
              patchShebangs "$out"
            ''}
          '';

    };
}
