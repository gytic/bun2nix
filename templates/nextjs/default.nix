{ bun2nix, ... }:
bun2nix.writeBunApplication {
  packageJson = ./package.json;

  src = ./.;

  buildPhase = ''
    bun run build
  '';

  # nextjs needs to bind to a port during the build process
  __darwinAllowLocalNetworking = true;

  startScript = ''
    bun run start
  '';

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./bun.nix;
  };
}
