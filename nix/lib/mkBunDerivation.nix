{
  bun,
  lib,
  mkBunNodeModules,
  rsync,
  stdenv,
  ...
}:
{
  pname,
  version,
  src,
  bunNix,
  buildFlags ? [
    "--compile"
    "--minify"
    "--sourcemap"
    "--bytecode"
  ],
  ...
}@args:
let
  bunDeps = mkBunNodeModules (import bunNix);
in
stdenv.mkDerivation (
  {
    inherit pname version src;

    nativeBuildInputs = [
      rsync
      bun
    ];

    # Load node_modules based on the expression generated from the lockfile
    configurePhase = ''
      runHook preConfigure

      # Unfortunately a full copy of node_modules does need to be done instead of a symlink as many packages will write to their install location
      rsync -a --copy-links --chmod=ugo+w --exclude=".bin" ${bunDeps}/node_modules/ ./node_modules/

      # Preserve symlinks in .bin
      if [ -d "${bunDeps}/node_modules/.bin" ]; then
        rsync -a --links ${bunDeps}/node_modules/.bin/ ./node_modules/.bin/
      fi

      mkdir tmp
      export HOME=$TMPDIR

      runHook postConfigure
    '';

    # Create a react static html site as per the script
    buildPhase =
      assert lib.assertMsg (args.index != null)
        "`index` input to `mkBunDerivation` pointing to your javascript index file must be set in order to use the default buildPhase";
      assert lib.assertMsg (lib.isString args.index)
        "`index` should be a string value pointing to your index file from the root of your repository. If you use a nix path here (./index.ts (BAD) vs 'index.ts'(GOOD)) this will not be able to resolve dependencies correctly as the path version will be copied to the nix store separately";
      ''
        runHook preBuild

        bun build ${lib.concatStringsSep " " buildFlags} ${args.index} --outfile ${pname}

        runHook postBuild
      '';

    # Install the binary to the output folder
    installPhase = ''
      runHook preInstall

      mkdir -p $out/bin

      cp ./${pname} $out/bin

      runHook postInstall
    '';

    # Bun binaries are broken by fixup phase
    dontFixup = true;
  }
  // lib.optionalAttrs (!(args ? buildPhase) && !(args ? installPhase)) {
    meta.mainProgram = pname;
  }
  // args
)
