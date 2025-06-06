{
  pkgs,
  flake,
  system,
  ...
}:
let
  lib = pkgs.lib;

  templates = builtins.attrNames (
    lib.filterAttrs (_n: v: v == "directory") (builtins.readDir ../templates)
  );

  evalTemplate =
    template:
    pkgs.callPackage (../templates/${template}/default.nix) {
      inherit (flake.lib.${system}) mkBunDerivation;
    };

  templatesPkgs = lib.map evalTemplate templates;
in
pkgs.symlinkJoin {
  name = "template-eval-check";
  paths = templatesPkgs;
}
