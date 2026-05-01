# AGENTS.md — sudachi-kwja

Context for AI agents working on the KWJA inference port.

## What you are working on

A pure-Rust port of [KWJA v2.4](https://github.com/ku-nlp/kwja) — Kyoto University & Waseda's multi-task Japanese analyzer. Specifically the **inference path** for the typo, char, and word modules. Training is not in scope; this crate consumes Lightning checkpoints exported via `scripts/convert_checkpoints.py`.

Backbone is **DeBERTa-v2 base** Japanese (HF `ku-nlp/deberta-v2-base-japanese{,-char-wwm}`). 12 task heads on the word module; 1 head consumed from the char module (sentence segmentation); 2 heads on the typo module (kdr + ins).

Runtime: [candle](https://github.com/huggingface/candle) 0.8 with vendored fp16 patches for DeBERTa attention. Default device is CUDA on Linux production, Metal on macOS dev, CPU as fallback.

## What this crate is NOT for

- ❌ Tokenizing Japanese (Sudachi's job — see `sudachi-optimizer`).
- ❌ Training KWJA from scratch (use upstream KWJA-Python).
- ❌ Re-implementing transformers/bert from scratch (use `candle-transformers`).
- ❌ Hosting a network service (that's `jisho-parse` in the jisho monorepo).
- ❌ Rust-side dictionary lookups for `代表表記` etc. (that's `services/jisho-parse/src/jumandic.rs`).

The crate is intentionally a **library** with one orchestrator (`Pipeline`) and 12 head implementations. Network protocol, dictionary enrichment, and batching policy beyond length-bucketing belong in callers.

## Two-path pipeline

```
parse(texts)
  ├─ CharModel.logits           → sent_segmentation argmax → sentence boundaries
  ├─ WordModel.forward          → WordLogits per sentence
  └─ decode_element_from_logits → Document tree

parse_morphemes(sentences)            ← jisho hot path
  ├─ length-bucket sort              ← 4 buckets when n_chunks ≥ 8
  ├─ WordModel.forward_pretokenized_batch (per bucket)
  └─ decode_element_from_logits      ← Sudachi data + KWJA structural data
```

`parse_morphemes` is the hot path. ~99% of production traffic uses it. The full `parse` exists for KWJA-equivalence testing and for callers without Sudachi.

## The 12 word heads — quick reference

| Head | Type | Output shape | Used as |
|---|---|---|---|
| `reading_tagger` | SequentialMlpHead | (1, T_subwords, R) | Subword reading classes (Sudachi-overridden) |
| `pos_tagger` | SequentialMlpHead | (1, T_words, 14) | KWJA POS (Sudachi-mapped) |
| `subpos_tagger` | SequentialMlpHead | (1, T_words, 35) | KWJA sub-POS (Sudachi-mapped) |
| `conjtype_tagger` | SequentialMlpHead | (1, T_words, 33) | Conj type (Sudachi-overridden) |
| `conjform_tagger` | SequentialMlpHead | (1, T_words, 81) | Conj form (Sudachi-overridden) |
| `ne_tagger` | SequentialMlpHead | (1, T_words, 17) | BIO NE tags |
| `dependency_parser` | BiaffineDependencyHead | (1, T_words, T_words, 1) | Pairwise dep parent scores |
| `dependency_type_parser` | SequentialMlpHead | (1, T_words, num_dep_types) | D / P / A / I edge type |
| `word_feature_tagger` | LoRASequenceMultiLabelingHead | (1, T_words, num_word_features) | Per-word multi-label sigmoid |
| `base_phrase_feature_tagger` | LoRASequenceMultiLabelingHead | (1, T_words, num_bp_features) | Per-word multi-label sigmoid (BP-level) |
| `cohesion_analyzer` | LoRARelationWiseWordSelectionHead | (1, T_words, T_words, R) | PAS / bridging / coreference |
| `discourse_relation_analyzer` | WordSelectionHead | (1, T_words, T_words, D) | Cross-sentence discourse |

## Hard rules

These will silently break equivalence or production if violated. Don't deviate without explicit reason.

1. **Don't re-tokenize.** Sudachi owns tokenization. KWJA word module is fed pre-tokenized input via `forward_pretokenized` whenever `parse_morphemes` is the entry point. Sudachi data wins for `surface` / `reading` / `lemma` / `conjtype` / `conjform`.
2. **Argmax-identical with KWJA-Python.** This is checked by `tests/equivalence/`. If you change a decode path, regenerate fixtures via `scripts/gen_fixtures.py` and verify the JSON is byte-equal — or, if a deliberate divergence, document it in CHANGELOG and update the fixture spec in `tests/equivalence/README.md`.
3. **Determinism in JSON.** Use `Vec<KeyValue>` for `features` / `semantics`. Never `HashMap`, never `BTreeMap` (KWJA-Python emits insertion-ordered list-of-objects; production JSONB stored in `passage_parse_tree.tree` matches that).
4. **fp16 on CUDA, fp32 elsewhere.** `DebertaBackbone::from_checkpoint` defaults this. The vendored candle-transformers patches in `~/code/shares/jisho/kwja-rs/vendor/candle/` cast f32 scalar literals (`XSoftmax`, `disentangled_attention_bias`) to `input.dtype()`. Don't drop the patches.
5. **`kwja_relative_pos` builds on CPU.** candle's CUDA backend lacks `uabs_i64`. We build relative position bias on CPU then `.to_device()`. Don't try to fuse it back to GPU until candle supports the op.
6. **Length-bucketed batching belongs in `pipeline.rs`.** Not in callers, not in `jisho-parse`. Callers tune via `BucketingConfig` + `parse_morphemes_with_config`. Default threshold (50 chunks) is data-driven — below that, single-bucket beats 4-bucket because launch overhead exceeds padding savings.
7. **Cohesion mask v3: anaphoric backward-only.** PAS and apposition can point either direction; ノ (`の`) and `=` (coreference) must point backward only (`tgt < src_word`). The mask is applied in-place on `cohesion_logits` before argmax. Don't relax this without checking equivalence fixtures.
8. **Cross-sentence discourse uses whole-element forward.** `parse_morphemes` runs each input element (potentially multi-sentence) as one row, then the decoder splits per-sentence using `WordLogits::slice_word_axis` for sentence-level outputs and uses the full tensor for cross-sentence pairs. Splitting before forward kills cross-sentence attention.
9. **DeBERTa-v2 base, not large.** `HIDDEN_SIZE = 768` baked in. Don't add a runtime toggle — head shapes are inferred from this constant.
10. **The crate is `MIT OR Apache-2.0`** (matches KWJA upstream). Don't tighten.

## File map

```
Cargo.toml                  Package + features (default = ["metal"], optional ["cuda"])
README.md                   Public docs
CLAUDE.md                   Crate-level Claude context
AGENTS.md                   This file
CHECKPOINTS.md              sha256s for converted safetensors

src/
├── lib.rs                  Public re-exports
├── checkpoint.rs           Checkpoint::load (safetensors + Device)
├── constants.rs            HIDDEN_SIZE, IGNORE_INDEX, label-list bindings
├── crf.rs                  Linear-chain CRF helpers
├── error.rs                Error enum (thiserror, source-preserving)
├── pipeline.rs             ★ Pipeline + parse + parse_morphemes
│                           + bucketing + cohesion_mask_v3
│                           + sudachi_to_kwja_pos
├── document/{mod,tree}.rs  Document tree types + serde
├── tokenizer/{char_,deberta,typo}.rs   HF tokenizer wrappers
├── model/
│   ├── deberta.rs          DebertaBackbone (candle-transformers wrapper)
│   ├── pool.rs             pool_subwords (mean / first reduction)
│   ├── heads.rs            All 6 head types
│   ├── char_.rs            CharModel
│   ├── word.rs             WordModel + WordLogits
│   └── typo.rs             TypoModel + TypoLogits
scripts/
├── convert_checkpoints.py  Lightning → safetensors
├── gen_fixtures.py         Per-head logit fixtures from KWJA-Python
└── pyproject.toml
resources/
└── labels.json             All label tables
examples/
├── dump_tensor_names.rs    Print loaded tensor names
└── test_pos_mapping.rs     Sanity-check Sudachi → KWJA POS map
tests/
└── equivalence/            Argmax fixtures vs KWJA-Python v2.4
```

## Common tasks

### Add a new head

1. Pick the right type in `model/heads.rs` (or add a new struct if none fit).
2. Add the field to `WordModel` (or `CharModel` / `TypoModel`).
3. Wire its `forward` call in the model's `forward` method, populating a new field on `WordLogits`.
4. Decode in `pipeline.rs::decode_element_from_logits`.
5. Add a label list to `resources/labels.json` if needed and bind in `constants.rs`.
6. Regenerate fixtures: `cd scripts && uv run python gen_fixtures.py --head <name>`.
7. `cargo test -p sudachi-kwja -- --include-ignored` to verify equivalence.

### Switch to DeBERTa-v2 large

1. Update `HIDDEN_SIZE` in `src/constants.rs` (768 → 1024).
2. Update `NUM_HIDDEN_LAYERS` if applicable (12 → 24).
3. Replace `.safetensors` with the large-variant ones (re-run `convert_checkpoints.py` against the large `.ckpt`).
4. Re-record sha256s in `CHECKPOINTS.md`.
5. Regenerate fixtures (training corpus is the same; argmax may differ).
6. Bench: large is ~3-4× slower per token. Make sure throughput targets still met.

### Add a Sudachi POS → KWJA POS mapping

1. Edit `pipeline.rs::sudachi_to_kwja_pos`. UniDic conventions on the input side; JUMAN on the output side.
2. Add a case to `examples/test_pos_mapping.rs`.
3. `cargo run -p sudachi-kwja --example test_pos_mapping` to verify.

### Update the cohesion mask

The cohesion analyzer's output is post-processed before argmax to enforce structural constraints. Current rules (mask v3):

- ノ (genitive anaphora), `=` (coreference): target must precede source (`tgt < src_word`).
- ガ / ヲ / ニ / ガ２ etc. (case roles): bidirectional.
- App / Bridge: bidirectional.

Modifying these rules risks breaking equivalence — regenerate fixtures and run `tests/equivalence/`.

### Diagnose a tensor-shape mismatch

```bash
cargo run -p sudachi-kwja --example dump_tensor_names -- /path/to/checkpoint.safetensors
```

Compare against KWJA-Python via:
```python
import torch
ck = torch.load("/path/word_deberta-v2-base.ckpt", map_location="cpu")
print(list(ck["state_dict"].keys()))
```

The `convert_checkpoints.py` script strips a `model.` prefix; if shapes mismatch on load, that's the most common cause.

### Check if you need to update the vendored candle patches

```bash
rg "input.dtype\(\)\)" ~/code/shares/jisho/kwja-rs/vendor/candle/candle-transformers/src/models/debertav2.rs
```

Two locations should match: `XSoftmax::apply` and `disentangled_attention_bias`. If candle 0.9+ ships these casts upstream, drop the `[patch.crates-io]` entry in the workspace `Cargo.toml`.

## Build & test

```bash
# macOS
cargo build -p sudachi-kwja
cargo test  -p sudachi-kwja

# Linux production
cargo build -p sudachi-kwja --no-default-features --features cuda

# Full equivalence (needs checkpoints + fixtures)
cargo test -p sudachi-kwja -- --include-ignored
```

## Sapling, not git

```bash
sl status
sl commit -m "feat(kwja): ..."
sl push
```

The `.sl/` directory is shared with sapling shares (`~/code/shares/jisho/kwja-rs/`).

## License

`MIT OR Apache-2.0` — matches KWJA upstream.
