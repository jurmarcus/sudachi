# Comprehension pipeline

> The three-layer architecture: what each layer produces, who consumes its output, where new code belongs, and how the 9K Anki passages serve as the golden regression corpus.

## How this doc was written

Every "purpose" claim is derived from **what consumers actually do today** in `~/CODE/jisho/`, not from theoretical architecture. Each layer's purpose is the intersection of (a) what it produces and (b) what downstream code reads. Claims are cited with file:line references so they can be verified and updated when the code changes. If you change consumption (e.g. start using a KWJA head that's currently ignored), update this doc.

## Mission

This monorepo exists to support **second-language Japanese acquisition** via the `jisho` learner-oriented pipeline (vocab lookup, grammar matching, comprehension scoring, FSRS spaced repetition, comprehensible-input content selection). Every architectural decision should serve one question:

> **Does this output help a learner read Japanese text and convert input into acquired vocabulary and grammar?**

Linguistic correctness, search-engine ergonomics, and parser elegance are means, not ends. They matter only insofar as they serve comprehension. A morphologically correct parse that splits set grammar phrases into their etymological components but produces output the learner cannot look up has failed the mission.

## The three-layer architecture

Every analyzed passage flows through three layers. Each layer has a single responsibility, produces a persisted artifact, and has explicit input/output contracts with its neighbours.

```
                   Text input
                       │
                       ▼
   ┌────────────────────────────────────────────────────────┐
   │ (1) Sudachi raw  ──►  sudachi-optimizer                │
   │                       (28 mechanical rules)            │
   │                                  │                     │
   │                                  ▼                     │
   │                       clean morpheme stream  ◄── (1) PERSISTED
   └─────────────────────────────────┬──────────────────────┘
                                     │
                                     │  (1) feeds clean morphemes into KWJA
                                     ▼
   ┌────────────────────────────────────────────────────────┐
   │ (2) KWJA inference on (1)                              │
   │           │                                            │
   │           ▼                                            │
   │     KWJA raw tree  ──►  sudachi-kwja-optimizer  (NEW)  │
   │                         (mechanical KWJA cleanup)      │
   │                                  │                     │
   │                                  ▼                     │
   │                       clean KWJA tree  ◄────── (2) PERSISTED
   └─────────────────────────────────┬──────────────────────┘
                                     │
                                     │  (1) and (2) both feed (3)
                                     ▼
   ┌────────────────────────────────────────────────────────┐
   │ (3) jisho-core comprehension layer                     │
   │     - matchers (vocab, grammar, expression, propnoun)  │
   │     - sudachi-morphology consumers (deconjugation)     │
   │     - hybrid rules (reading refinement, sense bias,    │
   │       idiom validation, NE augmentation, ...)          │
   │     - comprehension scoring                            │
   │                                  │                     │
   │                                  ▼                     │
   │                       comprehension output  ◄── (3) PERSISTED
   │                       (the consumer-facing artifact)   │
   └────────────────────────────────────────────────────────┘
                                     │
                                     ▼
                   Anki export, GraphQL responses,
                   comprehension scoring, golden corpus
```

The single discriminator that decides where new code goes: **does it require jisho-specific data** (vocab table, grammar table, learner acquisition state, comprehension scoring context)?

- **No** → it goes in (1) or (2). Both are mechanical: generic Japanese tooling that any consumer could use, including non-jisho consumers.
- **Yes** → it goes in (3). All jisho-specific opinion lives in one place.

## Layer (1) — sudachi-optimizer

**Crate**: `crates/sudachi-optimizer/`
**Job**: take raw Sudachi/UniDic output and fix its morpheme-level mistakes so each emitted token is a clean lexical unit.

**Inputs**: text + Sudachi dictionary (system + optional user dicts)
**Outputs**: `Vec<Morpheme>` — surface, lemma (`dictionary_form` / `normalized_form`), reading, POS array, char range, `applied_rules` audit trail
**Persisted as**: `passage_spans` morpheme stream (the underlying token sequence — historically conflated with matched spans; the cleanup separates them)

**Why it exists**: UniDic was tuned for newspaper-text analysis and search ranking. It produces over-merged compounds (`足蹴` glued to a passive auxiliary, leaving `られた` orphaned), under-merged conjugations (`食べ + て` instead of `食べて`), false-positive proper nouns (`いわね` parsed as a person name in `まずいわね`), mis-classified colloquial forms (`じゃない` broken into `じゃ + なーい` when emphasised), and a long tail of similar errors that are unambiguously *wrong* for vocab lookup. The 28-rule pipeline corrects these.

