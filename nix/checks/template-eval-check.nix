{ lib, self, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      templates = "${self}/templates";
    in
    {
      checks = lib.pipe templates [
        builtins.readDir
        lib.attrsToList
        builtins.filter
        ({ value, ... }: value == "directory")
        builtins.map
        ({ name, ... }: pkgs.callPackage "${templates}/${name}/default.nix")
      ];
    };
}
