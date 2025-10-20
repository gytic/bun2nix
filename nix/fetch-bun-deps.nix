{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.function = mkOption {
      description = ''
        Bun cache creator function.

        Produces a file accurate, symlink farm recreation of bun's global install cache.

        See [bun's cache docs](https://github.com/oven-sh/bun/blob/642d04b9f2296ae41d842acdf120382c765e632e/docs/install/cache.md#L24)
        for more information.
      '';
      type = types.functionTo types.package;
    };
  };

  config.perSystem =
    { pkgs, config, ... }:
    {
      fetchBunDeps.function =
        {
          bunNix,
          overrides ? { },
          ...
        }@args:
        let
          attrIsDerivation = _: value: lib.isDerivation value;

          packages = lib.filterAttrs attrIsDerivation (pkgs.callPackage bunNix { });

          extractPackage = config.fetchBunDeps.extractPackage args;
          overridePackage = config.fetchBunDeps.overridePackage args;
          toCacheEntry = config.fetchBunDeps.toCacheEntry args;
        in

        assert lib.asserts.assertEachOneOf "overrides" (builtins.attrNames overrides) (
          builtins.attrNames packages
        );

        assert lib.assertMsg (builtins.all builtins.isFunction (builtins.attrValues overrides))
          "All attr values of `overrides` must be functions taking the old, unoverrided package and returning the new source.";

        pkgs.symlinkJoin {
          name = "bun-cache";
          paths = lib.pipe packages [
            (builtins.mapAttrs extractPackage)
            (builtins.mapAttrs overridePackage)
            (builtins.mapAttrs toCacheEntry)
            builtins.attrValues
          ];
        };
    };
}
