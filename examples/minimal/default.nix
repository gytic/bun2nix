{
  stdenv,
  bun,
  gnutar,
  callPackage,
  ...
}: let
  bunDeps = callPackage ./bun.nix {};
in
  stdenv.mkDerivation {
    name = "minimal-bun2nix-example";
    version = "1.0.0";

    src = ./.;

    nativeBuildInputs = [gnutar bun];

    # Build node_modules based on the lockfile generated bun.nix
    postUnpack = ''
      ln -s ${bunDeps.nodeModules} ./node_modules
    '';

    # Compile a bun binary with all settings for production
    buildPhase = ''
      bun build \
        --compile \
        --minify \
        --sourcemap \
        --bytecode \
        ./index.ts \
        --outfile minimal
    '';

    # Install the binary to the output folder
    installPhase = ''
      mkdir -p $out/bin

      cp ./minimal $out/bin
    '';

    # Bun binaries are broken by fixup
    dontFixup = true;
  }
