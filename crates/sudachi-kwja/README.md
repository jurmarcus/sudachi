# sudachi-kwja

**Pure-Rust port of [KWJA](https://github.com/ku-nlp/kwja) inference, pre-tokenized via Sudachi.**

A drop-in replacement for KWJA's Python inference path that produces argmax-identical output for the modules jisho consumes — but runs as a single long-lived Rust process with no Python interpreter in the hot path. Backbone and tokenizers are loaded from the same HuggingFace artifacts; only the runtime is replaced.

- **Upstream**: KWJA v2.4 (`ku-nlp/kwja`)
- **Backbone**: DeBERTa-v2 **base** Japanese
  - char module → `ku-nlp/deberta-v2-base-japanese-char-wwm` (~423 MB, char-level WWM)
  - word module → `ku-nlp/deberta-v2-base-japanese` (~532 MB, subword)
  - typo module → `ku-nlp/deberta-v2-base-japanese-char-wwm` (~330 MB, char-level)
- **Inference**: [candle](https://github.com/huggingface/candle) 0.8 with vendored fp16 patches for DeBERTa attention
- **Devices**: `cuda` (default in production), `metal` (Apple Silicon), `cpu` (fallback)

> **Note on backbone size.** KWJA upstream ships `base` and `large` variants. We use `base` everywhere — the production sentences/sec target on a single 4090 doesn't justify `large`'s 3-4× cost, and the argmax accuracy delta on Japanese text is small for the heads we consume. To switch, replace the `.ckpt` files with the `large` ones, regenerate `safetensors`, and bump `HIDDEN_SIZE` in `src/constants.rs` from 768 to 1024.

---

## What KWJA is

KWJA (京都・早稲田 Japanese Analyzer) is a multi-task Japanese NLP toolkit from Kyoto University and Waseda University. It's organized as three sequential modules running on three separate DeBERTa-v2 backbones:

```
input text
    │
    ▼
┌────────────────┐
│  typo module   │  Character-level edit prediction. Optional.
│  (char-level)  │  Outputs corrected text or echoes input.
└────────┬───────┘
         │ corrected text
         ▼
┌────────────────┐
│  char module   │  Sentence segmentation + word segmentation +
│  (char-level)  │  word normalization. KWJA's full output uses all
└────────┬───────┘  three heads; jisho uses only sentence-segmentation.
         │ sentence boundaries
         ▼
┌────────────────────────────────────────────────────┐
│            word module (subword)                   │
│  ─────────────────────────────────────────────     │
│  pos · subpos · conjtype · conjform · reading      │
│  ne (named entity) · dependency parse              │
│  dep_type · word_features · bp_features            │
│  cohesion (PAS / bridging / coref)                 │
│  discourse (cross-sentence)                        │
└────────────────────────────────────────────────────┘
```

The word module hosts **12 task heads** sharing the same DeBERTa-v2 trunk — most are sequence taggers, two are biaffine pairwise scorers (dependency, discourse), one is a relation-wise pairwise scorer (cohesion), and the multi-label heads (`word_features`, `bp_features`, `cohesion`) use **LoRA deltas** on top of frozen MLP heads.

---

## What this crate provides

A single `Pipeline` struct that loads char + word (and optionally typo) checkpoints once and offers two entry points:

### `Pipeline::parse(texts)` — full KWJA-style path

Mirrors the upstream Python `kwja --tasks word` flow: char model splits the text into sentences, the word module then forwards each sentence and the decoder reconstructs the document tree.

```rust
use sudachi_kwja::Pipeline;
use std::path::Path;

let pipeline = Pipeline::load(Path::new("/checkpoints"))?;
let docs = pipeline.parse(&["今日は晴れだ。明日は雨だろう。"])?;

for item in docs {
    match item {
        sudachi_kwja::ParseItem::Tree(doc) => {
            for sent in &doc.sentences {
                println!("{}", sent.text);
                for bp in &sent.base_phrases {
                    println!("  BP {}: {} → head {}", bp.id, bp.surface, bp.head);
                }
            }
            for d in &doc.discourse_relations {
                println!("discourse: s{}/bp{} → s{}/bp{}: {}",
                    d.from_sentence, d.from_base_phrase,
                    d.to_sentence, d.to_base_phrase, d.r#type);
            }
        }
        sudachi_kwja::ParseItem::Error { kind, message } => {
            eprintln!("error {}: {}", kind, message);
        }
    }
}
```

### `Pipeline::parse_morphemes(sentences)` — Sudachi pre-tokenized hot path

The path jisho actually uses. Caller has already tokenized with Sudachi (the canonical UniDic tokenizer for the project); we skip char and run only word. KWJA's word module is fed pre-tokenized input via `forward_pretokenized`, which forces word boundaries to match the Sudachi morpheme list.

```rust
let sentences = vec![vec![
    SudachiMorpheme {
        surface: "今日".into(), reading: "きょう".into(), lemma: "今日".into(),
        pos: vec!["名詞".into(), "普通名詞".into(), /* ... */],
        conjtype: "*".into(), conjform: "*".into(), /* ... */
    },
    // ...
]];
let docs = pipeline.parse_morphemes(&sentences)?;
```

**Division of labor in this path:**

| Field | Source | Reason |
|---|---|---|
| `surface` | Sudachi | jisho is the canonical Sudachi tokenizer; never re-tokenize. |
| `reading` | Sudachi | Sudachi's UniDic readings beat KWJA's reading head on jisho corpora. |
| `lemma` / `dictionary_form` | Sudachi | Same. |
| `conjtype` / `conjform` | Sudachi | Same. |
| `pos` / `subpos` | Sudachi (mapped) | Sudachi UniDic tags → KWJA JUMAN tags via `sudachi_to_kwja_pos`. |
| `ne` (named entity) | KWJA | Sudachi has no NE; KWJA does. |
| `head` (dep parent) | KWJA | KWJA's biaffine dependency parser. |
| `dep_type` | KWJA | "D" / "P" / "A" / "I" parallel/apposition/dep edge types. |
| `features` (Morpheme) | KWJA | Multi-label sigmoid > 0.5: `基本句-主辞`, `用言表記末尾`, etc. |
| `features` (BasePhrase) | KWJA | Multi-label sigmoid > 0.5: BP-level tags. |
| `relations` (PAS, bridging, coref) | KWJA | Cohesion analyzer; argmax over target words per relation type. |
| `discourse_relations` (cross-sentence) | KWJA | Cross-sentence discourse, predicted on the whole input element. |

KWJA-Python's POS/lemma/reading output for a Sudachi-tokenized text is redundant with Sudachi's own — this crate consumes the **structural** heads (dependency, BP tree, cohesion, discourse, NE, features) and lets Sudachi own the morpheme-level fields.

### `Pipeline::correct_typos(texts, threshold)` — opt-in typo correction

Loaded only when explicitly requested (the typo checkpoint is ~330 MB and most callers don't need it). Outputs `(corrected_text, changed: bool)` per input. KWJA's typo module is character-level and emits two parallel taggers — `kdr` (Keep/Delete/Replace) and `ins` (Insert before). The decoder applies edit ops with a per-position softmax threshold below which no edit is taken.

```rust
let pipeline = Pipeline::load_with_typo(Path::new("/checkpoints"))?;
let corrected = pipeline.correct_typos(&["わたしわ学生です。"], 0.9)?;
// → [("わたしは学生です。", true)]
```

---

## Architecture

### Module layout

```
crates/sudachi-kwja/src/
├── lib.rs                 Public re-exports (Pipeline, Document, Sentence, ...)
├── checkpoint.rs          safetensors loader + Device-aware Checkpoint type
├── constants.rs           HIDDEN_SIZE, IGNORE_INDEX, label list bindings
├── crf.rs                 Linear-chain CRF (used by char module decode in upstream KWJA)
├── error.rs               Error enum (anyhow-shaped, source-preserving)
├── pipeline.rs            ★ Top-level orchestrator: load + parse + parse_morphemes
│
├── document/
│   ├── mod.rs             Re-exports
│   └── tree.rs            Document → Sentence → Phrase → BasePhrase → Morpheme tree
│                          + KeyValue (semantics/features) + Relation + DiscourseRelation
│                          + ParseItem enum (Tree | Error)
│
├── tokenizer/             HF tokenizer wrappers
│   ├── mod.rs
│   ├── char_.rs           Char-level vocab.txt loader (BERT-style)
│   ├── deberta.rs         Subword tokenizer.json loader; pretokenized variant
│   └── typo.rs            Char-level loader for typo module + extended_vocab
│
└── model/
    ├── mod.rs
    ├── deberta.rs         DeBERTaBackbone — wraps candle-transformers' DebertaV2
    │                      with fp16-on-CUDA defaults + relative-pos workaround
    ├── pool.rs            pool_subwords: subword → word reduction (mean/first)
    ├── heads.rs           SequentialMlpHead, WordSelectionHead, LoRADelta,
    │                      LoRASequenceMultiLabelingHead, LoRARelationWiseWordSelectionHead
    ├── char_.rs           CharModel (sent_segmentation_tagger only)
    ├── word.rs            WordModel: backbone + 12 heads + WordLogits
    └── typo.rs            TypoModel: encoder + kdr_tagger + ins_tagger
```

### The 12 word-module heads

```rust
pub struct WordModel {
    tokenizer: DebertaTokenizer,           // HF subword
    backbone: DebertaBackbone,             // DeBERTa-v2 base, fp16 on CUDA
    reading_tagger:           SequentialMlpHead,            // → reading classes (subword-level)
    pos_tagger:               SequentialMlpHead,            // → 14 POS tags
    subpos_tagger:            SequentialMlpHead,            // → 35 sub-POS tags
    conjtype_tagger:          SequentialMlpHead,            // → 33 conj types
    conjform_tagger:          SequentialMlpHead,            // → 81 conj forms
    ne_tagger:                SequentialMlpHead,            // → 17 NE BIO tags
    dependency_parser:        BiaffineDependencyHead,       // (T, T, 1) pairwise scores
    dependency_type_parser:   SequentialMlpHead,            // → dep_type per word
    word_feature_tagger:      LoRASequenceMultiLabelingHead,// per-word multi-label
    base_phrase_feature_tagger: LoRASequenceMultiLabelingHead, // per-word multi-label (BP-tagged)
    cohesion_analyzer:        LoRARelationWiseWordSelectionHead, // (T, T, R) PAS/bridge/coref
    discourse_relation_analyzer: WordSelectionHead,         // (T, T, D) cross-sentence
}
```

`WordLogits` carries the full output tensor set; the decoder in `pipeline.rs` reads each tensor and assembles the `Document` tree.

### Head taxonomy

| Head type | Used by | What it does |
|---|---|---|
| `SequentialMlpHead` | reading, pos, subpos, conjtype, conjform, ne, dep_type, sent_segmentation, kdr/ins | LayerNorm → dropout → Linear(hidden, hidden) → tanh → Linear(hidden, num_labels). Standard token classifier. |
| `BiaffineDependencyHead` | dependency_parser | Two MLPs (head, child) → biaffine product → (T, T, 1) pairwise scores. Argmax over parent axis = dependency parent. |
| `WordSelectionHead` | discourse_relation_analyzer | Like biaffine but with relation-typed output: (T, T, num_relations). |
| `LoRADelta` | (used by LoRA heads below) | Low-rank delta: `x → (x · A) · B → permute`. Fused into one `(L, H, R) × (L, R, H)` matmul for one CUDA call instead of L. |
| `LoRASequenceMultiLabelingHead` | word_features, bp_features | Frozen MLP head + relation-wise LoRA delta + sigmoid. Multi-label per word. |
| `LoRARelationWiseWordSelectionHead` | cohesion_analyzer | Like WordSelectionHead but each relation type has its own LoRA delta on the source-side projection. |

### Length-bucketed batching

`Pipeline::parse_morphemes` sorts inputs by length descending and partitions into 4 buckets when `n_chunks >= 50` (default; tunable). Below the threshold, bucketing is skipped entirely — bench data showed the 4-bucket variant **loses** to single-bucket below ~50 chunks (240 sps vs 404 sps measured at spr=15), because launch overhead exceeds padding savings on short batches. Above the threshold, each bucket pads to its own `T_max`, typically reducing wasted compute by 50–70% on mixed-length batches like a YouTube subtitle file.

```
inputs:        [3, 17, 5, 12, 1, 22, 8, 6, ...]  (lengths)
sorted desc:   [22, 17, 12, 8, 6, 5, 3, 1, ...]
4 buckets:     [22, 17, ...] [12, 8, ...] [6, 5, ...] [3, 1, ...]
forward calls: 4 × (one bucket each, padded to its own max)
output:        regrouped to original order via index tracking
```

The bucketing infra lives entirely in `pipeline.rs` — `jisho-parse` (the gRPC service) is a thin shell. The threshold and bucket count are both configurable via `BucketingConfig`:

```rust
use sudachi_kwja::{Pipeline, BucketingConfig};

let cfg = BucketingConfig {
    num_buckets: None,                  // None = legacy heuristic (4 if above threshold)
    min_chunks_for_bucketing: 50,       // raise/lower threshold; 0 = always bucket
};
let docs = pipeline.parse_morphemes_with_config(&sentences, &cfg)?;

// Force exactly N buckets regardless of input size:
let cfg = BucketingConfig { num_buckets: Some(1), ..Default::default() }; // disable
```

`Pipeline::parse_morphemes(&sentences)` uses `BucketingConfig::default()` (threshold 50, 4-bucket heuristic).

### Cohesion mask (anaphoric directionality)

The cohesion analyzer's PAS / bridging / coreference output is post-processed before argmax to enforce structural constraints: **anaphoric relations** (ノ, =) point only **backward** in word order (a referent must precede its anaphor). The mask is applied in-place on `cohesion_logits` before the per-source-word, per-relation argmax.

### Cross-sentence discourse

`discourse_logits` has shape `(1, T_words, T_words, num_discourse_relations)` over the **whole input element** (all sentences forwarded together, not per-sentence). The decoder iterates predicate base phrases across decoded sentences and looks up the discourse score at `(head_word_i, head_word_j)` for cross-sentence pairs. Single-sentence input → no cross-sentence relations emitted.

### Deterministic JSON

`Document` and friends use `Vec<KeyValue>` (a `(String, String)` tuple struct) instead of a `HashMap` for `features` and `semantics` so JSON serialization order is preserved. KWJA-Python emits these as `[{"key": ..., "value": ...}, ...]` and the production JSONB stored in `passage_parse_tree.tree` matches that shape — byte equality with KWJA-Python is a regression-test contract.

### fp16 inference (CUDA)

Default dtype on CUDA is **F16** to match KWJA-Python's emission path. This required vendored patches to `candle-transformers`' `debertav2.rs`:

- `XSoftmax::apply`: scalar literals (`1.0_f32`, `f32::MIN`, `0_f32`) cast to `input.dtype()` before tensor ops
- `disentangled_attention_bias`: `score = Tensor::new(&[0_f32], device).to_dtype(query_layer.dtype())`

Without these the model errored with "dtype mismatch in add: lhs F32, rhs F16" on CUDA. Patches live in `~/code/shares/jisho/kwja-rs/vendor/candle/`; eventual upstreaming will let us drop the vendored fork.

---

## Checkpoint setup

KWJA distributes Lightning `.ckpt` files via HuggingFace; they're cached in the running KWJA-Python container at `/root/.cache/kwja/v2.4/`. Run `scripts/convert_checkpoints.py` (Python, uses `torch` and `safetensors`) once to convert:

```bash
# from inside this crate
docker cp jisho-parse-1:/root/.cache/kwja/v2.4/. ~/.cache/kwja-export/

cd scripts
uv run python convert_checkpoints.py \
  --in ~/.cache/kwja-export/word_deberta-v2-base.ckpt \
  --out ~/.local/share/jisho/checkpoints/word.safetensors

uv run python convert_checkpoints.py \
  --in ~/.cache/kwja-export/char_deberta-v2-base-wwm.ckpt \
  --out ~/.local/share/jisho/checkpoints/char.safetensors
```

Final layout consumed by `Pipeline::load`:

```
/checkpoints/
├── char.safetensors                  423 MB
├── word.safetensors                  532 MB
├── typo.safetensors                  330 MB        (optional)
├── kwja-tokenizer/
│   └── tokenizer.json                              from HF deberta-v2-base-japanese
├── kwja-char/
│   ├── vocab.txt                                   from HF deberta-v2-base-japanese-char-wwm
│   └── tokenizer_config.json
└── kwja-typo/
    ├── vocab.txt
    └── multi_char_vocab.txt                        extended insertion vocab
```

See `CHECKPOINTS.md` for current sha256s. The hashes are checked by `tests/equivalence/` to detect drift when the source `.ckpt` ever changes.

---

## Build

```bash
# macOS — Metal backend
cargo build -p sudachi-kwja

# Linux — CUDA backend (production)
cargo build -p sudachi-kwja --no-default-features --features cuda

# CPU-only (portable testing, slow)
cargo build -p sudachi-kwja --no-default-features
```

Features:

| Feature | Default? | What it enables |
|---|---|---|
| `metal` | yes | candle Metal kernels (Apple Silicon) |
| `cuda`  | no  | candle CUDA kernels (Linux + NVIDIA) |
| (none)  | —   | CPU-only fallback via candle |

The `cuda` feature is mutually exclusive with `metal` for compilation.

---

## Status (2026-05)

| Module / Head | Status | Notes |
|---|---|---|
| char: sent_segmentation | shipped | Used when caller skips Sudachi pre-tokenization. |
| char: word_segmentation | not consumed | Sudachi handles word boundaries in the hot path. |
| char: word_norm_op | not consumed | Reserved for future use. |
| word: pos / subpos | shipped | Sudachi-mapped values win in the hot path. |
| word: conjtype / conjform | shipped | Same. |
| word: reading | shipped | Sudachi UniDic readings preferred. |
| word: ne | shipped | KWJA-only signal. |
| word: dependency | shipped | Biaffine head; argmax over parent axis. |
| word: dependency_type | shipped | "D" / "P" / "A" / "I" edge types. |
| word: word_features | shipped | Multi-label sigmoid > 0.5. |
| word: base_phrase_features | shipped | Same. |
| word: cohesion (PAS/bridge/coref) | shipped | Anaphoric backward-only mask v3. |
| word: discourse (cross-sentence) | shipped | Whole-element forward + cross-sentence decode. |
| typo: kdr / ins | shipped (opt-in) | Loaded only when caller requests. |

**Equivalence:** argmax-identical with KWJA-Python v2.4 on the heads listed above, validated via `tests/equivalence/` against fixtures generated by `scripts/gen_fixtures.py`.

---

## License

`MIT OR Apache-2.0` — matches KWJA upstream's licensing posture. Model weights remain under their original KWJA / HuggingFace license terms.
