# Comprehension pipeline

> Why this monorepo has three Japanese-NLP crates that look like they overlap, what each one is actually for, and how to decide which crate a new rule belongs in.

## Mission

This monorepo exists to support **second-language Japanese acquisition** via the `jisho` learner-oriented pipeline (vocab lookup, grammar matching, comprehension scoring, FSRS spaced repetition, comprehensible-input content selection). Every architectural decision should be evaluated against one question:

> **Does this output help a learner read Japanese text and convert input into acquired vocabulary and grammar?**

Linguistic correctness, search-engine ergonomics, and parser elegance are means, not ends. They matter only insofar as they serve comprehension. A morphologically "correct" parse that splits set grammar phrases into their etymological components but produces output the learner cannot look up has failed the mission. A "convenient" merge that fuses an idiom into one opaque token but lets the learner skip the lookup entirely has also failed the mission.

The right unit at each layer of the pipeline is the **smallest unit a learner can usefully act on at that layer**. That is different at the morpheme layer (vocab lookup), the bunsetsu layer (grammar pattern), and the cross-bunsetsu layer (idiom recognition).

## Three crates, three jobs, three granularities

```
text
 │
 ▼
┌─────────────────────────────────────┐
│        sudachi-optimizer            │  morphemes — vocab-lookup units
│                                     │
│  Sudachi gateway + 28-rule          │  output: Vec<Morpheme>
│  morpheme-correction pipeline       │  each = "a word the learner can look up"
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│           sudachi-kwja              │  bunsetsu / BP / dependency / cohesion
│                                     │
│  DeBERTa-v2 structural analyser     │  output: Document tree
│  (12 heads on a shared trunk)       │  morphemes grouped into meaning chunks
└─────────────────┬───────────────────┘                + dependency edges
                  │                                    + predicate-argument structure
                  ▼                                    + named entities
       jisho-core consumers
       (vocab matcher, grammar matcher,
        comprehension scorer, idiom detector,
        furigana renderer, ...)


sudachi-morphology  ◄── library used by sudachi-optimizer rules
(forward conjugation +     and by jisho-core vocab lookup
 backward deconjugation,   for lemma resolution
 standalone — no Sudachi
 dependency)
```

Three crates, three completely different jobs at three different granularities. The boundaries are deliberate; understanding them is the whole point of this document.

### sudachi-optimizer — *"what are the words?"*

**Job**: take raw Sudachi/UniDic output and fix its morpheme-level mistakes so each emitted token corresponds to a unit a learner can look up in a dictionary.

**Inputs**: text + Sudachi dictionary (system + optional user dicts)
**Outputs**: `Vec<Morpheme>` — each morpheme has a surface, lemma (`dictionary_form` / `normalized_form`), reading, POS, and an `applied_rules` audit trail of which optimizer rules fired.

**Why it exists**: UniDic was tuned for newspaper-text analysis and search ranking. It produces:

- Over-merged compounds (`足蹴` glued to a passive auxiliary, leaving `られた` as an orphaned passive attached to a noun — grammatically nonsensical)
- Under-merged conjugations (`食べ + て` instead of `食べて`, breaking lemma lookup)
- False-positive proper nouns (`いわね` parsed as a person name in `まずいわね`)
- Mis-classified colloquial forms (`じゃない` broken into `じゃ + なーい` when the speaker emphasised it)
- A long tail of similar errors that are unambiguously *wrong* for vocab lookup

The 28-rule pipeline is organised in five phases — `Split → Repair → Combine → Cleanup → Disambiguation` — that interleave to correct these. Each rule is a single file with a tightly-scoped purpose and unit tests alongside it.

**What it does NOT do**:

