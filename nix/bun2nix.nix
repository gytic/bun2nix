{
  self,
  lib,
  config,
  ...
}:
let
  rootConfig = config;
in
{
  perSystem =
    { pkgs, config, ... }:
    {
      packages.bun2nix = pkgs.rustPlatform.buildRustPackage (
        finalAttrs:
        let
          pkgInfo = rootConfig.cargoTOML.package;
        in
        {
          pname = pkgInfo.name;
          inherit (pkgInfo) version;

          src = "${self}/programs/bun2nix";

          cargoLock = {
            lockFile = "${finalAttrs.src}/Cargo.lock";
          };

          passthru = with config; {
            mkDerivation = mkDerivation.function;
            inherit (mkDerivation) hook;
            fetchBunDeps = fetchBunDeps.function;
          };

          meta = {
            description = "A fast rust based bun lockfile to nix expression converter.";
            homepage = "https://github.com/baileyluTCD/bun2nix";
            license = lib.licenses.mit;
            maintainers = [ lib.maintainers.baileylu ];
          };

        }
      );
    };

}
