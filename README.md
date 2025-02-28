# Bun2Nix

A fast rust based tool for converting [Bun](https://bun.sh/) (v1.2+) package lock files to [Nix](https://nixos.wiki/) expressions.

## Installation

### As a Flake

> TODO: write flake installation description

### From Cargo 

> TODO: package on cargo
> TODO: write cargo description

### From nixpkgs

> TODO: package on nix
> TODO: write nix description

## Usage

`bun2nix` can be invoked as a command as follows:

```
Usage: bun2nix [OPTIONS]

Options:
  -l, --lock-file <LOCK_FILE>      The Bun (v1.2+) lockfile to use to produce the Nix expression [default: ./bun.lock]
  -o, --output-file <OUTPUT_FILE>  The output file to write to - if no file location is provided, print to stdout instead
  -h, --help                       Print help
  -V, --version                    Print version
```

> TODO: detail how to place in a package.json so that bun2nix gets ran automatically on `bun install`
