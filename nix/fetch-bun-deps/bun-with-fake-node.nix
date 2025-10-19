{ lib, flake-parts-lib, ... }:
let
  inherit (flake-parts-lib) mkPerSystemOption;
  inherit (lib) mkOption types;
in
{
  options.perSystem = mkPerSystemOption {
    options.fetchBunDeps.bunWithFakeNode = mkOption {
      description = ''
        Copy of nixpkgs's bun package containing an extra
        binary `node` which aliases to the `bun` binary output
        of the original package
      '';
      type = types.package;
    };
  };

  config.perSystem =
    { pkgs, ... }:
    {
      fetchBunDeps.bunWithFakeNode = pkgs.stdenvNoCC.mkDerivation {
        name = "fake-node";

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
      };
    };
}
