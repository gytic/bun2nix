#!/usr/bin/env node

import { convert_lockfile_to_nix_expression } from "./bun2nix-wasm.js";

import sade from "sade";
import pkgJson from "./package.json";

const prog = sade("bun2nix", true);

type Opts = {
  "lock-file": string;
  "output-file": string | undefined;
};

async function run(opts: Opts) {
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
  .action((opts) => run(opts));

prog.parse(process.argv);

/** # Convert Bun Lockfile to a Nix expression
 *
 * Takes a string input of the contents of a bun lockfile and converts it into a ready to use Nix expression which fetches the packages
 */
export function convertLockfileToNixExpression(contents: string): string {
  return convert_lockfile_to_nix_expression(contents);
}
