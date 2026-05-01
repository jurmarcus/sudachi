# kwja-rs

Pure-Rust port of [KWJA](https://github.com/ku-nlp/kwja) inference.

## Scope

- typo module (sentence-level typo correction)
- char module (sentence segmentation)
- word module (POS, lemma, dependency parse, BasePhrase tree, NER)

## Out of scope

- seq2seq (T5-based typo correction — typo module suffices)
- cohesion analyzer
- discourse parser
- anaphora resolution

## Backend

candle. Default `metal` (Apple Silicon). Production `cuda`.

```bash
cargo build                              # macOS, metal
cargo build --no-default-features --features cuda   # Linux, CUDA
```

## License

MIT OR Apache-2.0 (matches KWJA upstream).
