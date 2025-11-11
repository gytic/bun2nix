{ bun2nix, ... }:
bun2nix.writeBunApplication {
  packageJson = ./package.json;

  src = ./.;

  buildPhase = ''
    bun run build
  '';

  startScript = ''
    bun run start
  '';

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./bun.nix;
  };
}
