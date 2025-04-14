{
  description = "Create nix expressions from bun lockfiles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    blueprint.url = "github:numtide/blueprint";
    blueprint.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs:
    inputs.blueprint {
      inherit inputs;
      prefix = "nix/";
    };

  # outputs = {
  #   nixpkgs,
  #   flake-utils,
  #   pre-commit-hooks,
  #   ...
  # }:
  #   flake-utils.lib.eachDefaultSystem (system: let
  #     pkgs = import nixpkgs {inherit system;};
  #
  #     bun2nix = pkgs.callPackage ./default.nix {};
  #   in {
  #     defaultPackage = bun2nix;
  #
  #     defaultApp = {
  #       type = "app";
  #       program = "${bun2nix.bin}/bin/bun2nix";
  #     };
  #
  #     checks = {
  #       pre-commit-check = pre-commit-hooks.lib.${system}.run {
  #         src = ./.;
  #         hooks = {
  #           cargo-check.enable = true;
  #           clippy.enable = true;
  #           rustfmt.enable = true;
  #         };
  #       };
  #     };
  #
  #   });
}
