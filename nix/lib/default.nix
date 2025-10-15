{ inputs, ... }:
let
  eachSystem = inputs.nixpkgs.lib.genAttrs (import inputs.systems);
in
eachSystem (
  system:
  let
    pkgs = inputs.nixpkgs.legacyPackages.${system};
  in
  rec {
    mkDotBunDir = pkgs.callPackage ./mkDotBunDir.nix { };
    writeBunScriptBin = pkgs.callPackage ./writeBunScriptBin.nix { };

    mkBunDerivation = pkgs.callPackage ./mkBunDerivation.nix { inherit mkDotBunDir; };

    treefmt = inputs.treefmt-nix.lib.evalModule pkgs (import ./treefmt-config.nix);
  }
)
