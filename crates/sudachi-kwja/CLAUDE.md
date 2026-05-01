# sudachi-kwja

Pure-Rust port of [KWJA](https://github.com/ku-nlp/kwja) v2.4 inference. DeBERTa-v2 **base** backbone. candle 0.8 runtime.

## What this crate is for

A drop-in replacement for KWJA's Python inference path. Same checkpoints (`.ckpt → .safetensors`), same tokenizers (HF artifacts), same argmax outputs — but runs in a single long-lived Rust process with no Python interpreter, no `transformers`/`pytorch_lightning` import cost, and no per-request subprocess fork.

The crate's design assumption: jisho already has a canonical morphological analyzer (Sudachi UniDic). Re-tokenizing in KWJA is wasteful. Instead we feed KWJA's word module pre-tokenized input and consume only the **structural** heads it provides on top of the morphemes Sudachi already produced.

## Public API surface

Three entry points on `Pipeline`:

```rust
Pipeline::load(checkpoint_dir)            -> Result<Pipeline>
Pipeline::load_with_typo(checkpoint_dir)  -> Result<Pipeline>   // opt-in
Pipeline::load_with_device(dir, device)   -> Result<Pipeline>   // override

pipeline.parse(texts)                     -> Result<Vec<ParseItem>>          // full path
pipeline.parse_morphemes(sentences)       -> Result<Vec<ParseItem>>          // hot path
pipeline.correct_typos(texts, threshold)  -> Result<Vec<(String, bool)>>     // opt-in
```

`ParseItem` is `Tree(Document) | Error { kind, message }`. Per-input failures never abort the batch.

## Where the heads live

```
backbone (DeBERTa-v2 base, fp16 on CUDA)
   │
   ├── reading_tagger              SequentialMlpHead       subword-level reading
   ├── pos_tagger                  SequentialMlpHead       14 POS labels
   ├── subpos_tagger               SequentialMlpHead       35 sub-POS
   ├── conjtype_tagger             SequentialMlpHead       33 conj types
   ├── conjform_tagger             SequentialMlpHead       81 conj forms
   ├── ne_tagger                   SequentialMlpHead       17 BIO NE tags
   ├── dependency_parser           BiaffineDependencyHead  (T, T, 1) parent scores
   ├── dependency_type_parser      SequentialMlpHead       D/P/A/I edge types
   ├── word_feature_tagger         LoRASequenceMultiLabelingHead   per-word multi-label
   ├── base_phrase_feature_tagger  LoRASequenceMultiLabelingHead   per-word multi-label
   ├── cohesion_analyzer           LoRARelationWiseWordSelectionHead  (T, T, R) PAS+bridge+coref
   └── discourse_relation_analyzer WordSelectionHead       (T, T, D) cross-sentence
```

12 heads on one trunk. Three are pairwise scorers (dependency, cohesion, discourse) and produce `(T, T, *)` tensors; the rest are token classifiers producing `(T, num_labels)`.

## Hard rules

1. **Do not re-tokenize Japanese here.** This crate is the KWJA *inference* port; tokenization is owned by `sudachi-optimizer` (the canonical Sudachi gateway). When in doubt, push the Sudachi morphemes through `parse_morphemes` and let KWJA's word module run on the pre-tokenized stream.
2. **Argmax-identical equivalence with KWJA-Python is a contract, not a goal.** The tests in `tests/equivalence/` compare emitted JSON byte-for-byte against `scripts/gen_fixtures.py` output. If you change a decode path, regenerate fixtures and check they remain byte-equal — or update both ends with a written justification.
3. **Determinism.** Use `Vec<KeyValue>` (tuple struct) for `features` and `semantics`, never `HashMap`. KWJA-Python emits ordered list-of-objects and the production JSONB store matches that shape.
4. **Don't add HashMap to `Document`-tree types** for the same reason.
5. **fp16 on CUDA is the production path.** The vendored candle-transformers patches in `~/code/shares/jisho/kwja-rs/vendor/candle/` are required. Don't drop them until upstreamed.
6. **Length-bucketed batching belongs here, not in callers.** `pipeline.rs` sorts and re-groups; `jisho-parse` is a thin shell. Callers tune via `BucketingConfig` + `parse_morphemes_with_config`, never by re-implementing bucketing in their own layer.
7. **The crate is `MIT OR Apache-2.0`** to match KWJA upstream.

## File map

```
Cargo.toml                package = "sudachi-kwja"; features default = ["metal"]
README.md                 Public docs (audience: external users + jisho contributors)
CLAUDE.md                 This file (audience: AI working on the crate)
AGENTS.md                 Hard rules + common-task recipes
CHECKPOINTS.md            sha256s of converted safetensors

src/
├── lib.rs                Public re-exports
├── checkpoint.rs         safetensors loader, Device-aware
├── constants.rs          HIDDEN_SIZE = 768, IGNORE_INDEX, label-list bindings
├── crf.rs                Linear-chain CRF (decode helper)
├── error.rs              Error enum
├── pipeline.rs           ★ Top-level Pipeline + parse + parse_morphemes
│                         + length-bucketed batching + cohesion mask v3
│                         + cross-sentence discourse decode
│                         + sudachi_to_kwja_pos mapping
├── document/
│   ├── mod.rs            Re-exports
│   └── tree.rs           Document, Sentence, Phrase, BasePhrase, Morpheme,
│                         KeyValue, Relation, DiscourseRelation, ParseItem
├── tokenizer/
│   ├── mod.rs
│   ├── char_.rs          BERT-style char vocab loader
│   ├── deberta.rs        HF subword tokenizer + pretokenized variant
│   └── typo.rs           Typo-module char tokenizer + extended_vocab
└── model/
    ├── mod.rs
    ├── deberta.rs        DebertaBackbone (candle-transformers wrapper, fp16-aware)
    ├── pool.rs           pool_subwords (subword → word reduction)
    ├── heads.rs          SequentialMlpHead, WordSelectionHead,
    │                     BiaffineDependencyHead, LoRADelta,
    │                     LoRASequenceMultiLabelingHead,
    │                     LoRARelationWiseWordSelectionHead
    ├── char_.rs          CharModel (sent_segmentation only)
    ├── word.rs           WordModel + WordLogits (12-head output)
    └── typo.rs           TypoModel + TypoLogits (kdr + ins)

scripts/
├── convert_checkpoints.py   Lightning .ckpt → safetensors
├── gen_fixtures.py          Emit per-head logit fixtures from KWJA-Python
└── pyproject.toml

resources/
└── labels.json              All label tables (pos_tags, subpos_tags, ne_tags,
                             conjtype_tags, conjform_tags, dependency_types,
                             cohesion_relations, discourse_relations,
                             word_features, base_phrase_features,
                             sent_segmentation_tags, word_segmentation_tags,
                             word_norm_op_tags, ignore_index)

examples/
├── dump_tensor_names.rs     Print all loaded tensor names from a checkpoint
└── test_pos_mapping.rs      Sanity-check sudachi_to_kwja_pos

tests/
└── equivalence/             argmax-identical fixtures (against KWJA-Python v2.4)
```

## Two paths through the pipeline

### Full path: `parse(texts: &[&str])`

```
text  ──(CharTokenizer)──► char model ──(sent_segmentation argmax)──► sentence boundaries
                                                                           │
text  ──(DebertaTokenizer)──► word model.forward ──► WordLogits ──► decode_element_from_logits
                                                                           │
                                                                           ▼
                                                                       Document
```

Used when the caller has raw text and wants KWJA to handle everything.

### Hot path: `parse_morphemes(sentences: &[Vec<SudachiMorpheme>])`

```
Sudachi morphemes ──► forward_pretokenized_batch ──► WordLogits per chunk
                                                          │
                                                          ▼
                                       decode_element_from_logits
                                       (Sudachi data wins for surface/reading/
                                        lemma/conjtype/conjform; KWJA wins for
                                        dependency/dep_type/features/cohesion/discourse)
                                                          │
                                                          ▼
                                                       Document
```

This is what `jisho-parse` calls. The contract is: caller is responsible for tokenization. KWJA's word module is forced to honor the supplied word boundaries via subword-to-word pooling at the pre-supplied positions.

## DeBERTa-v2 base backbone

Fixed in `src/constants.rs`:

```rust
pub const HIDDEN_SIZE: usize = 768;        // base = 768; large would be 1024
pub const NUM_HIDDEN_LAYERS: usize = 12;   // base = 12; large = 24
```

To switch to KWJA-large, change these constants and replace the safetensors. Don't ship a runtime toggle — the head shapes inferred from constants are baked into the LoRA delta initializers.

## Performance notes

- **fp16 on CUDA**: ~2× speedup over fp32 with no measurable argmax change. Required for production throughput targets.
- **LoRA delta fused matmul**: heads.rs `LoRADelta::forward` does `(L, H, R) × (L, R, H) → (L, H, H)` as one CUDA kernel instead of L. Significant for `cohesion_analyzer` where L = num_relations.
- **Length-bucketed batching**: 4 buckets when `n_chunks ≥ 50` (default), else 1. Tunable via `BucketingConfig` + `parse_morphemes_with_config`. Threshold of 50 is data-driven — below that, single-bucket beats 4-bucket because launch overhead exceeds padding savings.
- **kwja_relative_pos**: candle's CUDA backend lacks `uabs_i64`, so we build relative position bias on CPU and transfer to target device. One-time cost per forward; negligible against attention compute.
- **DataLoader / worker pools**: not implemented. Single-threaded pipeline; concurrency is achieved by callers (jisho-parse spawns one Pipeline per process; multi-process scaling is via container replicas).

## Testing

```bash
cargo test -p sudachi-kwja                    # fast unit tests, no checkpoint
cargo test -p sudachi-kwja -- --include-ignored   # full equivalence vs KWJA-Python fixtures
```

The ignored tests need:
- `~/.local/share/jisho/checkpoints/{char,word}.safetensors`
- HF tokenizer artifacts at the same path
- A baseline fixture set in `tests/equivalence/fixtures/` (regenerate via `scripts/gen_fixtures.py`)

## Context outside this crate

- The downstream gRPC service is `jisho-parse` in the jisho monorepo (`~/code/jisho/services/jisho-parse/`). It depends on this crate via relative path `../../../sudachi/crates/sudachi-kwja`.
- The Sudachi tokenizer side lives in `sudachi-optimizer` (workspace sibling).
- Shared types between the gRPC service and this crate (`SudachiMorpheme`, `ParseItem`, `Document` tree) are mirrored in `proto/parse.proto` in the jisho monorepo.

## Sapling, not git

This whole monorepo is a Sapling repo — use `sl status` / `sl commit` / `sl push`. The `.sl/` graph is shared with any sapling shares (`~/code/shares/jisho/kwja-rs/` is a working share that points at this repo's `.sl/` for the jisho-side files but ships its own working dir).
