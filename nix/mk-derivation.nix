{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.mkDerivation.function = mkOption {
      description = ''
        Bun `mkDerivation` function.

        Similar to `stdenv.mkDerivation` but comes with
        additional specifics for creating bun packages.

        A lot of the implementation details exist inside
        the setup hook consumed by this function. This function's
        main role is to provide a more idiomatic interface for simple
        builds while the hook serves anything more custom.
      '';
      type = types.functionTo types.package;
    };
  };

  config.perSystem =
    { config, pkgs, ... }:
    {
      mkDerivation.function = lib.extendMkDerivation {
        constructDrv = pkgs.stdenv.mkDerivation;
        excludeDrvArgNames = [
          "packageJson"
          "index"
          "bunNix"
          "workspaceRoot"
          "workspaces"
        ];
        extendDrvArgs =
          _finalAttrs:
          {
            packageJson ? null,
            bunDeps,
            dontPatchShebangs ? false,
            nativeBuildInputs ? [ ],
            # Bun binaries built by this derivation become broken by the default fixupPhase
            dontFixup ? !(args ? buildPhase),
            ...
          }@args:

          assert lib.assertMsg (args ? pname || packageJson != null)
            "bun2nix.mkDerivation: Either `pname` or `packageJson` must be set in order to assign a name to the package. It may be assigned manually with `pname` which always takes priority or read from the `name` field of `packageJson`.";

          assert lib.assertMsg (args ? version || packageJson != null)
            "bun2nix.mkDerivation: Either `version` or `packageJson` must be set in order to assign a version to the package. It may be assigned manually with `version` which always takes priority or read from the `version` field of `packageJson`.";

          let
            pkgJsonContents = builtins.readFile packageJson;
            package = if packageJson != null then (builtins.fromJSON pkgJsonContents) else { };

            pname = args.pname or package.name or null;
            version = args.version or package.version or null;
            module = args.module or package.module or null;
          in

          assert lib.assertMsg (pname != null) ''
            bun2nix.mkDerivation: Either `name` must be specified in the given `packageJson` file, or passed as the `name` argument.

            `package.json`:
            ```json
            ${pkgJsonContents}
            ```
          '';

          assert lib.assertMsg (version != null) ''
            bun2nix.mkDerivation: Either `version` must be specified in the given `packageJson` file, or passed as the `version` argument.

            `package.json`:
            ```json
            ${pkgJsonContents}
            ```
          '';
          {
            inherit
              pname
              version
              dontFixup
              dontPatchShebangs
              bunDeps
              ;

            bunDefaultFlags = [
              "--linker=isolated"
            ];

            bunBuildFlags = [
              "${module}"
              "--outfile"
              "${pname}"
              "--compile"
              "--minify"
              "--sourcemap"
              "--bytecode"
            ];

            meta.mainProgram = pname;

            nativeBuildInputs = nativeBuildInputs ++ [
              config.mkDerivation.hook
            ];
          };
      };
    };
}