**No jisho dependency**. The optimizer is generic Japanese morphology cleanup. Every search-side consumer (`sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`, paradedb) depends on it. None of them depend on KWJA or jisho-core.

**Phase organisation**: `Split → Repair → Combine → Cleanup → Disambiguation`, interleaved. Each rule lives in its own file; tests live alongside the rule. See [`crates/sudachi-optimizer/CLAUDE.md`](crates/sudachi-optimizer/CLAUDE.md) for internals.

## Layer (2) — sudachi-kwja-optimizer

**Crate**: `crates/sudachi-kwja-optimizer/` — exists; one rule today (NE filter)
**Job**: take raw KWJA output (computed on (1)'s clean morphemes) and fix its mechanical mistakes so each emitted tree is structurally clean.

**Inputs**: KWJA's `Document` tree
**Outputs**: cleaned `Document` tree (same shape; corrected attributes)
**Persisted as**: `passage_spans.tree` (today's column repurposed for cleaned output)

**Why it exists**: KWJA running on already-cleaned Sudachi morphemes is mostly fine, but it has its own known failure modes — over-tagged NE spans, malformed dependency arcs, inconsistent feature label spellings, low-confidence multi-label noise. Today these aren't handled anywhere; downstream consumers either trust the tree or work around drift case-by-case (e.g. `kwja_reading_refinement.rs` works around reading drift, but that fix lives in jisho-core because it requires vocab corroboration).

The (2) optimizer captures the **mechanical** cleanups that don't need jisho data — defensive structural validation, label normalisation, confidence thresholding. Anything KWJA correction that requires vocab/grammar/learner data is a (3) hybrid rule, not a (2) optimizer rule.

**No jisho dependency**. Same as (1): generic KWJA cleanup, reusable by any KWJA consumer.

### Current rules

| Rule | Phase | What it does |
|---|---|---|
| `filter/ne` | Filter | Drop spurious NE feature entries via type-aware surface heuristics: pure-hiragana proper-noun tags, single-kanji/pure-hiragana ARTIFACT tags, malformed values, unknown tags. Preserves DATE / TIME / MONEY / PERCENT (which can legitimately be hiragana-only). |

Other candidate rules — BP feature label normalisation, dependency arc validation, word-feature multi-label thresholding, PAS sanity checks — should be added only when concrete failure cases emerge in the regression corpus, not built speculatively.

## Layer (3) — jisho-core comprehension layer

**Location**: `~/CODE/jisho/packages/rs/jisho-core/`
**Job**: combine (1) and (2) into the consumer-facing comprehension artifact. All jisho-specific opinion lives here.

**Inputs**: clean morpheme stream from (1), clean KWJA tree from (2), vocab/grammar/proper-noun tries, learner acquisition state
**Outputs**: matched spans + per-span attributes (sense pick, reading refinement, register tag, idiom annotations, NE augmentations) + comprehension score
**Persisted as**: `passage_spans.spans` (today's column repurposed for the consumer-facing combined output)

**What's in (3)**:

- **Matchers** (`jisho-core/src/analysis/matchers/`) — vocab, grammar, expression, proper-noun. Walk a flat morpheme stream and produce span matches against the trie indices.
- **sudachi-morphology consumers** — the matchers use it for deconjugation (surface → candidate lemmas) so they can find vocab entries even when the word is conjugated. Morphology is a library, not a layer; its USAGE is concentrated here in (3).
- **Hybrid rules** — combine signals from (1) and (2) with vocab/grammar data:
  - `kwja_reading_refinement.rs` — override Sudachi readings with KWJA's contextual reading when (a) KWJA disagrees AND (b) the reading is corroborated by a vocab-table entry (`jisho-core/src/analysis/kwja_reading_refinement.rs`)
  - `sense_pick::register_for_span` — bias gloss selection by KWJA's BP `敬語` feature for honorific/humble register (`jisho-core/src/analysis/sense_pick.rs:98-120`)
- **Comprehension scoring** (`jisho-core/src/scoring/comprehension.rs`) — classify each matched span by acquisition tier (new/young/mature) and produce the comprehension score the learner sees.

**Has jisho dependency** by definition. (3) is the architectural home for everything that knows about vocab, grammar, learner state, and comprehension goals.

**Sudachi-morphology** sits alongside (3) as a library used by the matchers. It doesn't "live in" (3) per se — it's a separate crate (`crates/sudachi-morphology/`) usable standalone for conjugation tables, quiz generation, etc. But its primary consumer is (3)'s matcher layer, which is why it's most associated with comprehension work.

## How the layers harmoniously aggregate

Today's `analyze.rs` orchestration runs phases sequentially: tokenize, match, decorate with KWJA, refine readings, refine senses. Under the (1)/(2)/(3) model, the same orchestration is reframed:

```
Phase 1 — generate (1)
  sudachi-optimizer.tokenize() → clean morphemes  ── persist as (1)

Phase 2 — generate (2)
  KWJA on (1)'s morphemes → raw tree
  sudachi-kwja-optimizer → clean tree  ── persist as (2)

Phase 3 — generate (3)
  matchers on (1) + (2) → matched spans
  hybrid rules combining (1) + (2) → refinements
  comprehension scoring → score                  ── persist as (3)
```

Each phase generates a persisted artifact. Cache hits on any layer skip recomputation of that layer. Re-running just (3) (e.g. after adding a new hybrid rule) doesn't require re-tokenizing or re-running KWJA.

**Operational reality**: KWJA is treated as always-present infrastructure. The Rust port took boot time from seconds (Python GIL bottleneck) to ~50ms; both Sudachi+optimizer and KWJA outputs are persisted alongside the analyzed text; coverage is 100% on the ~8,900 Anki-card passages. The "best-effort" framing of the old Python era no longer applies — hybrid rules in (3) can rely on (2) being available without conditional fallback logic.

## The 9K Anki passages as golden corpus

The 9K passages stored as Anki cards are implicitly QA'd content — the user wouldn't keep studying with a passage whose analysis was wrong. That makes them a high-quality regression corpus for the entire pipeline, especially layer (3).

**The discipline**: snapshot today's (3) output for all 9K passages as `golden_v3.json` (or a fixture DB table keyed by passage hash). Every future change to (1), (2), or (3) is graded against the corpus:

```
rule change ──► run pipeline against 9K passages ──► diff new (3) vs golden (3)
                                                          │
                                                          ▼
                                          per-passage delta report:
                                          - regressions  → reject the change
                                          - improvements → update golden,
                                                          PR justifies each diff
```

This converts pipeline development from "I think this rule helps" into "this rule changed N spans across the corpus, here's exactly what changed, judgment per case." The same discipline `cargo test` provides for code, applied to morphological analysis on real-content distribution. No other Japanese morphological analyzer ships with a real-content golden corpus this size.

**Bootstrap**: snapshot `passage_spans.spans` for the 9K Anki rows today as the initial golden. The current rules already produce this output; locking it in as golden means future changes have a baseline to diff against.

## The layered decision rules

Adding new code? First decide **what signals it needs**, then **which layer**:

```
What signals does the new code need?
   │
   ├── Just morpheme-level Sudachi output
   │     │
   │     └── (1) sudachi-optimizer rule. Mechanical, no jisho data.
   │         28 examples today.
   │
   ├── Just KWJA tree
   │     │
   │     └── (2) sudachi-kwja-optimizer rule. Mechanical, no jisho data.
   │         Zero rules today; NE filtering is the obvious first one.
   │
   └── Both signals AND/OR jisho data (vocab, grammar, learner state)
         │
         └── (3) jisho-core comprehension layer.
             Matcher, hybrid rule, or scoring extension.
             All jisho-specific opinion lives here.

Then: which kind of change?
   │
   ├── Single dictionary-lookup unit appearing as one token
   │     (te-form + verb stem, number + counter, prefix + noun)
   │     │
   │     └── (1) combine rule
   │
   ├── Splitting a Sudachi over-merge that no other layer can fix
   │     │
   │     └── (1) split or repair rule
   │
   ├── Multi-token grammar pattern (なくてはならない, 〜ている, 〜て来る)
   │     │
   │     └── DO NOT merge in (1). Add the pattern to the GRAMMAR TABLE.
   │         The GrammarNgramMatcher in (3) walks a flat token stream
   │         and pattern-matches across tokens.
   │
   ├── Multi-token vocab expression / idiom (確かに, 手を抜く, 気がつく)
   │     │
   │     └── DO NOT merge in (1). Add the expression as a multi-token
   │         entry in the VOCAB TABLE. The ExpressionSpanMatcher in (3)
   │         detects it via vocab_common_prefix_search.
   │
   ├── Mechanical KWJA cleanup (NE noise, dep validation, label norm)
   │     │
   │     └── (2) sudachi-kwja-optimizer rule
   │
   ├── Hybrid correction (KWJA evidence corroborated by vocab data)
   │     │
   │     └── (3) hybrid rule. Today's templates: kwja_reading_refinement,
   │         sense_pick::register_for_span.
   │
   └── BM25 retrieval atoms for cross-passage search
         │
         └── (3) search_text encoder reads (2) directly.
```

### The most common architectural mistake

**Layer (1) doing layer (3)'s job by pre-merging multi-token patterns.**

Merging `〜ている`, `〜てしまう`, `〜て来る`, `〜による` etc. in the optimizer makes the constituent morphemes invisible to (a) the vocab matcher (which can no longer surface `来る` as a vocab item the learner is studying) and (b) the grammar matcher (which can no longer recognise the productive pattern across tokens). The merged token has no home — not a dictionary entry, not a grammar pattern.

The right choice for productive multi-token patterns: **leave them as separate tokens in (1); let the (3) matchers pick them up via the vocab/grammar tables**.

### When (1) SHOULD merge

The optimizer should merge when the merge produces a token that maps to **exactly one entry in either the vocab table or the grammar table**:

| Merge | Why (1) is the right home |
|---|---|
| `食べ + て → 食べて` | Conjugation. Learner needs to find `食べる` via this inflected form. |
| `食べ + ます + でし + た → 食べました` (lemma `食べる`) | Politeness/tense conjugation chain. One vocab unit. |
| `お + 寿司 → お寿司` | Honorific prefix is part of the vocab entry. |
| `三 + 本 → 三本` | Number + counter is one quantitative unit. |
| `先生 + 様 → 先生様` (when Sudachi over-split) | Bound suffix attached to a noun. |

Discriminator: **does the resulting token exist as exactly one entry in the vocab or grammar table?** If yes, (1) merge is correct. If the result is a productive pattern (`〜ている`), don't merge — leave separate, let (3)'s matcher handle it.

## Audit of the existing 28 sudachi-optimizer rules

Categorising every current (1)-layer rule by the decision rules above. Verdicts unchanged from prior audit; framing now uses (1)/(2)/(3) terminology.

### Split phase — KEEP all 5

Splits Sudachi over-merges. KWJA cannot un-merge what it's been handed. Splits MUST happen in (1).

| Rule | Decision |
|---|---|
| `split/compound_auxiliary_verbs` | KEEP |
| `split/proper_noun_with_particle` | KEEP |
| `split/tan_suffix` | KEEP |
| `split/tatte_particle` | KEEP |
| `split/tawake_noun` | KEEP |

### Repair phase — KEEP all 11; EXTEND vowel_elongation

Fixes Sudachi morpheme-level errors. None are bunsetsu-shaped or multi-token-pattern-shaped.

| Rule | Decision |
|---|---|
| `repair/colloquial_negative_nee` | KEEP |
| `repair/colloquial_ran_nai` | KEEP |
| `repair/compound_noun_suffix` | KEEP |
| `repair/fused_interjection_particle` | KEEP — ported from Jiten, unambiguous win |
| `repair/hasa_noun` | KEEP |
| `repair/honorific_lemma` | KEEP |
| `repair/n_tokenisation` | KEEP |
| `repair/orphaned_auxiliary` | KEEP |
| `repair/process_special_cases` | KEEP |
| `repair/tanka_to_ta_n_ka` | KEEP |
| `repair/vowel_elongation` | **EXTEND** — handle cross-word ー (`あなたー`, `じゃなーい`, `来るー`); strip ー when stripped form is a known UniDic word |

### Combine phase — KEEP most; SCOPE DOWN auxiliary + verb_dependant

This is where the discipline matters most.

| Rule | Decision |
|---|---|
| `combine/inflections` | KEEP — produces one vocab-entry-resolvable token |
| `combine/tte` | KEEP — te-form lemma resolution |
| `combine/conjunctive_particle` | KEEP — conjugation merge |
| `combine/auxiliary_verb_stem` | KEEP — `〜そう` is one vocab item ("looks like X") |
| `combine/prefixes` | KEEP — bound morpheme |
| `combine/suffix` | KEEP — bound morpheme |
| `combine/amounts` | KEEP — number + counter is one quantitative unit |
| `combine/to_naru` | KEEP — fixes a Sudachi over-split; result is one verb form |
| `combine/final_` | KEEP — conjugation + lemma |
| `combine/adverbial_particle` | KEEP — part of conjugation chains |
| `combine/particles` | **REVIEW per-pair** — pairs that produce a single grammar-table entry (e.g. `では`) might be better as grammar-table multi-token entries; pairs that are conjugation merges should stay |
| `combine/auxiliary` | **SCOPE DOWN** — keep merging genuine conjugation aux (た, ます, でした). **Stop merging productive aux constructions** (`〜ている`, `〜てしまう`, `〜てある`, `〜ておく`). Leave them separate so (3)'s GrammarNgramMatcher recognises them as grammar patterns and the vocab matcher surfaces the head verb independently. |
| `combine/verb_dependant` | **SCOPE DOWN** — stop merging productive aux verbs (`〜て来る`, `〜て行く`, `〜てみる`, `〜ていく`). Same reason. Keep merging genuine compound verbs that produce a single dictionary entry (`〜やすい`, `〜にくい`, `〜過ぎる`, `〜始める`, `〜終わる`). |

### Cleanup phase — KEEP both

| Rule | Decision |
|---|---|
| `cleanup/filter_misparse` | KEEP |
| `cleanup/reclassify_orphaned_suffixes` | KEEP |

### Disambiguation phase — KEEP

| Rule | Decision |
|---|---|
| `disambiguation/fix_reading_ambiguity` | KEEP — runs in (1) (before KWJA). The (3) hybrid rule `kwja_reading_refinement` is the second pass that uses KWJA's contextual reading where it improves on (1)'s choice. Complementary, not redundant. |

## Recommended new work

### New (1) rules (Sudachi morpheme errors)

The runnable harness for documented Jiten failure cases is at `crates/sudachi-optimizer/examples/jiten_regression.rs`.

1. **Extend `repair/vowel_elongation`** — handle ー glued across word boundaries (Jiten cases #04, #13).
2. **New `split/interrogative_counter`** — split `何 + counter` when Sudachi merges them as a rare surname (Jiten case #06). Generalises to `誰`, `どれ`, `どこ` + suffixes.
3. **New `split/passive_compound_noun`** — split contextual cases like `足蹴 + られた` (Jiten case #03). Gate carefully — `足蹴` is a real noun in `足蹴にする`.
4. **Scope down `combine/auxiliary` and `combine/verb_dependant`** — see audit above.

### Extend (2) sudachi-kwja-optimizer

The crate exists with one rule (NE filter). Add new rules as concrete failure cases emerge in the regression corpus, not speculatively. Candidates:

1. **BP feature label normalisation** (`normalize/`) — canonicalise `敬語=尊敬` vs `敬語=尊敬語` etc.
2. **Dependency arc validation** (`validate/`) — defensive cleanup of malformed dep arcs (cycles, orphans, multi-head BPs).
3. **Word-feature multi-label thresholding** (`filter/`) — drop low-confidence per-morpheme labels.

Unblocked downstream by the existing NE filter: **NE-augmented proper-noun hybrid rule in (3)** that surfaces ProperNounSpans from KWJA NE entries the proper_noun trie didn't catch.

### NOT new (1) rules — populate the vocab and grammar tables instead

These are tempting to add as optimizer rules but belong in the data layer where (3) consumers already exist:

- **Compound particles** — `によって`, `として`, `について`, `に関して`, `にとって`, `に対して`, `ながら`, `ばかり`, `くらい`, `ように`, `みたいに`, `ところ`, `わけ`, `はず`. **Add as multi-token grammar table entries.** The `GrammarNgramMatcher` walks the flat token stream and matches across tokens. (1) must keep `に + よって` separate.
- **Idioms / multi-word expressions** — `手を抜く`, `気がつく`, `腹が立つ`, `気を付ける`, `仕方がない`, `世話になる`, `気にする`. **Add as multi-token vocab table entries.** `ExpressionSpanMatcher` already detects these via `vocab_common_prefix_search`. The work is data, not architecture.
- **Set grammar phrases** — `〜なければならない`, `〜てはいけない`, `〜ことができる`, `〜ようとする`. Same — populate the grammar table.

### Candidate (3) hybrid rules

KWJA produces signals that are persisted in (2) but not yet consumed. Each is a candidate for a new (3) hybrid rule:

| KWJA signal in (2) | Candidate (3) hybrid rule | What it solves |
|---|---|---|
| **NE spans** (after (2) filtering) | Augment proper-noun detection: surface `ProperNounSpan` when KWJA NE labels a token run AND the proper_noun trie didn't already cover it | Novel character names / show-specific entities in YouTube subtitles, light novels, manga |
| **Dependency edges** | Validate `ExpressionSpanMatcher` matches: confirm a multi-token vocab idiom only when KWJA's dep arc connects the constituent morphemes | Lets the vocab table be populated aggressively with idioms without cross-clause false positives |
| **BP grouping** | Aggregate compound-particle spans: when `に + よって` are in same BP, attach a span pointing to the grammar-table entry `〜によって` | Solves Jiten cases #01 (`によって`) and #02 (`として`) without an (1) merge |
| **Word-feature labels beyond 敬語** | Generalise `sense_pick::register_for_span`'s pattern to other BP features | Better sense-picking for non-keigo register variation |

The pattern to follow: `kwja_reading_refinement.rs` (gates KWJA evidence on a corroborating vocab-table signal) and `sense_pick::register_for_span` (reads a specific BP feature, applies it to attribute selection).

## Why three crates and not one for layers (1) and (2)

Sibling crates (`sudachi-optimizer` and `sudachi-kwja-optimizer`) rather than a monolithic optimizer crate, because:

1. **Dependency-graph cleanliness**. Search-side consumers (`sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`, paradedb) want only (1). A monolithic crate would force them to pull in `candle`, the GPU model code, and KWJA tokenizer artifacts as transitive deps. **`sudachi-wasm` literally cannot have KWJA deps** — it compiles to wasm32 and candle's GPU stack doesn't go there.

2. **Test environment separation**. (1) tests are CPU-only with golden fixtures. (2) tests need KWJA inference to consume real KWJA output. Different test infrastructure; different runtime requirements.

3. **Update cadence**. (1) updates whenever a new Sudachi tokenisation bug is discovered. (2) updates when KWJA's known failure modes are catalogued. Independent change rates.

4. **Naming clarity**. `sudachi-optimizer` and `sudachi-kwja-optimizer` as siblings make the parallel architecture obvious from the workspace tree. A monolithic `sudachi-core-optimizer` would conflate two distinct jobs under one name.

## Why three layers and not two

A reasonable question: why isn't (3) just "the matcher pipeline" without a distinct layer name? Because (3) does more than matching:

- Matchers walk the (1) morpheme stream and produce span hits
- Hybrid rules combine (1) and (2) signals to refine spans (reading override, register bias, idiom validation)
- Comprehension scoring classifies span acquisition tiers
- Persistence of the consumer-facing combined output

Treating these as one layer with a shared persisted output makes the consumer contract explicit: Anki export reads (3); GraphQL exposes (3); the golden corpus verifies (3). Phase 1/2/2.5/3 ordering inside `analyze.rs` becomes implementation detail of (3) generation, not architecture.

## Cross-references

- [`crates/sudachi-optimizer/CLAUDE.md`](crates/sudachi-optimizer/CLAUDE.md) — (1) layer internals
- [`crates/sudachi-morphology/CLAUDE.md`](crates/sudachi-morphology/CLAUDE.md) — bidirectional conjugator library used by (3) matchers
- [`crates/sudachi-kwja/CLAUDE.md`](crates/sudachi-kwja/CLAUDE.md) — KWJA inference port
- [`crates/sudachi-kwja-optimizer/CLAUDE.md`](crates/sudachi-kwja-optimizer/CLAUDE.md) — (2) layer internals
- [`CLAUDE.md`](CLAUDE.md) — workspace overview, dependency invariants
- `~/CODE/jisho/schema/CLAUDE.md` — (3) consumer domain model: vocab, grammar, kanji, acquisition, scoring, card
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/analyze.rs` — current (3) generation orchestration
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/matchers/` — (3) matchers (vocab, grammar, expression, propnoun)
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/sense_pick.rs` — (3) hybrid rule for register bias
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/kwja_reading_refinement.rs` — (3) hybrid rule for reading override
- `~/CODE/jisho/proto/parse.proto` — gRPC contract for KWJA output
- `crates/sudachi-optimizer/examples/jiten_regression.rs` — runnable harness for the 13 documented Jiten failure cases
