{
  bun,
  symlinkJoin,
  runCommandLocal,
  stdenvNoCC,
  makeWrapper,
  lib,
  libarchive,
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
    name: tarball:
    runCommandLocal "patch-${name}"
      {
        nativeBuildInputs = [
          bunWithFakeNode
          libarchive
        ];
      }
      ''
       mkdir "$out"

       bsdtar --extract \
         --file "${tarball}" \
         --directory "$out" \
         --strip-components=1 \
         --no-same-owner \
         --no-same-permissions

        chmod -R a+X "$out"

        ${lib.optionalString (!dontPatchShebangs) ''
          patchShebangs "$out"
        ''}
      '';

  toNamedPath =
    name: pkg:
    runCommandLocal "pkg-${name}"
      { }
      ''
        mkdir "$out"
        ln -sf "${pkg}" "$out/${name}"
      '';

  patched = if dontPatchShebangs then packages else (builtins.mapAttrs extractPackage packages);

  packagePaths = builtins.mapAttrs toNamedPath patched;
in
symlinkJoin {
  name = ".bun";
  paths = builtins.attrValues packagePaths;
}

# runCommand ".bun"
#   {
#     nativeBuildInputs = [
#       libarchive
#       makeWrapper
#     ];
#   }
#   ''
#     mkdir -p $out/node_modules/.bin
#
#     # Extract a given package to it's destination
#     extract() {
#       local pkg=$1
#       local dest=$2
#
#       mkdir -p "$dest"
#       bsdtar --extract \
#         --file "$pkg" \
#         --directory "$dest" \
#         --strip-components=1 \
#         --no-same-owner \
#         --no-same-permissions
#
#       chmod -R a+X "$dest"
#     }
#
#     # Process each package
#     ${lib.concatStringsSep "\n" (
#       lib.mapAttrsToList (
#         name: pkg:
#         let
#           # Check if this is a workspace package (URL contains "workspace:")
#           isWorkspace = lib.strings.hasInfix "workspace:" pkg.url;
#
#           src =
#             if isWorkspace then
#               null
#             else
#               fetchurl {
#                 inherit (pkg) name url hash;
#               };
#
#           binaries =
#             if pkg ? binaries then
#               lib.concatStringsSep "\n" (
#                 lib.mapAttrsToList (binName: binPath: ''
#                   ln -sf "${binPath}" "$out/node_modules/.bin/${binName}"
#                 '') pkg.binaries
#               )
#             else
#               "";
#
#           # For workspace packages, we'll create an empty directory instead of extracting the package
#           installWorkspacePackage = ''
#             echo "Setting up workspace package ${name}..."
#             mkdir -p "$out/node_modules/${pkg.out_path}"
#
#             # Create a placeholder package.json to satisfy dependencies
#             cat > "$out/node_modules/${pkg.out_path}/package.json" << EOF
#             {
#               "name": "${name}",
#               "version": "0.0.0-workspace",
#               "private": true
#             }
#             EOF
#           '';
#
#           # For regular packages, we'll extract them as usual
#           installNpmPackage = ''
#             echo "Installing package ${name}..."
#             mkdir -p "$out/node_modules/${pkg.out_path}"
#             extract "${src}" "$out/node_modules/${pkg.out_path}"
#           '';
#         in
#         ''
#           ${if isWorkspace then installWorkspacePackage else installNpmPackage}
#           ${binaries}
#         ''
#       ) packages
#     )}
#
#     ${lib.optionalString (!dontPatchShebangs) ''
#       # Force bun instead of node for script execution
#       makeWrapper ${bun}/bin/bun $out/bin/node
#       export PATH="$out/bin:$PATH"
#
#     ''}
#   ''
