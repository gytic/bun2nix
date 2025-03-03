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

    # Compile a bun binary with all settings for production
    buildPhase = ''
      # Load node_modules based on the lockfile generated bun.nix
      cp -rL ${bunDeps.nodeModules} ./node_modules

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
