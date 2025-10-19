# Workspace template for bun2nix
{
  mkBunDerivation,
}:

mkBunDerivation {
  pname = "workspace-test-app";
  version = "1.0.0";

  src = ./.;
  bunNix = ./bun.nix;

  index = "packages/app/index.js";
}
