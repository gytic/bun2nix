{ inputs, flake, ... }:
let
  eachSystem = inputs.nixpkgs.lib.genAttrs (import inputs.systems);
in
eachSystem (
  system:
  let
    pkgs = inputs.nixpkgs.legacyPackages.${system};
  in
  rec {
    mkDotBunDir = pkgs.callPackage ./mkDotBunDir.nix {
      inherit (flake.packages.${system}) cache-entry-creator;
    };
    writeBunScriptBin = pkgs.callPackage ./writeBunScriptBin.nix { };

    mkBunDerivation = pkgs.callPackage ./mkBunDerivation.nix {
      inherit mkDotBunDir;
      inherit (flake.packages.${system}) cache-entry-creator;
    };

    treefmt = inputs.treefmt-nix.lib.evalModule pkgs (import ./treefmt-config.nix);
  }
)
