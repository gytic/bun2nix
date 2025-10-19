{
  bun,
  symlinkJoin,
  runCommandLocal,
  stdenvNoCC,
  makeWrapper,
  lib,
  libarchive,
  cache-entry-creator,
  ...
}:
{
  packages,
  dontPatchShebangs ? false,
  ...
}:
assert lib.assertMsg (lib.isAttrs packages)
  "`mkDotBunDir`: `packages` attr must be an attr set of string names to derivation values for every package";
let

  bunWithFakeNode = stdenvNoCC.mkDerivation {
    name = "fake-node";

    nativeBuildInputs = [
      makeWrapper
    ];

    dontUnpack = true;
    dontBuild = true;

    installPhase = ''
      cp -r "${bun}/." "$out"
      chmod +w $out/bin
      makeWrapper "$out/bin/bun" "$out/bin/node"
    '';
  };

  extractPackage =
    name: pkg:
    runCommandLocal "patch-${name}"
      {
        nativeBuildInputs = [
          bunWithFakeNode
          libarchive
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

        ${lib.optionalString (!dontPatchShebangs) ''
          patchShebangs "$out"
        ''}
      '';

  toNamedPath =
    name: pkg:
    runCommandLocal "pkg-${name}" { } ''
      "${lib.getExe cache-entry-creator}" \
        --out "$out" \
        --name "${name}" \
        --package "${pkg}"
    '';
in
symlinkJoin {
  name = "bun-cache";
  paths = lib.pipe packages [
    (builtins.mapAttrs extractPackage)
    (builtins.mapAttrs toNamedPath)
    builtins.attrValues
  ];
}
