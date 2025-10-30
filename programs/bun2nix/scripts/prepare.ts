#!/usr/bin/env node

import { $ } from "bun";

await $`bun build index.ts --no-bundle --outfile=dist/index.js`;
await $`cp package.json ./dist`;
