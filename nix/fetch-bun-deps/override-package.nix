{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.overridePackage = mkOption {
      description = ''
        Allows applying a custom override function to a specific
        package via `fetchBunDeps`.

        # API Type

        Takes a struct of overrides where attributes have the
        type:

        String => Package => Package

        # Example

        ```nix
        "@types/bun@1.2.4" = prev: runCommandLocal "bun-types-override" {
          nativeBuildInputs = [ pkgs.jq ];
          src = prev;
        } \'\'
          # Apply an aribitrary patch to the package.json

          jq '. | {version: "0.1.0"}' package.json > package.json

          mkdir $out
          cp -r ./. $out
        \'\'
        ```
      '';
      type = types.functionTo (types.functionTo (types.functionTo types.package));
    };
  };

  config.perSystem =
    { pkgs, ... }:
    {
      fetchBunDeps.overridePackage =
        {
          overrides ? { },
          ...
        }:
        let
          isTarball = pkg: lib.hasSuffix ".tgz" pkg;

          preExtractPackage =
            name: pkg:
            pkgs.runCommandLocal "pre-extract-${name}"
              {
                nativeBuildInputs = [
                  pkgs.libarchive
                ];
              }
              ''
                mkdir -p "$out"

                ${
                  if (isTarball pkg) then
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

                chmod -R u+rwx "$out"
              '';
        in
        name: pkg:
        if (overrides ? "${name}") then (overrides.${name} (preExtractPackage name pkg)) else pkg;
    };
}
