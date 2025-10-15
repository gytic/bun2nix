{ flake, pkgs, system, ... }:
  pkgs.callPackage ../templates/react/default.nix {
    inherit (flake.lib.${system}) mkBunDerivation;
  }
