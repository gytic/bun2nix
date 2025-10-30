{ config, ... }:
{
  perSystem =
    { final, ... }:
    {
      packages = {
        bun2nix-js = final.pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "bun2nix-js";
          inherit (config.cargoTOML.package) version;

          src = ../../programs/bun2nix;

          cargoLock = {
            lockFile = "${finalAttrs.src}/Cargo.lock";
          };

          bunDeps = final.bun2nix.fetchBunDeps {
            bunNix = "${finalAttrs.src}/bun.nix";
          };

          nativeBuildInputs = with final; [
            bun2nix.hook
            wasm-pack
            wasm-bindgen-cli_0_2_104
            binaryen
            cargo
            lld
            jq
          ];

          buildPhase = ''
            runHook preBuild

            bun run build

            runHook postBuild
          '';

          installPhase = ''
            mkdir "$out"

            cp -R ./dist "$out"
          '';

          doCheck = true;

          checkPhase = ''
            version="$(jq '.version' dist/package.json)"
            if ! [[ "$version" == "\"${config.cargoTOML.package.version}\"" ]]; then
              echo "Tag version \"$version\" does not match \"${config.cargoTOML.package.version}\" bun2nix js package'."
              exit 1
            fi
          '';
        });
      };
    };
}
