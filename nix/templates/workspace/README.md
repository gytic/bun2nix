# Bun2Nix Workspace Template

This is a template for a Bun workspace project using bun2nix to create Nix derivations from Bun projects. It demonstrates how to build packages that reference other workspace packages.

## Structure

- `packages/app`: A simple application that depends on the workspace library
- `packages/lib`: A library package that is referenced by the application

## Building

```bash
# Build the app
nix build
```

## Development

```bash
# Enter a development shell with bun available
nix develop
```

## How It Works

This template demonstrates how to use the `workspaceRoot` parameter in `mkBunDerivation` to enable proper workspace package resolution when building with Nix. The `workspaceRoot` parameter tells bun2nix where to find the workspace packages so they can be properly linked into the node_modules directory.

The template showcases workspace dependencies in Bun with the `workspace:*` syntax in package.json.
