{ self, lib, ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      packages.bun2nix = pkgs.rustPlatform.buildRustPackage (
        finalAttrs:
        let
          cargoTOML = builtins.fromTOML (builtins.readFile "${finalAttrs.src}/Cargo.toml");
        in
        {
          pname = cargoTOML.package.name;
          inherit (cargoTOML.package) version;

          src = "${self}/programs/bun2nix";

          cargoLock = {
            lockFile = "${finalAttrs.src}/Cargo.lock";
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
