{
  lib,
  mkBunNodeModules,
  bun2nixHook,
  stdenv,
  ...
}:

lib.extendMkDerivation {
  constructDrv = stdenv.mkDerivation;
  excludeDrvArgNames = [
    "packageJson"
    "index"
    "bunNix"
    "workspaceRoot"
    "workspaces"
  ];
  extendDrvArgs = (
    _finalAttrs:
    {
      packageJson ? null,
      bunNix,
      dontPatchShebangs ? false,
      nativeBuildInputs ? [ ],
      buildFlags ? [
        "--compile"
        "--minify"
        "--sourcemap"
        "--bytecode"
      ],
      # Bun binaries built by this derivation become broken by the default fixupPhase
      dontFixup ? !(args ? buildPhase),
      ...
    }@args:

    assert lib.assertMsg (args ? pname || packageJson != null)
      "mkBunDerivation: Either `pname` or `packageJson` must be set in order to assign a name to the package. It may be assigned manually with `pname` which always takes priority or read from the `name` field of `packageJson`.";

    assert lib.assertMsg (args ? version || packageJson != null)
      "mkBunDerivation: Either `version` or `packageJson` must be set in order to assign a version to the package. It may be assigned manually with `version` which always takes priority or read from the `version` field of `packageJson`.";

    let
      packages = import bunNix;
      bunDeps = mkBunNodeModules { inherit packages dontPatchShebangs; };

      package = if packageJson != null then (builtins.fromJSON (builtins.readFile packageJson)) else { };

      pname = args.pname or package.name or null;
      version = args.version or package.version or null;
      index = args.index or package.module or null;
    in

    assert lib.assertMsg (pname != null)
      "mkBunDerivation: Either `name` must be specified in the given `packageJson` file, or passed as the `name` argument";

    assert lib.assertMsg (version != null)
      "mkBunDerivation: Either `version` must be specified in the given `packageJson` file, or passed as the `version` argument";

    {
      inherit
        pname
        version
        dontFixup
        dontPatchShebangs
        ;

      preConfigurePhases =
        args.preConfigurePhases or [
          "preNodeModulesInstallFixupPhase"
          "installNodeModulesPhase"
        ];

      preNodeModulesInstallFixupPhase =
        args.preNodeModulesInstallFixupPhase or ''
          patchShebangs .
        '';

      installNodeModulesPhase =
        args.installNodeModulesPhase or ''
          runHook preInstallNodeModulesPhase

          ln -sf ${bunDeps} ./node_modules/.bin

          # Unfortunately a full copy of node_modules does need to be done instead of a symlink as many packages will write to their install location
          rsync -a --copy-links --chmod=ugo+w --exclude=".bin" ${bunDeps}/node_modules/ ./node_modules/


          runHook postInstallNodeModulesPhase
        '';

      buildPhase =
        args.buildPhase or (
          assert lib.assertMsg (builtins.isString index)
            "mkBunDerivation: to use the default buildPhase, either `module` must be specified in the given `packageJson` file, or passed as the `index` argument, and it should not be a nix store path, but a path relative to the workspace directory";
          ''
            runHook preBuild
            bun build ${lib.concatStringsSep " " buildFlags} ${index} --outfile ${pname}
            runHook postBuild
          ''
        );

      installPhase =
        args.installPhase or ''
          runHook preInstall
          install -Dm755 ${pname} $out/bin/${pname}
          runHook postInstall
        '';

      nativeBuildInputs = nativeBuildInputs ++ [
        bun2nixHook
      ];
    }
  );
}
