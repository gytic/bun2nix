{
  lib,
  mkBunNodeModules,
  stdenv,
  bun,
  rsync,
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
      workspaceRoot ? null,
      workspaces ? { },
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
      hasWorkspaces = lib.any (pkg: lib.strings.hasInfix "workspace:" pkg.url) (lib.attrValues packages);
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

      preConfigurePhases = args.preConfigurePhases or [
        "installNodeModulesPhase"
      ];

      installNodeModulesPhase =
        args.installNodeModulesPhase or ''
          runHook preInstallNodeModulesPhase

          # Unfortunately a full copy of node_modules does need to be done instead of a symlink as many packages will write to their install location
          rsync -a --copy-links --chmod=ugo+w --exclude=".bin" ${bunDeps}/node_modules/ ./node_modules/

          # Preserve symlinks in .bin
          if [ -d "${bunDeps}/node_modules/.bin" ]; then
            rsync -a --links ${bunDeps}/node_modules/.bin/ ./node_modules/.bin/
          fi

          # Handle workspace packages automatically if present
          ${lib.optionalString hasWorkspaces ''
            echo "Setting up workspace packages..."
          ''}

          # Setup workspace packages if workspaceRoot is provided
          ${lib.optionalString (hasWorkspaces && workspaceRoot != null) ''
            # Loop through all packages to detect workspace packages and link them
            ${lib.concatStrings (
              lib.mapAttrsToList (
                name: pkg:
                if !(lib.strings.hasInfix "workspace:" pkg.url) then
                  ""
                else
                  let
                    workspacePath = builtins.replaceStrings [ "workspace:" ] [ "" ] pkg.url;
                  in
                  ''
                    # Extract workspace identifier from npm identifier
                    echo "Linking workspace package ${name} from ${workspaceRoot}/${workspacePath}"
                    mkdir -p $(dirname "node_modules/${pkg.out_path}")

                    if [ -d "${workspaceRoot}/${workspacePath}" ]; then
                      # Primary path exists, use it
                      rsync -a --copy-links "${workspaceRoot}/${workspacePath}/" "node_modules/${pkg.out_path}/"
                    else
                      echo "Warning: Workspace package ${name} directory not found at ${workspaceRoot}/${workspacePath}"

                      # Fallback to common workspace paths
                      SIMPLE_NAME=$(echo "${name}" | sed -e 's|^@[^/]*/||')

                      if [ -d "${workspaceRoot}/packages/$SIMPLE_NAME" ]; then
                        echo "Found alternative at ${workspaceRoot}/packages/$SIMPLE_NAME"
                        rsync -a --copy-links "${workspaceRoot}/packages/$SIMPLE_NAME/" "node_modules/${pkg.out_path}/"
                      else
                        if [ -d "${workspaceRoot}/$SIMPLE_NAME" ]; then
                          echo "Found alternative at ${workspaceRoot}/$SIMPLE_NAME"
                          rsync -a --copy-links "${workspaceRoot}/$SIMPLE_NAME/" "node_modules/${pkg.out_path}/"
                        else
                          echo "Could not find workspace package directory"
                        fi
                      fi
                    fi
                  ''
              ) packages
            )}
          ''}

          # Setup explicitly provided workspaces
          ${lib.concatStrings (
            lib.mapAttrsToList (name: path: ''
              echo "Linking workspace package ${name} from ${path}"
              mkdir -p $(dirname "node_modules/${name}")
              rsync -a --copy-links "${path}/" "node_modules/${name}/"
            '') workspaces
          )}

          mkdir tmp
          export HOME=$TMPDIR

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
        rsync
        bun
      ];
    }
  );
}
