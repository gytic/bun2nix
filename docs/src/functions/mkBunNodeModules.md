# Create a valid `node_modules` directory with `mkBunNodeModules`

`mkBunNodeModules` is used by [`mkBunDerivation`](./functions/mkBunDerivation.md) for producing the node_modules directory it links to before building the app. When passed the `packages` list produced by the [bun2nix command line tool](./using-the-command-line-tool.md), it produces a fully valid `node_modules` directory as would be created by a fresh `bun install`.

## Example

Example usage of `mkBunNodeModules` might look like:

```nix
  bunNix = import ./bun.nix;

  node_modules = mkBunNodeModules bunNix;
```

## Arguments

`mkBunNodeModules` takes a single argument, the contents of the `bun.nix` file.
