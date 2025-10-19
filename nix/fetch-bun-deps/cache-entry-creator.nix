{ self, config, ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      packages.cacheEntryCreator = pkgs.stdenvNoCC.mkDerivation (
        finalAttrs:
        let
          depsNix = "${finalAttrs.src}/deps.nix";
        in
        {
          pname = "bun2nix-cache-entry-creator";
          inherit (config) version;

          src = "${self}/programs/cache-entry-creator";

          nativeBuildInputs = with pkgs; [
            zig.hook
          ];

          postPatch = ''
            ln -s ${pkgs.callPackage depsNix { }} $ZIG_GLOBAL_CACHE_DIR/p
          '';

          buildPhase = ''
            zig build --release=fast
          '';

          doCheck = true;

          meta = {
            description = "Cache entry creator for bun packages";
            longDescription = ''
              Uses bun's specific `wyhash` implementation to calculate
              the correct location in which to place a cache entry for
              a given package after the tarball has been downloaded and
              extracted.
            '';
            mainProgram = "cache_entry_creator";
          };
        }
      );
    };

}
