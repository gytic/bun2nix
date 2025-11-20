# Building with `bun2nix.hook`

The `bun2nix` hook provides a simple way to extend an existing derivation with Bun dependencies by way of a [setup hook](https://nixos.org/manual/nixpkgs/unstable/#sec-pkgs.makeSetupHook).

This is especially useful for building things like websites or other artifacts that have a build artifact that is not an executable, or for building polyglot projects that need Bun dependencies on hand.

> [`mkDerivation`](./mkDerivation.md) is a thin wrapper over this with `stdenv.mkDerivation` with some extra goodies for building Bun executables.

## Example

You can use the `bun2nix` hook to integrate with an existing `stdenv.mkDerivation` style function by adding it to `nativeBuildInputs` like so:

```nix
{ stdenv, bun2nix, ... }:
stdenv.mkDerivation {
  pname = "my-react-website";
  version = "1.0.0";

  src = ./.;

  nativeBuildInputs = [
    bun2nix.hook
  ];

  bunDeps = bun2nix.fetchBunDeps {
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

## Troubleshooting

The default behavior of `bun2nix` is to hard-link installs from the Nix store. Unfortunately, this is not guaranteed to work the same on all systems - if you see strange permissions errors from `bun install` try setting `bunInstallFlags` to `--backend=symlink`, which works but may be marginally slower.

## Useful Functional Information

The `bun2nix` hook installs the fake [Bun install cache](https://github.com/oven-sh/bun/blob/642d04b9f2296ae41d842acdf120382c765e632e/docs/install/cache.md#L24) created by [`fetchBunDeps`](./fetchBunDeps.md) at `$BUN_INSTALL_CACHE_DIR`.

This is then installed into your repo via a regular `bun install` during `bunNodeModulesInstallPhase`, which runs before the `buildPhase`.

## Arguments

The full list of extra arguments `bun2nix.hook` adds to a derivation are:

| Argument                  | Purpose                                                                                                                                                                                                                                                                                       |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bunDeps`                 | The output of [`fetchBunDeps`](./fetchBunDeps.md) (or any other Nix derivation which produces a Bun-compatible install cache). This is required.                                                                                                                                              |
| `bunBuildFlags`           | Flags to pass to Bun in the default Bun build phase                                                                                                                                                                                                                                           |
| `bunCheckFlags`           | Flags to pass to Bun in the default Bun check phase                                                                                                                                                                                                                                           |
| `bunInstallFlags`         | Flags to pass to `bun install`. If not set these default to "--linker=isolated --backend=symlink" on `aarch64-darwin` or "--linker=isolated" on other systems                                                                                                                                 |
| `dontRunLifecycleScripts` | By default, after `bunNodeModulesInstallPhase` runs `bun install --ignore-scripts`, `bunLifecycleScriptsPhase` runs any missing lifecycle scripts after making the `node_modules` directory writable and executable. This attribute can be used to disable running `bunLifecycleScriptsPhase` |
| `dontUseBunPatch`         | Don't patch any shebangs in your `src` directory to use Bun as their interpreter                                                                                                                                                                                                              |
| `dontUseBunBuild`         | Disable the default build phase                                                                                                                                                                                                                                                               |
| `dontUseBunCheck`         | Disable the default check phase                                                                                                                                                                                                                                                               |
| `dontUseBunInstall`       | Disable the default install phase                                                                                                                                                                                                                                                             |

## New Build Phases

The `bun2nix` hook introduces a number of new build phases which are worth knowing about:

> These all have `pre` and `post` run hooks available

| Phase                        | Purpose                                                                                     |
| ---------------------------- | ------------------------------------------------------------------------------------------- |
| `bunPatchPhase`              | Before doing anything, patch shebangs of your local scripts to use Bun as their interpreter |
| `bunNodeModulesInstallPhase` | Runs `bun install` in your `src` repo                                                       |
| `bunLifecycleScriptsPhase`   | Runs any Bun lifecycle scripts (i.e., "install", etc.) after making `node_modules` writable |
