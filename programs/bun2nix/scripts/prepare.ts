#!/usr/bin/env node

import { $ } from "bun";

await $`bun build index.ts --no-bundle --outfile=dist/index.js`;
await $`cp package.json ./dist`;

const transpiled = await $`cat dist/index.js"`.text();

const with_shebang = `#!/usr/bin/env node

${transpiled}
`;

await $`echo "${with_shebang}" > dist/index.js`;
