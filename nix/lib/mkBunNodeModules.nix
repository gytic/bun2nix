{
  bun,
  fetchurl,
  lib,
  libarchive,
  makeWrapper,
  runCommand,
  ...
}:
packages:
runCommand "node-modules"
  {
    nativeBuildInputs = [
      libarchive
      makeWrapper
    ];
  }
  ''
    mkdir -p $out/node_modules/.bin

    # Extract a given package to it's destination
    extract() {
      local pkg=$1
      local dest=$2

      mkdir -p "$dest"
      bsdtar --extract \
        --file "$pkg" \
        --directory "$dest" \
        --strip-components=1 \
        --no-same-owner \
        --no-same-permissions

      chmod -R a+X "$dest"
    }

    # Process each package
    ${lib.concatStringsSep "\n" (
      lib.mapAttrsToList (
        name: pkg:
        let
          # Check if this is a workspace package (URL contains "workspace:")
          isWorkspace = lib.strings.hasInfix "workspace:" pkg.url;
          
          src = if isWorkspace 
            then null
            else fetchurl {
              inherit (pkg) name url hash;
            };

          binaries =
            if pkg ? binaries then
              lib.concatStringsSep "\n" (
                lib.mapAttrsToList (binName: binPath: ''
                  ln -sf "${binPath}" "$out/node_modules/.bin/${binName}"
                '') pkg.binaries
              )
            else
              "";
              
          # For workspace packages, we'll create an empty directory instead of extracting the package
          workspaceSetup = ''
            echo "Setting up workspace package ${name}..."
            mkdir -p "$out/node_modules/${pkg.out_path}"
            
            # Create a placeholder package.json to satisfy dependencies
            cat > "$out/node_modules/${pkg.out_path}/package.json" << EOF
            {
              "name": "${name}",
              "version": "0.0.0-workspace",
              "private": true
            }
            EOF
          '';
          
          # For regular packages, we'll extract them as usual
          regularSetup = ''
            echo "Installing package ${name}..."
            mkdir -p "$out/node_modules/${pkg.out_path}"
            extract "${src}" "$out/node_modules/${pkg.out_path}"
          '';
        in
        ''
          ${if isWorkspace then workspaceSetup else regularSetup}
          ${binaries}
        ''
      ) packages
    )}

    # Force bun instead of node for script execution
    makeWrapper ${bun}/bin/bun $out/bin/node
    export PATH="$out/bin:$PATH"

    patchShebangs $out/node_modules
  ''
