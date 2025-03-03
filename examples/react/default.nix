{
  stdenv,
  bun,
  gnutar,
  callPackage,
  nodejs,
  ...
}: let
  bunDeps = callPackage ./bun.nix {};
in
  stdenv.mkDerivation {
    name = "react-bun2nix-example";
    version = "1.0.0";

    src = ./.;

    nativeBuildInputs = [gnutar bun nodejs];

    # Create a react static html site as per the script
    buildPhase = ''
      # Load node_modules based on the lockfile generated bun.nix
      cp -rL ${bunDeps.nodeModules} ./node_modules

      # No install forces bun to inspect our ./node_modules symlink
      npm run build \
        --minify
    '';

    # Install the binary to the output folder
    installPhase = ''
      mkdir -p $out/dist

      cp -R ./dist $out
    '';
  }
