{
  bun2nix,
  stdenv,
  lib,
  ...
}:
bun2nix.writeBunApplication {
  packageJson = ./package.json;

  src = ./.;

  # all cpus are needed for x86 darwin builds
  bunInstallFlags = lib.optionals (stdenv.hostPlatform.system == "x86_64-darwin") [
    "--linker=isolated"
    "--backend=symlink"
    "--cpu=*"
  ];

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
