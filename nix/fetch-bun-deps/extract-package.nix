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
        otherwise make a copy of the directory in $out/share/bun-packages.

        If `patchShebangs` is enabled patch all
        scripts to use bun as their executor.

        Then, produce a bun cache compatible symlink in $out/share/bun-cache.
      '';
      type = types.functionTo (types.functionTo (types.functionTo types.package));
    };
  };

  config.perSystem =
    {
      pkgs,
      config,
      self',
      ...
    }:
    {
      fetchBunDeps.extractPackage =
        {
          patchShebangs ? true,
          ...
        }@args:
        let
          bunWithNode = config.fetchBunDeps.bunWithNode args;

          isTarball = pkg: lib.hasSuffix ".tgz" pkg;
        in
        name: pkg:
        pkgs.stdenvNoCC.mkDerivation {
          name = "bun-pkg-${name}";

          nativeBuildInputs = [
            bunWithNode
            pkgs.libarchive
          ];

          phases = [
            "extractPhase"
            "patchPhase"
            "cacheEntryPhase"
          ];

          extractPhase = ''
            bun_package_out="$out/share/bun-packages/${name}"
            mkdir -p "$bun_package_out"

            ${
              if (isTarball pkg) then
                ''
                  bsdtar --extract \
                    --file "${pkg}" \
                    --directory "$bun_package_out" \
                    --strip-components=1 \
                    --no-same-owner \
                    --no-same-permissions
                ''
              else
                ''
                  cp -r "${pkg}/." "$bun_package_out"
                ''
            }

            chmod -R u+rwx "$bun_package_out"
          '';

          patchPhase = lib.optionalString patchShebangs ''
            patchShebangs "$bun_package_out"
          '';

          cacheEntryPhase = ''
            bun_cache_out="$out/share/bun-cache"
            mkdir -p "$bun_cache_out"

            "${lib.getExe self'.packages.cacheEntryCreator}" \
              --out "$bun_cache_out" \
              --name "${name}" \
              --package "$bun_package_out"
          '';

          preferLocalBuild = true;
          allowSubstitutes = false;
        };
    };
}
