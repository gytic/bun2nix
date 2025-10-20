{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.toCacheEntry = mkOption {
      description = ''
        Takes an extracted package and creates a valid
        symlink in the place where bun would expect
        to find it's cache entry.
      '';
      type = types.functionTo (types.functionTo (types.functionTo types.package));
    };
  };

  config.perSystem =
    { pkgs, self', ... }:
    {
      fetchBunDeps.toCacheEntry =
        _: name: pkg:
        pkgs.runCommandLocal "pkg-${name}" { } ''
          "${lib.getExe self'.packages.cacheEntryCreator}" \
            --out "$out" \
            --name "${name}" \
            --package "${pkg}"
        '';

    };
}
