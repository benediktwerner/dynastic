import { compileFromFile } from 'json-schema-to-typescript';
import * as fs from 'node:fs/promises';
import * as path from 'path';

const style = {
  bracketSpacing: false,
  printWidth: 120,
  semi: true,
  singleQuote: false,
  tabWidth: 2,
  trailingComma: 'es5',
  useTabs: false,
};
const bannerComment = `/* eslint-disable */
/**
 * This file was automatically generated by schemar and json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the underlying Rust types in \`src/model.rs\`,
 * and run \`./gen-ts.sh\` to regenerate this file.
 */`;

const args = process.argv.slice(2);
if (args.length !== 2) {
  console.log('Usage: node gen-ts.js <schemas dir> <out>.d.ts');
  process.exit(1);
}

const inDir = args[0];
const outPath = args[1];

async function main() {
  const schemaFiles = (await fs.readdir(inDir)).filter((x) =>
    x.endsWith('.json')
  );

  const out = new Set();

  for (const schemaFile of schemaFiles) {
    const compiled = await compileFromFile(path.join(inDir, schemaFile), {
      style,
      bannerComment: '',
    });
    compiled
      .split('export')
      .filter(Boolean)
      .forEach((t) => out.add(`export ${t.trim()}`));
  }

  const output = [bannerComment, ...out].join('\n\n') + "\n";
  try {
    const existing = await fs.readFile(outPath);
    if (existing === output) {
      // Skip writing if it hasn't changed, so that we don't confuse any sort of incremental builds.
      console.log('Schemas are up to date');
      return;
    }
  } catch (e) {
    // It's fine if there's no output from a previous run.
    if (e.code !== 'ENOENT') {
      throw e;
    }
  }

  await fs.writeFile(outPath, output);
  console.log(`Wrote types to ${outPath}`);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