- Phrase grouping (that's KWJA)
- Dependency parsing (that's KWJA)
- Idiom recognition (that's a separate semantic layer)
- Reading-furigana generation (that's downstream in `jisho-core`)
- Vocab/grammar lookup (that's downstream)

Each emitted morpheme is a lexical unit. Relationships between morphemes are downstream concerns.

**Gateway role**: this crate is also the workspace's *only* direct importer of upstream `sudachi`. Every other crate (`sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`) imports Sudachi types from `sudachi_optimizer::sudachi::*`. That single-gateway invariant lets rule changes, dictionary swaps, and rev pinning apply uniformly across the workspace.

### sudachi-kwja — *"how do those words chunk into meaning?"*

**Job**: take pre-tokenised morphemes and add the structural layer Japanese readers actually parse — bunsetsu boundaries, dependency arrows, base-phrase trees, predicate-argument structure, named entities, discourse relations.

**Inputs**: `Vec<Vec<SudachiMorpheme>>` (sentences of morphemes, supplied by the caller — typically the optimizer's output)
**Outputs**: `Document` tree — sentences → phrases → base-phrases → morphemes, with dependency edges, cohesion relations, NE spans, and discourse relations.

**Why it exists**: morphemes are the wrong unit for comprehension. A Japanese reader processes 「友達と」 as one chunk meaning "with (my) friend" — not as the three morphemes `友達` + `と` + (whatever attaches next). Bunsetsu (文節) is the smallest grammatical chunk of meaning in Japanese: roughly one content word plus all its grammatical scaffolding (particles, auxiliaries, copula). It is the Japanese equivalent of an English prepositional phrase or noun phrase — a unit you comprehend as one thought.

KWJA's bunsetsu/BP output gives the comprehension-relevant grouping. Its dependency arrows give the syntactic role (subject? modifier? object? complement?). Its cohesion analysis resolves anaphora and ellipsis. Its NE tagger finds names. **None of this is recoverable from morpheme-level output alone** — bunsetsu boundaries require syntax/semantics, not lexicon.

**Hard rule**: KWJA does NOT re-tokenise. The crate's design assumption (and a documented invariant in its CLAUDE.md) is that the caller is responsible for tokenisation; KWJA's word module is forced to honour the supplied morpheme boundaries via subword-to-word pooling. KWJA's own reading/lemma/conjtype/conjform predictions are computed by the model but **discarded** in production in favour of Sudachi's UniDic values, because Sudachi's dictionary-derived values are more authoritative for those fields.

**What KWJA contributes that nothing else can**:

- **Bunsetsu boundaries** — which morphemes form a meaning chunk together
- **Base-phrase tree** — the recursive grouping above bunsetsu
- **Dependency arrows** — which BP modifies which (D = dependent, P = parallel, A = apposition, I = imperfect)
- **Cohesion** — predicate-argument structure, anaphora, bridging, coreference
- **Discourse relations** — cross-sentence connectives
- **NE spans** — named entity recognition (17 BIO tags)
- **Word and BP feature labels** — fine-grained semantic flags

### sudachi-morphology — *"how does this verb conjugate (and unconjugate)?"*

**Job**: forward conjugation (verb + form → surface) and backward deconjugation (surface → candidate dictionary forms with derivation chains). Standalone — no Sudachi dependency.

**Inputs**: a `Verb` + a desired `Conjugation`, OR an arbitrary surface string
**Outputs**: a conjugated surface form, OR a list of candidate dictionary forms each with the chain of derivations that produced the surface

**Why it exists**: lemma resolution is a stack, not a function. Japanese conjugation composes Voice → Mood → Politeness → Polarity → Tense, plus auxiliary verbs, plus colloquial transformations. Downstream consumers need both directions:

- **Forward** — for quiz/display purposes, generate the polite-negative-past form of `食べる` and present it to the learner
- **Backward** — for vocab lookup, see `食べさせられませんでした` in text and recover `食べる` as the source verb plus the chain `[causative, passive, polite, negative, past]`

The forward output of any well-formed verb round-trips through `deconjugate()` back to the original verb — a property guaranteed by the round-trip test corpus.

**Where it sits architecturally**: this is a *library*, not a pipeline stage. It does not appear in the data-flow diagram above because it is consumed *by* layers in that flow:

- `sudachi-optimizer` rules use it for verb-class checks (e.g. deciding whether a Sudachi morpheme matches a known irregular paradigm)
- `jisho-core`'s vocab matcher uses it to deconjugate surface forms into looked-up lemmas

It can also be used standalone for conjugation tables, verb quizzes, etc. — that is by design. The bidirectional taxonomy is a single source of truth for both directions.

## The layered decision rules

Adding a new rule? Use this decision tree to decide which crate it belongs in.

```
Is the change a fix for a Sudachi morpheme being wrong?
   (wrong split, wrong merge, wrong POS, wrong lemma, wrong reading)
   │
   ├── YES ──► sudachi-optimizer
   │           (new rule in split/repair/combine/cleanup/disambiguation)
   │
   └── NO
       │
       Is the change about how morphemes group into meaning chunks?
       (bunsetsu boundaries, BP groupings, which morphemes form a phrase)
       │
       ├── YES ──► sudachi-kwja already handles this. Don't add a rule.
       │           If KWJA is producing wrong groupings, the fix is in
       │           KWJA's word-feature labels or BP labels, not in the
       │           optimizer.
       │
       └── NO
           │
           Is the change about cross-morpheme semantic patterns?
           (idiom recognition, set grammar phrase recognition,
            "this combination of words has a non-compositional meaning")
           │
           ├── YES ──► New layer above the optimizer (idiom detector,
           │           grammar pattern matcher in jisho-core).
           │           These do NOT belong in the optimizer.
           │
           └── Reading disambiguation (homograph)?
               │
               └── sudachi-optimizer disambiguation phase OR KWJA
                   reading_tagger — currently both, prefer the
                   context-aware optimizer rule for high-frequency
                   cases (faster, deterministic, auditable).
```

### The most common architectural mistake: the optimizer doing KWJA's job

**Don't merge morphemes in the optimizer just because they form a phrase together.** KWJA will group them into the same bunsetsu/BP, the comprehension scorer will see them as a unit via that grouping, and you will have lost the ability to look up the constituent morphemes independently.

Concrete examples of layer-confusion to avoid:

| Tempting merge | Why it's wrong here |
|---|---|
| `に + よって → によって` in optimizer | KWJA's bunsetsu groups them into one chunk attached to the head noun. Merging in optimizer loses `よって` as a lookup-able morpheme. The grammar matcher recognises `〜によって` as a pattern across the bunsetsu. |
| `と + し + て → として` in optimizer | Same. KWJA bunsetsu handles the grouping. Grammar matcher recognises the set pattern. |
| `手伝って + 来る → 手伝って来る` in optimizer | `〜て来る` is a productive aspectual construction. KWJA attaches `来る` to `手伝って` via a dependency arrow inside the same BP. Optimizer merge loses both vocab lookups (`手伝う` and `来る` as independent vocab items the learner has acquired) AND the grammar pattern signal (the aspect itself is a learnable grammar point). |
| `手 + を + 抜く → 手を抜く` in optimizer | This is an idiom (non-compositional meaning), not a morpheme-level correction. Belongs in an idiom-detection layer above the optimizer. The morphemes are correct; the *interpretation* is non-literal. That's a semantic layer concern. |

The optimizer's mission is **morpheme correction only**. Phrase grouping is KWJA's job. Idiom recognition is a separate semantic layer.

### When you DO want to merge in the optimizer

The optimizer should merge when the merge produces a single **vocab-lookup unit** that the learner is treating as one word:

| Merge | Why it belongs in the optimizer |
|---|---|
| `食べ + て → 食べて` | Conjugation. Te-form is one verb form for lemma lookup; learner needs to find `食べる`. |
| `食べ + ます + でし + た → 食べました` (with lemma `食べる`) | Politeness/tense conjugation chain. One vocab unit (the verb in a specific form). |
| `お + 寿司 → お寿司` | Honorific prefix is part of the vocab item. |
| `三 + 本 → 三本` | Number + counter is one quantitative unit. |
| `先生 + 様 → 先生様` (when Sudachi over-split) | Bound suffix attached to a noun. |

The discriminator: **does the lookup return a single dictionary entry, or do you want the learner to look up the parts separately?**

- Conjugations and bound morphemes (prefixes, suffixes, counters, te/ta-forms) → merge in the optimizer. The result is one vocab entry.
- Productive grammar constructions (auxiliary chains, set phrases, idioms) → do NOT merge in the optimizer. Let KWJA group at the bunsetsu/BP layer, let the grammar/idiom matcher recognise the pattern across morphemes.

### A useful check

If a learner's textbook lists the construction as **one entry with one meaning** (e.g. `食べる` → "to eat"; `お寿司` → "(honorific) sushi"), the optimizer should produce one morpheme.

If the textbook lists the construction as **a productive grammar pattern** (e.g. `〜ている` → "progressive aspect; combine with any verb"), the optimizer should NOT merge — the head verb and the auxiliary are separate vocab units, and the pattern is recognised by the grammar matcher.

If the textbook lists the construction as **an idiom with a non-compositional meaning** (e.g. `手を抜く` → "slack off", which is unrelated to "hand" + "pull"), the optimizer should NOT merge — the morphemes are correct as morphemes; the idiom-detection layer handles the non-literal interpretation.

## Audit of the existing 28 optimizer rules

Categorising every current rule by the decision rules above. Headlines:

- **No rules need to be removed from the crate** — every rule is doing morpheme-correction work that no other layer can do.
- **Two rules need scope reduction** — `combine/auxiliary` and `combine/verb_dependant` are currently over-merging into territory that belongs to KWJA + grammar matcher.
- **One rule needs extension** — `repair/vowel_elongation` should grow to handle cross-word ー cases.

### Split phase — KEEP all 4

Splits Sudachi over-merges. KWJA cannot un-merge what it's been handed; if Sudachi gave one token, KWJA inherits that boundary. Splits MUST happen at the optimizer layer.

| Rule | Job | Decision |
|---|---|---|
| `split/compound_auxiliary_verbs` | Split compound verbs Sudachi over-merged | KEEP |
| `split/proper_noun_with_particle` | Split a "proper noun" containing a particle | KEEP |
| `split/tan_suffix` | Split たん(suffix) + だ/です | KEEP |
| `split/tatte_particle` | Split tatte/datte particle from miscategorisation | KEEP |
| `split/tawake_noun` | Split たわけ misanalysed as 戯け noun | KEEP |

### Repair phase — KEEP all 11; EXTEND vowel_elongation

These fix Sudachi morpheme-level errors (wrong POS, wrong lemma, missing morpheme boundaries because of normalisation). None are bunsetsu-shaped or idiom-shaped.

| Rule | Job | Decision |
|---|---|---|
| `repair/colloquial_negative_nee` | Recombine colloquial negative ね+え | KEEP |
| `repair/colloquial_ran_nai` | Merge colloquial らん + negative | KEEP |
| `repair/compound_noun_suffix` | Rewrite lemma of merged compound noun | KEEP |
| `repair/fused_interjection_particle` | Split fused `ごめんなさいね` etc. | KEEP — ported from Jiten, unambiguous win |
| `repair/hasa_noun` | Repair はさ misclassification | KEEP |
| `repair/honorific_lemma` | Rewrite lemma of merged honorific form | KEEP |
| `repair/n_tokenisation` | Repair ん particle/copula tokenisation | KEEP |
| `repair/orphaned_auxiliary` | Recover verb stems Sudachi orphaned | KEEP |
| `repair/process_special_cases` | Hand-curated battery of repairs | KEEP |
| `repair/tanka_to_ta_n_ka` | Repair たんか misparsed as 短歌 ("tanka") | KEEP |
| `repair/vowel_elongation` | Repair morphemes broken by elongated vowel | **EXTEND** — add cross-word ー handling for `あなたー`, `じゃなーい`, `来るー`. Heuristic: if stripping a leading/internal ー produces a known UniDic word, do so. |

### Combine phase — KEEP most; SCOPE DOWN auxiliary + verb_dependant; REVIEW particles

This is where the layer-boundary discipline matters most. Most combines produce vocab-lookup units (correct), but two are currently over-merging into KWJA's territory.

| Rule | Job | Decision |
|---|---|---|
| `combine/inflections` | Iteratively merge a base inflectable + suffixes | KEEP — conjugation chains form one vocab unit |
| `combine/tte` | Glue `Xっ + て...` | KEEP — te-form lemma resolution |
| `combine/conjunctive_particle` | Glue て/で/ちゃ/ば onto verb stems | KEEP — conjugation merge |
| `combine/auxiliary_verb_stem` | Glue auxiliary verb stem (そう, etc.) | KEEP — `〜そう` is one vocab item ("looks like X") |
| `combine/prefixes` | Glue Prefix morphemes onto the following base | KEEP — bound morpheme |
| `combine/suffix` | Glue specific Suffix morphemes onto the preceding base | KEEP — bound morpheme |
| `combine/amounts` | Glue numeric morpheme onto a known counter | KEEP — number + counter is one quantitative unit |
| `combine/to_naru` | Re-merge `と + なる` from Sudachi over-split | KEEP — fixes a known Sudachi error; result is one verb form |
| `combine/final_` | Final pass: merge ば onto verbs + sentence-final particles | KEEP — conjugation + lemma alignment |
| `combine/adverbial_particle` | Glue だり/たり onto verbs | KEEP — `〜たり〜たりする` is a recognisable conjugated form for lemma lookup |
| `combine/particles` | Glue specific particle pairs and the negative particle | **REVIEW per-pair** — some entries (e.g. compound particles like `では`, `には`) might be redundant with KWJA bunsetsu grouping. Audit which pairs produce vocab-lookup units (KEEP) vs which produce phrase groupings (DEPRECATE — let KWJA handle). |
| `combine/auxiliary` | Merge Auxiliary morphemes (た, ます, ている, …) | **SCOPE DOWN** — keep merging genuine conjugation aux (た past, ます polite, でし polite-past). **Stop merging productive aux constructions** (`〜ている` progressive, `〜てしまう` completive, `〜てある` resultative, `〜ておく` preparative). Those should remain as separate morphemes so KWJA's BP groups them and the grammar matcher can recognise the aspect/auxiliary pattern as a learnable grammar point. |
| `combine/verb_dependant` | Run four sub-passes to merge various dependants onto verbs | **SCOPE DOWN** — stop merging productive auxiliary verbs (`〜て来る`, `〜て行く`, `〜てみる`, `〜ていく`). Those are KWJA's job to group via dependency, and the grammar matcher recognises the construction as a vocab-pattern unit. Keep merging genuine compound verbs that produce a single dictionary entry (`〜やすい`, `〜にくい`, `〜過ぎる`, `〜始める`, `〜終わる` — these are bound suffixes attached to verb stems that produce one vocab unit). |

### Cleanup phase — KEEP both

| Rule | Job | Decision |
|---|---|---|
| `cleanup/filter_misparse` | Battery of POS reclassifications for surfaces | KEEP — fine-grained POS fixes |
| `cleanup/reclassify_orphaned_suffixes` | Reclassify Suffix morphemes that lost their head | KEEP |

### Disambiguation phase — KEEP

| Rule | Job | Decision |
|---|---|---|
| `disambiguation/fix_reading_ambiguity` | Resolve kanji-homograph reading using neighbouring context | KEEP — context-aware optimizer rule is faster, deterministic, and auditable for high-frequency cases. KWJA's reading_tagger remains the fallback for low-frequency / novel cases. |

## Recommended new work

### New rules to add to the optimizer

These are all morpheme-level Sudachi errors documented in Jiten's regression suite that no other layer can fix. The runnable harness lives at `crates/sudachi-optimizer/examples/jiten_regression.rs`.

1. **Extend `repair/vowel_elongation`** — handle ー glued across word boundaries when stripping ー produces a known UniDic word. Fixes `あなたーそこ` (currently produces `[あ, なたー, そこ, ...]`) and `じゃなーい` (currently `[じゃ, ない]`, should be `じゃない`).

2. **New `split/interrogative_counter`** — split `何 + counter` when Sudachi merges them into a single token (typically returning a rare surname). Fixes `何本ぐらい` (currently parses `何本` as a surname). Generalises to `誰`, `どれ`, `どこ` + various suffixes.

3. **New `split/passive_compound_noun`** — split contextual cases like `足蹴 + られた` where the noun reading is wrong because passive `られる` follows. Gate carefully: `足蹴` is a real noun in `足蹴にする`. Rule should only fire when the next morpheme is the passive auxiliary AND the verb interpretation is well-formed.

4. **Scope down `combine/auxiliary`** — stop merging `〜ている`, `〜てしまう`, `〜てある`, `〜ておく` so the grammar matcher can recognise these as productive aux patterns and the vocab matcher can independently surface the head verb.

5. **Scope down `combine/verb_dependant`** — stop merging `〜て来る`, `〜て行く`, `〜てみる`, `〜ていく` for the same reason.

### Rules NOT to add to the optimizer (layer-confusion examples)

These are tempting to add but belong elsewhere. Listed here to prevent future drift:

- **Compound particle merging** — `によって`, `として`, `について`, `に関して`, `にとって`, `に対して`, `ながら`, `ばかり`, `くらい`, `ように`, `みたいに`, `ところ`, `わけ`, `はず`. KWJA's bunsetsu groups them; the grammar matcher recognises the pattern. Merging in the optimizer destroys the constituent vocab lookups and duplicates work KWJA already does.

- **Idiom merging** — `手を抜く`, `気がつく`, `腹が立つ`, `気を付ける`, `仕方がない`, `〜わけがない`, `〜に違いない`, `世話になる`, `気にする`. These are non-compositional MWE (multi-word expressions) that often span multiple bunsetsu (`手を` is one bunsetsu, `抜いている` is another). Merging in the optimizer would (a) produce one opaque token the learner cannot decompose, (b) require maintaining a lemma mapping for every conjugated variant, (c) make the comprehension scorer report 100% even when the learner only knew the literal parse and missed the idiomatic meaning. Belongs in a separate idiom-detection layer.

- **Set grammar phrase merging** — `〜なければならない`, `〜てはいけない`, `〜ことができる`, `〜ようとする`, etc. These are productive constructions with thousands of inflected variants. Pattern matching belongs in the grammar layer, not as morpheme-level merges.

### New layer to add (above the optimizer, alongside KWJA, consumed by jisho-core)

An **idiom / set-phrase detection layer** is the single highest-leverage piece of unfinished work for comprehension. It does not belong in the optimizer.

Suggested architecture:

- A curated list of idioms keyed on a normalised morpheme sequence (head verb + its arguments / particles)
- Applied after the optimizer's morpheme stream is clean AND after KWJA's BP/dependency tree is built
- Output: spans of `(start_morpheme, end_morpheme, idiom_id, gloss)` attached to the analysis result
- Consumed by the comprehension scorer: a learner who doesn't know the idiom should NOT get credit for the literal compositional reading. Without this layer, the score lies — it reports comprehension based on knowing each literal morpheme even when the meaning is non-compositional.

This solves Jiten test cases #08 (`手を抜いている`) and #12 (`気がついて`) and the long tail of common Japanese idioms without polluting the optimizer.

The home for this layer is most naturally `jisho-core/idiom/` (consumes both optimizer morphemes and KWJA structure), or a new standalone crate `sudachi-idioms` if it stays in this monorepo.

## Why three crates and not one

A reasonable question: why isn't this all one crate? The answer is that the three jobs have **different inputs, different outputs, different runtime characteristics, and different update cadences**:

- **sudachi-optimizer** is rule-based and deterministic. CPU-bound. Updates whenever a new tokenisation bug is discovered. Tested with golden fixtures and a regression harness. Loads ~70 MB Sudachi dictionary.
- **sudachi-kwja** is neural and stochastic-but-deterministic-per-checkpoint. GPU-bound (fp16 on CUDA in production). Updates when the upstream KWJA model is retrained. Tested with argmax-equivalence vs the upstream Python reference. Loads ~1 GB of safetensors checkpoints + tokenizer artifacts.
- **sudachi-morphology** is rule-based, deterministic, has no external assets, and runs on tiny inputs (one verb at a time). Updates when a new conjugation pattern is documented. Tested with ~4,800 golden cases.

Putting them in one crate would conflate their dependency graphs, runtime requirements, and test infrastructure. Keeping them separate lets each crate have the right toolchain and the right test discipline for its job, and lets consumers depend on only what they need (e.g. `sudachi-search` depends on the optimizer for its tokenizer gateway but has no use for KWJA or the deconjugator).

## Cross-references

- [`crates/sudachi-optimizer/CLAUDE.md`](crates/sudachi-optimizer/CLAUDE.md) — internals of the rule pipeline, stage-by-stage breakdown, gateway role
- [`crates/sudachi-morphology/CLAUDE.md`](crates/sudachi-morphology/CLAUDE.md) — bidirectional conjugator design, verb-class taxonomy
- [`crates/sudachi-kwja/CLAUDE.md`](crates/sudachi-kwja/CLAUDE.md) — KWJA inference port, head architecture, hot path for pre-tokenized input
- [`CLAUDE.md`](CLAUDE.md) — workspace overview, dependency invariants, two-product-surfaces explanation
- `~/CODE/jisho/schema/CLAUDE.md` — downstream consumer (jisho-core) domain model: vocab, grammar, kanji, acquisition, scoring, card, etc.
- `crates/sudachi-optimizer/examples/jiten_regression.rs` — runnable harness for the 13 Jiten test cases. Re-run after any rule change; the summary line tells you if you regressed anything.
