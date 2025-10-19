{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem.treefmt = {
    projectRootFile = "flake.nix";
    programs = {
      deadnix.enable = true;
      nixfmt.enable = true;
      statix.enable = true;
      shfmt.enable = true;
      shellcheck.enable = true;
      rustfmt.enable = true;
      prettier.enable = true;
      toml-sort.enable = true;
      zig.enable = true;
      mdformat.enable = true;
    };
  };
}
