{ inputs, ... }:
{
  imports = [
    inputs.flake-parts.flakeModules.easyOverlay
  ];

  perSystem =
    { self', ... }:
    {
      overlayAttrs = {
        inherit (self'.packages) bun2nix;
      };
    };
}
