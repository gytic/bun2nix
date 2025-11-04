{ lib, inputs, ... }:
let
  throwRemovedV2 =
    name:
    builtins.throw ''
      The `bun2nix.lib.''${system}.${name}` function has been 
      removed/replaced/moved with the breaking API changes for `bun2nix` v2.

      Please take a look at the v2 update guide in the documentation for a
      walk through on how to update.

      If this was sprung on you randomly with `nix flake update`,
      please consider pinning your `bun2nix` dependency's version with a 
      `tag` specifier:

      ```nix
      # Put the appropriate version here
      bun2nix.url = "github:baileyluTCD/bun2nix?tag=2.0.0";
      ```
    '';

  removedFns = [
    "mkBunDerivation"
    "mkBunNodeModules"
    "writeBunScriptBin"
  ];

  systems = import inputs.systems;
in
{
  flake.lib = lib.genAttrs systems (
    _:
    lib.pipe removedFns [
      (map (x: lib.nameValuePair x (throwRemovedV2 x)))
      builtins.listToAttrs
    ]
  );
}
