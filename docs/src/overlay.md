# Overlay

`bun2nix` provides [an overlay](https://nixos.wiki/wiki/Overlays) which places the `bun2nix` binary (along with it's `passthru` functions) into `pkgs` for ease of use.

## Usage

### Add the overlay

```nix
{ bun2nix, nixpkgs, ... }:
let pkgs = (nixpkgs.legacyPackages.${system}.extend bun2nix.overlays.default);
```

### Use `bun2nix` from `pkgs`

```
{ pkgs, ... }:
pkgs.stdenv.mkDerivation {
  pname = "react-website";
  version = "1.0.0";

  src = ./.;

  nativeBuildInputs = [
    pkgs.bun2nix.hook
  ];

  bunDeps = pkgs.bun2nix.fetchBunDeps {
    bunNix = ./bun.nix;
  };

  buildPhase = ''
    bun run build \
      --minify
  '';

  installPhase = ''
    mkdir -p $out/dist

    cp -R ./dist $out
  '';
}
```
