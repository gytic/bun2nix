# The Command Line Tool

## Recommended Usage

The recommended way to use `bun2nix` is by adding it to your `package.json` to automatically run after any package management option:

```json
"scripts": {
    "postinstall": "bun2nix -o bun.nix"
}
```

However, if you run without the `-o` flag it will produce a text output over stdout similar to other `lang2nix` tools, hence if you enforce formatting rules in your repository it is likely a good idea to pass it through a formatter before writing the file.

## Options

Currently, the options available from the command line tool are as follows:

```
Convert Bun (v1.2+) packages to Nix expressions

Usage: bun2nix [OPTIONS]

Options:
  -l, --lock-file <LOCK_FILE>      The Bun (v1.2+) lockfile to use to produce the Nix expression [default: ./bun.lock]
  -o, --output-file <OUTPUT_FILE>  The output file to write to - if no file location is provided, print to stdout instead
  -h, --help                       Print help
  -V, --version                    Print version
```
