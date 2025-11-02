# Fetch Bun Deps

`fetchBunDeps` is a handy function responsible for creating a [bun compatible cache](https://github.com/oven-sh/bun/blob/642d04b9f2296ae41d842acdf120382c765e632e/docs/install/cache.md#L24) for doing offline installs off of.

## Example

You should use `fetchBunDeps` in conjunction with the rest of `bun2nix` to build your bun packages like so:

```nix
{
  bun2nix,
  ...
}:
bun2nix.mkDerivation {
  pname = "workspace-test-app";
  version = "1.0.0";

  src = ./.;

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./bun.nix;
  };

  module = "packages/app/index.js";
}
```

## Arguments

`fetchBunDeps` is designed to offer a number of flexible options for customizing your bun install process:

| Argument        | Purpose                                                                                                                                                                                                                                                                                                  |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bunNix`        | The `bun.nix` file as created by the [bun2nix cli](../using-the-command-line-tool.md)                                                                                                                                                                                                                    |
| `overrides`     | Allows for modifying packages before install in the nix store to patch any broken dependencies. See the overriding section below                                                                                                                                                                         |
| `useFakeNode`   | By default, `bun2nix` patches any scripts that use node in your dependencies to use `bun` as it's executable instead. Turning this off will patch them to use `node` instead. This might be useful, if, for example, you need to link to actual node v8 while building a native addon. Defaults to true. |
| `patchShebangs` | If scripts in your dependencies should have their shebangs patched or not. Defaults to true.                                                                                                                                                                                                             |

## Overrides

`fetchBunDeps` provides an overrides api for modifying packages in the nix store before they become a part of bun's install cache and ultimately your project's node modules.

You may want to use this to patch a dependency which, for example, makes a network request during install and fails because it's sandboxed by nix.

### Type

Each override attribute name must be a key which exists in your `bun.nix` file, and attribute value a function which takes a derivation and returns another one.

### Example

```nix
bunDeps = bun2nix.fetchBunDeps {
  bunNix = ./bun.nix;
    overrides = {
      "typescript@5.7.3" =
        pkg:
        runCommandLocal "override-example" { } ''
          mkdir $out
          cp -r ${pkg}/. $out

          echo "hello world" > $out/my-override.txt
        '';
    };
};

# Assertation will not fail
postBunNodeModulesInstallPhase = ''
  if [ ! -f "node_modules/typescript/my-override.txt" ]; then
    echo "Text file created with override does not exist."
    exit 1
  fi
'';
```
