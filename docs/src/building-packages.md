# Building Packages

`bun2nix` provides a number of functions to aid building bun related packages:

> All of these functions are available as attributes on the `bun2nix` packages itself via it's [passthru](https://aux-docs.pyrox.pages.gay/Nixpkgs/Standard-Environment/passthru.chapter/).

- [`mkDerivation`](./building-packages/mkDerivation.md)
- [`hook`](./building-packages/hook.md)
- [`fetchBunDeps`](./building-packages/fetchBunDeps.md)
- [`writeBunScriptBin`](./building-packages/writeBunScriptBin.md)
