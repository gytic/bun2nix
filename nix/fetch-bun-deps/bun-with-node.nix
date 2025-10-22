{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.bunWithNode = mkOption {
      description = ''
        Copy of nixpkgs's bun package containing an extra
        binary `node` which aliases to the `bun` binary output
        of the original package
      '';
      type = types.functionTo types.package;
    };
  };

  config.perSystem =
    { pkgs, ... }:
    {
      fetchBunDeps.bunWithNode =
        {
          useFakeNode ? true,
          ...
        }:
        if useFakeNode then
          pkgs.stdenvNoCC.mkDerivation {
            name = "bun-with-fake-node";

            nativeBuildInputs = with pkgs; [
              makeWrapper
            ];

            dontUnpack = true;
            dontBuild = true;

            installPhase = ''
              cp -r "${pkgs.bun}/." "$out"
              chmod +w $out/bin
              makeWrapper "$out/bin/bun" "$out/bin/node"
            '';
          }
        else
          pkgs.symlinkJoin {
            name = "bun-with-real-node";
            paths = with pkgs; [
              bun
              nodejs
            ];
          };
    };
}
