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
  embeddedFiles ? null,
  # New fields for workspace support
  workspaceRoot ? null, # Root directory containing all workspace packages
  workspaces ? { }, # Map of package name to source directory
  ...
}@args:
let
  bunDeps = mkBunNodeModules (import bunNix);
  packages = import bunNix;

  # Check if there are workspace packages
  hasWorkspaces = lib.any (pkg: lib.strings.hasInfix "workspace:" pkg.url) (lib.attrValues packages);
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

        bun build ${lib.concatStringsSep " " buildFlags} ${args.index} ${args.embeddedFiles} --outfile ${pname}

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
