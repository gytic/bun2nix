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
  # New fields for workspace support
  workspaceRoot ? null,        # Root directory containing all workspace packages
  workspaces ? {},             # Map of package name to source directory
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
        
        # If workspaceRoot is provided, use a standardized workspace layout
        ${lib.optionalString (workspaceRoot != null) ''
          # Loop through all packages to detect workspace packages and link them
          ${lib.concatStrings (lib.mapAttrsToList (name: pkg: 
            lib.optionalString (lib.strings.hasInfix "workspace:" pkg.url) ''
              # Extract workspace identifier from npm identifier
              WORKSPACE_PATH=$(echo "${pkg.name}" | sed -n 's/.*workspace:\(.*\)/\1/p')
              if [ -n "$WORKSPACE_PATH" ]; then
                echo "Linking workspace package ${name} from ${workspaceRoot}/$WORKSPACE_PATH"
                mkdir -p $(dirname "node_modules/${pkg.out_path}")
                if [ -d "${workspaceRoot}/$WORKSPACE_PATH" ]; then
                  rsync -a --copy-links "${workspaceRoot}/$WORKSPACE_PATH/" "node_modules/${pkg.out_path}/"
                else
                  echo "Warning: Workspace package ${name} directory not found at ${workspaceRoot}/$WORKSPACE_PATH"
                  # Fallback to common workspace paths
                  SIMPLE_NAME=$(echo "${name}" | sed -e 's|^@[^/]*/||')
                  if [ -d "${workspaceRoot}/packages/$SIMPLE_NAME" ]; then
                    echo "Found alternative at ${workspaceRoot}/packages/$SIMPLE_NAME"
                    rsync -a --copy-links "${workspaceRoot}/packages/$SIMPLE_NAME/" "node_modules/${pkg.out_path}/"
                  elif [ -d "${workspaceRoot}/$SIMPLE_NAME" ]; then
                    echo "Found alternative at ${workspaceRoot}/$SIMPLE_NAME"
                    rsync -a --copy-links "${workspaceRoot}/$SIMPLE_NAME/" "node_modules/${pkg.out_path}/"
                  fi
                fi
              fi
            ''
          ) packages)}
        ''}
        
        # If specific workspaces are provided, use those mappings
        ${lib.concatStrings (lib.mapAttrsToList (name: path: ''
          echo "Linking workspace package ${name} from ${path}"
          mkdir -p $(dirname "node_modules/${name}")
          rsync -a --copy-links "${path}/" "node_modules/${name}/"
        '') workspaces)}
      ''}

      mkdir tmp
      export HOME=$TMPDIR

      runHook postConfigure
    '';

    # Create a react static html site as per the script
    buildPhase =
      assert lib.assertMsg (args.index != null)
        "`index` input to `mkBunDerivation` pointing to your javascript index file must be set in order to use the default buildPhase";
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
