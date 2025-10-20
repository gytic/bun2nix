{ lib, self, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      templates = "${self}/templates";
      filterDirectories = builtins.filter ({ value, ... }: value == "directory");
      evaluatePackages = builtins.map (
        { name, ... }:
        {
          "${name}" = pkgs.callPackage "${templates}/${name}/default.nix" { };
        }
      );
    in
    {
      checks = lib.pipe templates [
        builtins.readDir
        lib.attrsToList
        filterDirectories
        evaluatePackages
        lib.mergeAttrsList
      ];
    };
}
