{
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
      packages = rec {
        bun2nix = pkgs.rustPlatform.buildRustPackage (
          finalAttrs:
          let
            pkgInfo = rootConfig.cargoTOML.package;
          in
          {
            pname = pkgInfo.name;
            inherit (pkgInfo) version;

            src = ../programs/bun2nix;

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
        default = bun2nix;
        bun2nix-wasm = bun2nix.overrideAttrs (
          _finalAttrs: _previousAttrs: {
            nativeBuildInputs = with pkgs; [
              wasm-pack
            ];

            buildPhase = ''
              wasm-pack build
            '';

            installPhase = ''
              mkdir $out
              mv ./pkg $out
            '';
          }
        );
      };
    };

}
