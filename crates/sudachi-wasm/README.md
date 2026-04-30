# sudachi-wasm

**Sudachi Japanese tokenizer for browsers and Node.js.**

A `wasm-bindgen` wrapper around `sudachi-search` that ships B+C multi-granularity tokenisation as a WebAssembly module. Targets `web`, `nodejs`, and `bundler`.

---

## What you get

```js
import init, { SudachiTokenizer } from './pkg/sudachi_wasm.js';

await init();

const dictBytes = new Uint8Array(
  await (await fetch('/system_full.dic')).arrayBuffer()
);
const tokenizer = new SudachiTokenizer(dictBytes);

const tokens = tokenizer.tokenize("東京都立大学で研究");
// [
//   { surface: "東京都立大学", byteStart: 0,  byteEnd: 18, isColocated: false },
//   { surface: "東京",         byteStart: 0,  byteEnd: 6,  isColocated: true  },
//   { surface: "都立",         byteStart: 6,  byteEnd: 12, isColocated: true  },
//   { surface: "大学",         byteStart: 12, byteEnd: 18, isColocated: true  },
//   { surface: "で",           byteStart: 18, byteEnd: 21, isColocated: false },
//   { surface: "研究",         byteStart: 21, byteEnd: 27, isColocated: false },
// ]
```

`isColocated: true` tokens are at the **same position** as the immediately preceding compound — sub-tokens within `東京都立大学`, in this case.

---

## Build

```bash
just wasm-build           # ES module for browsers (target=web)
just wasm-build-node      # CommonJS for Node.js
just wasm-build-bundler   # webpack / vite / rollup
just wasm-build-dev       # development build (faster, unoptimised)
```

Output lands in `crates/sudachi-wasm/pkg/`:

```
pkg/
├── sudachi_wasm.js          ES module / CJS wrapper
├── sudachi_wasm.d.ts        TypeScript declarations
├── sudachi_wasm_bg.wasm     The wasm binary
├── sudachi_wasm_bg.wasm.d.ts
└── package.json
```

---

## Demo

```bash
just wasm-build       # build for the browser target first
just wasm-serve       # serves the example at http://localhost:3000/example/
```

The demo at `example/index.html` lets you load a `.dic` file or paste a URL and tokenise text in the browser. `example/node.mjs` is the equivalent for Node.

---

## API

```ts
class SudachiTokenizer {
  /**
   * Build a tokenizer from raw dictionary bytes (Uint8Array).
   * Caller is responsible for fetching / loading the bytes.
   * Default: normalised form (better recall — 食べた → 食べる, etc.)
   */
  constructor(dictBytes: Uint8Array);

  /**
   * B+C tokenisation. Returns objects with surface, byteStart, byteEnd,
   * isColocated. Colocated tokens are at the same logical position as
   * the previous primary token.
   */
  tokenize(text: string): Token[];

  /**
   * Convenience: just the surface forms of primary tokens (drops colocated).
   */
  tokenize_surfaces(text: string): string[];

  /**
   * Detect compound words. Returns objects with surface, components,
   * byteStart, byteEnd. Only emits compounds (more than one component).
   */
  detect_compounds(text: string): Compound[];

  /**
   * Switch to surface form (raw input text instead of normalised).
   * Returns a new tokenizer; the original is consumed.
   */
  with_surface_form(): SudachiTokenizer;
}

interface Token {
  surface: string;
  byteStart: number;
  byteEnd: number;
  isColocated: boolean;
}

interface Compound {
  surface: string;
  components: string[];   // Mode B parts
  byteStart: number;
  byteEnd: number;
}
```

---

## Shipping the dictionary

The Sudachi full dictionary is ~70 MB. Strategies:

| Strategy                              | When to use                                     |
| ------------------------------------- | ----------------------------------------------- |
| `fetch()` from your CDN               | Browser apps; cache via service worker          |
| `fs.readFileSync` in Node             | CLI / server tools                              |
| `include_bytes!` at compile time      | Single-file bundles (huge wasm — avoid for web) |

The constructor takes `Uint8Array`; how you obtain it is up to you.

---

## Why a wasm crate

The wasm32 build of upstream `sudachi.rs` historically failed because of `libloading` (used for plugin DSO loading on native platforms). The workspace `Cargo.toml` applies a `[patch]` that gates `libloading` behind `cfg(not(target_family = "wasm"))`. With that patch in place, this crate builds for `wasm32-unknown-unknown` cleanly.

The crate itself is a thin shim — see `src/lib.rs` (~145 LOC). All the work happens in `sudachi-search`.

---

## License

Apache-2.0
