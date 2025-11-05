{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.buildPackage = mkOption {
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
      fetchBunDeps.buildPackage =
        {
          patchShebangs ? true,
          ...
        }@args:
        let
          bunWithNode = config.fetchBunDeps.bunWithNode args;
        in
        name: pkg:
        pkgs.stdenvNoCC.mkDerivation {
          name = "bun-pkg-${name}";

          nativeBuildInputs = [
            bunWithNode
          ];

          phases = [
            "extractPhase"
            "patchPhase"
            "cacheEntryPhase"
          ];

          extractPhase = ''
            "${lib.getExe config.fetchBunDeps.extractPackage}" \
              ${pkg} \
              "$out/share/bun-packages/${name}"
          '';

          patchPhase = lib.optionalString patchShebangs ''
            patchShebangs "$out/share/bun-packages"
          '';

          cacheEntryPhase = ''
            "${lib.getExe self'.packages.cacheEntryCreator}" \
              --out "$out/share/bun-cache" \
              --name "${name}" \
              --package "$out/share/bun-packages/${name}"
          '';

          preferLocalBuild = true;
          allowSubstitutes = false;
        };
    };
}
