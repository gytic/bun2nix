#!/usr/bin/env bun

import { convert_lockfile_to_nix_expression } from "./bun2nix-wasm.js";

import sade from "sade";
import pkgJson from "./package.json" with { type: "json" };

const prog = sade("bun2nix", true);

/** `bun2nix` command line options. */
type Opts = {
  /** String path to the lockfile to read in. */
  "lock-file": string;
  /** Output file to write to - writes to stdout if undefined. */
  "output-file": string | undefined;
};

/**
 * Generate a nix expression for a given bun lockfile
 * Writes to stdout if `output-file` is not specified.
 *
 * @param {Opts} opts - An instance of bun2nix CLI options
 */
export async function generateNixExpression(opts: Opts): Promise<void> {
  const lock_file = Bun.file(opts["lock-file"]);
  const contents = await lock_file.text();

  const nix_expression = convert_lockfile_to_nix_expression(contents);

  const output_file = opts["output-file"] || Bun.stdout;
  await Bun.write(output_file, nix_expression + "\n");
}

prog
  .version(pkgJson.version)
  .describe("Convert Bun (v1.2+) packages to Nix expressions")
  .option(
    "-l, --lock-file",
    "The Bun (v1.2+) lockfile to use to produce the Nix expression",
    "bun.lock",
  )
  .option(
    "-o, --output-file",
    "The output file to write to - if no file location is provided, print to stdout instead",
  )
  .action((opts) => generateNixExpression(opts));

prog.parse(process.argv);

/**
 * Convert Bun Lockfile to a Nix expression
 *
 * @param {string} contents - The contents of a bun lockfile
 * @return {string} The generated nix expression
 */
export function convertLockfileToNixExpression(contents: string): string {
  return convert_lockfile_to_nix_expression(contents);
}
