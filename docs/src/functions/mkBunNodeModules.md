# Create a valid `node_modules` directory with `mkBunNodeModules`

`mkBunNodeModules` is used by [`mkBunDerivation`](./functions/mkBunDerivation.md) for producing the node_modules directory it links to before building the app. When passed the `packages` list produced by the [bun2nix command line tool](./using-the-command-line-tool.md), it produces a fully valid `node_modules` directory as would be created by a fresh `bun install`.

## Example

Example usage of `mkBunNodeModules` might look like:

```nix
  bunNix = import ./bun.nix;

  node_modules = mkBunNodeModules { packages = bunNix };
```

## Arguments

The full list of accepted arguments is:

| Argument            | Purpose                                                         |
| ------------------- | --------------------------------------------------------------- |
| `packages`          | The contents of the `bun.nix` file.                             |
| `dontPatchShebangs` | (Optional) Prevent patching shebangs in `node_modules` scripts. |

By default, shebangs in scripts inside `node_modules` are patched to use `bun` instead of `node`. Use `dontPatchShebangs = true;` if you want to preserve the original shebangs (for example, to maintain compatibility with tools that expect Node.js).
