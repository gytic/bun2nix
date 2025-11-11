{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.writeBunApplication = mkOption {
      description = ''
        Bun Application Builder

        Used to create an executable for a project which
        running requires:
        - A `bun install`
        - Running some command from package.json
      '';
      type = types.functionTo types.package;
    };
  };

  config.perSystem =
    { pkgs, config, ... }:
    {
      writeBunApplication = lib.extendMkDerivation {
        constructDrv = config.mkDerivation.function;

        extendDrvArgs =
          _finalAttrs:
          {
            startScript,
            runtimeInputs ? [ ],
            runtimeEnv ? { },
            excludeShellChecks ? [ ],
            extraShellCheckFlags ? [ ],
            bashOptions ? [
              "errexit"
              "nounset"
              "pipefail"
            ],
            inheritPath ? true,
            ...
          }:
          let
            script = pkgs.writeShellApplication {
              inherit
                runtimeEnv
                excludeShellChecks
                extraShellCheckFlags
                bashOptions
                inheritPath
                ;

              name = "bun2nix-application-startup";
              text = startScript;
              runtimeInputs = [
                pkgs.bun
              ]
              ++ runtimeInputs;
            };
          in
          {
            nativeBuildInputs = [
              pkgs.makeWrapper
            ];

            installPhase = ''
              runHook preInstall

              mkdir -p \
                "$out/share/$pname" \
                "$out/bin"

              cp -r ./. "$out/share/$pname"

              makeWrapper ${lib.getExe script} $out/bin/$pname \
                --chdir "$out/share/$pname"

              runHook postInstall
            '';
          };
      };
    };
}
