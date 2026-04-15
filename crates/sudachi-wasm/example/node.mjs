/**
 * Node.js integration test for sudachi-wasm.
 *
 * Usage:
 *   just wasm-build-node          # build first
 *   SUDACHI_DICT_PATH=~/.sudachi/system_full.dic node example/node.mjs
 *
 * Or with an explicit path:
 *   node example/node.mjs /path/to/system_full.dic
 */

import { readFile } from 'fs/promises';
import { homedir } from 'os';
import { join } from 'path';

// ---- Load WASM module -------------------------------------------------------

const { default: init, SudachiTokenizer } = await import('../pkg/sudachi_wasm.js');
await init();

// ---- Find dictionary --------------------------------------------------------

const dictPath =
  process.argv[2] ||
  process.env.SUDACHI_DICT_PATH ||
  join(homedir(), '.sudachi', 'system_full.dic');

console.log(`Loading dictionary: ${dictPath}`);
const dictBytes = await readFile(dictPath);
const tokenizer = new SudachiTokenizer(new Uint8Array(dictBytes.buffer));
console.log(`Dictionary loaded (${(dictBytes.length / 1_000_000).toFixed(1)} MB)\n`);

// ---- Test cases -------------------------------------------------------------

const tests = [
  '東京都立大学で研究しています',
  '食べた魚が美味しかった',
  '予約困難店を探す',
];

for (const text of tests) {
  console.log(`── ${text}`);

  // Full B+C tokens
  const tokens = tokenizer.tokenize(text);
  for (const t of tokens) {
    const marker = t.isColocated ? ' (colocated)' : '';
    console.log(`   ${t.surface.padEnd(12)}${marker}`);
  }

  // Surface-only (no colocated sub-tokens)
  const surfaces = tokenizer.tokenizeSurfaces(text);
  console.log(`   → surfaces: [${surfaces.join(', ')}]`);

  // Compound words
  const compounds = tokenizer.detectCompounds(text);
  if (compounds.length > 0) {
    for (const c of compounds) {
      console.log(`   → compound: ${c.surface} = [${c.components.join(' + ')}]`);
    }
  }

  console.log();
}
