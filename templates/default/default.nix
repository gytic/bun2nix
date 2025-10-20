{ bun2nix, ... }:
bun2nix.mkDerivation {
  pname = "minimal-bun2nix-example";
  version = "1.0.0";

  src = ./.;

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./bun.nix;
  };

  module = "index.ts";
}
