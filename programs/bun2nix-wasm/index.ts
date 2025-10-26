#!/usr/bin/env node

import sade from "sade";
import pkgJson from "./package.json";

const prog = sade("bun2nix", true);

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
  );

prog.parse(process.argv);
