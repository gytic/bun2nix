{
  flake,
  pkgs,
  system,
  ...
}:
pkgs.callPackage ../templates/default/default.nix {
  inherit (flake.lib.${system}) mkBunDerivation;
}
