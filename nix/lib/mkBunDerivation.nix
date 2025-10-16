{
  lib,
  mkDotBunDir,
  bun,
  stdenv,
  fetchurl,
  rsync,
  strace,
  writeShellApplication,
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
      packages = (import bunNix) { inherit fetchurl; };
      bunDeps = mkDotBunDir { inherit packages dontPatchShebangs; };

      package = if packageJson != null then (builtins.fromJSON (builtins.readFile packageJson)) else { };

      pname = args.pname or package.name or null;
      version = args.version or package.version or null;
      index = args.index or package.module or null;

      bun2nixNoOp = writeShellApplication {
        name = "bun2nix";
        text = "";
      };
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

          echo "Bun Deps: '${bunDeps}'"

          export HOME=$(mktemp -d)
          export BUN_INSTALL_CACHE_DIR=$(mktemp -d)

          cp -r ${bunDeps}/. $BUN_INSTALL_CACHE_DIR

          if [ ! -f "$BUN_INSTALL_CACHE_DIR/@types/react@19.0.10@@@1/package.json" ]; then
            echo "missing react pkg json"
            ls -la $BUN_INSTALL_CACHE_DIR
            exit 1
          fi

          if [ ! -f "$BUN_INSTALL_CACHE_DIR/tailwindcss@4.0.0-73c5c46324e78b9b@@@1/package.json" ]; then
            echo "missing tailwind pkg json"
            ls -la $BUN_INSTALL_CACHE_DIR
            exit 1
          fi

          strace bun install --linker=isolated --verbose &
          sleep 5
          exit 1

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
        bun
        bun2nixNoOp
        rsync
        strace
      ];
    }
  );
}
