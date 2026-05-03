# Comprehension pipeline

> What each crate in this monorepo actually does, who actually consumes its output, and how to decide where new work belongs.

## How this doc was written

Every "purpose" claim in this document is derived from **what consumers actually do today** in `~/CODE/jisho/`, not from theoretical architecture. Each layer's purpose is the intersection of (a) what it produces and (b) what downstream code reads. Claims are cited with file:line references so they can be verified and updated when the code changes. If you change consumption (e.g. start using a KWJA head that's currently ignored), update this doc.

## Mission

This monorepo exists to support **second-language Japanese acquisition** via the `jisho` learner-oriented pipeline (vocab lookup, grammar matching, comprehension scoring, FSRS spaced repetition, comprehensible-input content selection). Every architectural decision should serve one question:

> **Does this output help a learner read Japanese text and convert input into acquired vocabulary and grammar?**

Linguistic correctness, search-engine ergonomics, and parser elegance are means, not ends. They matter only insofar as they serve comprehension. A morphologically correct parse that splits set grammar phrases into their etymological components but produces output the learner cannot look up has failed the mission.

## What each crate produces, and who actually consumes it

The three crates collaborate but **do not have symmetric roles**. The optimizer is on the critical path for matching; KWJA is a decoration layer that runs after matching is complete; morphology is a library used by both. The current consumption pattern is asymmetric and that asymmetry is load-bearing for correct architectural decisions.

### sudachi-optimizer — the critical-path tokenizer

**Produces**: `Vec<Morpheme>` with surface, lemma (`dictionary_form` / `normalized_form`), reading, POS array, char range, and an `applied_rules` audit trail.

**Consumed by** (via the converted `Token` type at `jisho-core/src/analysis/types.rs:158`):

- **All grammar matchers** walk a flat `&[Token]` slice. `jisho-core/src/analysis/matchers/grammar.rs:1-88` defines four matcher structs (`GrammarSpanMatcher`, `VolitionalSuffixMatcher`, `GrammarNgramMatcher`, `GrammarSuffixMatcher`) — every one of them takes `tokens: &[Token]` and matches by walking the sequence. **None of them know about bunsetsu boundaries.**
- **Multi-token expression detection** in `jisho-core/src/analysis/matchers/expression.rs:53-102` concatenates token surfaces and runs `index.vocab_common_prefix_search()` against the vocab trie. Multi-token vocab entries (idioms like `確かに`, `道を聞く`, `時に`) are detected purely from the optimizer's token stream — KWJA is not involved.
- **Vocab matching** uses the same flat stream.
- **Comprehension scoring** at `jisho-core/src/scoring/comprehension.rs:75-150` iterates `spans` (which are sequences of these tokens) classified by `MatchLayer` and checks each vocab/grammar ID against acquisition state. **No structural information from KWJA enters the score.**
- **Phase 1 of the analyze pipeline** (`jisho-core/src/analysis/analyze.rs`) runs all matchers and produces final span boundaries from the optimizer's token stream alone. **Spans are final before KWJA ever runs.**

**Therefore the optimizer's purpose is** to produce a flat morpheme stream where each token corresponds to either (a) a single dictionary-lookup unit, or (b) a discrete component of a multi-token pattern that downstream matchers can recognise. The granularity choice for every optimizer rule should be evaluated against: *will downstream matchers walk this stream and find the right vocab/grammar/expression spans?*

### sudachi-kwja — the post-matching decoration layer

**Produces**: a `Document` tree with `Phrase` (bunsetsu), `BasePhrase`, `Morpheme`, dependency edges, and per-BP feature labels. The proto contract is at `~/CODE/jisho/proto/parse.proto:194-259`.

**Consumed by** — and this list is short and specific:

1. **Sense-picking register bias.** `jisho-core/src/analysis/sense_pick.rs:98-120` defines `register_for_span()`. It walks `parse_tree.sentences[0].base_phrases` looking for a `敬語` (keigo) feature on the BP that contains the span's character range. If found, the value (humble / honorific / polite) biases which sense from the vocab entry's sense list is shown to the learner. **This is the only place KWJA's BP features influence what the learner sees.**

2. **Reading refinement for kanji homographs.** `jisho-core/src/analysis/kwja_reading_refinement.rs:1-30` is "Phase 2.5" of the analyze pipeline. KWJA's transformer-based reading tagger is more context-aware than UniDic's static reading. Examples from the file: `人 in 英吉利人` reads じん, not Sudachi's default にん; `方 in 次の方` reads かた, not ほう; `日 in 体育の日` reads ひ, not にち. The refinement is two-gated (KWJA disagrees with Sudachi AND the KWJA reading is corroborated by a vocab-table entry) so KWJA's drift cases don't propagate.

3. **BM25 search-text encoding for passages.** `jisho-core/src/passage/search_text.rs:72-137` encodes structural atoms (bunsetsu surfaces as "B", base_phrase dep_type as "BP", PAS relations, semantic categories as "SEM") into `passage_parse_tree.search_text`. This indexed text is searched by `services/jisho-graphql/src/schema/passage/query.rs:68-81` for cross-passage retrieval. **Discourse relations are stored in the proto but never queried; cohesion was deleted in 2026-05 with "zero consumers".**

**Consumption flow** — KWJA runs in **Phase 2** of the analyze pipeline (`jisho-core/src/analysis/analyze.rs:645+`), specifically `decorate_with_kwja_batch()`. Today this runs **after** Phase 1 has produced span boundaries — but the ordering is implementation-driven (the existing hybrid rules refine attributes of already-matched spans, not the boundaries themselves), not a fundamental constraint. Both Sudachi+optimizer output and KWJA output are persisted to the database alongside the analyzed text; any consumer that re-reads an analyzed passage gets both signals from the cache. **KWJA is treated as always-present infrastructure, not best-effort decoration.**

**Therefore KWJA's purpose today is**:

- Refine readings for furigana display when Sudachi's context-blind default is wrong (Phase 2.5 hybrid rule)
- Bias sense picking when the speaker is using honorific/humble register (Phase 3 hybrid rule)
- Provide structural atoms (BP boundaries, PAS, SEM) for BM25 indexing of analyzed passages

That is the **current** contribution — but the architecture supports more. KWJA produces additional signal (NE spans, dependency edges, BP feature labels beyond 敬語, discourse relations) that no consumer currently reads. The "what KWJA could be used for but isn't" section below catalogues these as candidates for new hybrid rules. Because KWJA is always present and always cached, adding a new hybrid rule does not require a new architectural layer or a new conditional fallback — it just consumes both streams.

### sudachi-morphology — the bidirectional conjugation library

**Produces**: forward conjugation (verb + form → surface) and backward deconjugation (surface → candidate dictionary forms with derivation chains). Standalone, no Sudachi dependency.

**Consumed by**:

- `sudachi-optimizer` rules that need verb-class checks
- `jisho-core` vocab matcher when surface forms must be deconjugated to look up a lemma
- Quiz/display surfaces that need to generate specific conjugated forms (polite past, causative-passive, etc.)

**Therefore morphology's purpose is** to provide bidirectional conjugation as a library, used wherever lemma resolution or form generation is needed. It is a *library*, not a pipeline stage — it doesn't appear on the data-flow path; it's invoked from inside the layers that do.

## How the layers harmoniously aggregate

Concretely, in pipeline order (`jisho-core/src/analysis/analyze.rs` orchestration):

```
Input: raw Japanese text
   │
   ▼
┌──────────────────────────────────────────────────────────────────┐
│  Phase 1 — analyze_structural                                    │
│    sudachi-optimizer.tokenize() → Vec<Token>                     │
│    Run all SpanMatchers in priority order:                       │
│      GrammarSpan, GrammarNgram, ExpressionSpan,                  │
│      GrammarDirect, ProperNounSpan, Vocab                        │
│    Output: Vec<AnalyzedSpan> — span boundaries are FINAL here    │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│  Phase 2 — decorate_with_kwja_batch                              │
│    Flatten spans back into tokens, send to jisho-parse RPC       │
│    Receive Document tree, attach to AnalyzedText.parse_tree      │
│    Both Sudachi and KWJA outputs are persisted to the DB; any    │
│    re-analysis of stored passages is a cache hit on both signals │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│  Phase 2.5 — kwja_reading_refinement                             │
│    Walk parse_tree, override per-token readings where:           │
│      (a) KWJA disagrees with Sudachi, AND                        │
│      (b) the KWJA reading exists in the vocab table              │
│    Affects furigana display, not span boundaries                 │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│  Phase 3 — refine_vocab_with_jitendex                            │
│    Re-pick senses, biased by KWJA BP features (敬語) via          │
│    sense_pick::register_for_span()                               │
│    Affects which gloss is shown, not span boundaries             │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
                  AnalyzedText
                  (spans + parse_tree + refined senses)
                         │
                         ▼
       Comprehension scoring (uses spans only)
       GraphQL exposure (exposes spans, parse_tree, senses)
       Search-text encoding (uses parse_tree atoms for BM25)
```

The phase ordering is sequential by dependency, not by importance:

- **Phase 1 (optimizer + matchers)** establishes span boundaries first because the matchers operate on flat morpheme sequences and need final boundaries to score against.
- **Phase 2/2.5/3 (KWJA-driven refinement)** runs after spans because today's hybrid rules (reading refinement, sense register bias) refine attributes of already-matched spans rather than altering what was matched.

This ordering is a current implementation choice, not a fundamental constraint. Both signals are computed on every analysis and persisted; the architecture is now best understood as **two complementary signal streams that both inform the final analysis**, not as a critical path with optional decoration. The Rust port of KWJA brought boot time from seconds (Python GIL) to ~50ms and made the service rock-solid; KWJA output is present on 100% of analyzed passages (e.g. all ~8,900 Anki-card passages have both Sudachi+optimizer and KWJA stored). New rules can rely on both being available.

The implication for new architecture: **rules can be additive**. A rule that needs only morpheme features runs in the optimizer pipeline (28 examples today). A rule that needs only KWJA output runs in jisho-core (none today). A *hybrid* rule that needs both runs wherever both are accessible (today: jisho-core's Phase 2.5 reading refinement and Phase 3 sense register bias). The hybrid pattern is the natural home for everything that needs cross-signal aggregation — and with KWJA always available, hybrid rules don't need conditional fallback logic; they can just consume both inputs directly.

## The layered decision rules

Adding a new rule? First decide **which signals it needs**, then **where it runs**:

```
What signals does the rule need?
   │
   ├── Morpheme features only (Sudachi+optimizer output)
   │     │
   │     └── Pure morpheme rule. Lives in sudachi-optimizer.
   │         28 examples today (split/repair/combine/cleanup/disambiguation).
   │
   ├── KWJA features only (parse_tree output)
   │     │
   │     └── Pure KWJA rule. Lives in jisho-core (KWJA output is
   │         attached to AnalyzedText after Phase 2; rule reads from
   │         result.parse_tree).
   │         Zero examples today, but candidates exist (see "What KWJA
   │         could be used for" below — NE-derived ProperNounSpans is
   │         the lowest-hanging).
   │
   └── Both signals (HYBRID rule)
         │
         └── Lives in jisho-core where both are accessible.
             Today's examples: kwja_reading_refinement (Phase 2.5),
             sense_pick::register_for_span (Phase 3). Both gate KWJA
             evidence on a corroborating Sudachi+optimizer or vocab-
             table signal — that gating pattern is the template for
             new hybrid rules.

Then: which kind of change?
   │
   ├── Span boundaries / what gets matched
   │     │
   │     └── Today: pure morpheme rule in sudachi-optimizer (Phase 1).
   │         Future option: a HYBRID rule that gates a span merge on
   │         KWJA bunsetsu evidence (e.g. merge `に + よって` only when
   │         KWJA puts them in same BP). Would need to run as part of
   │         (or before) Phase 1 matching — see "Future architecture"
   │         below.
   │
   ├── A single dictionary-lookup unit appearing as one token
   │     │
   │     └── sudachi-optimizer combine rule (e.g. te-form + verb stem,
   │         number + counter, prefix + noun). Pure morpheme.
   │
   ├── A multi-token grammar pattern (なくてはならない, 〜ている, 〜て来る)
   │     │
   │     └── DO NOT merge in optimizer. The grammar matcher
   │         (GrammarNgramMatcher etc.) walks a flat token stream and
   │         pattern-matches across tokens. Add the pattern to the
   │         grammar trie. Tokens must remain SEPARATE so the matcher
   │         can find them. (Optionally a hybrid rule could validate
   │         the match against KWJA dependency, but the trie hit comes
   │         first.)
   │
   ├── A multi-token vocab expression / idiom (確かに, 手を抜く, 気がつく)
   │     │
   │     └── DO NOT merge in optimizer. Add the expression as a
   │         multi-token entry to the vocab table. The
   │         ExpressionSpanMatcher detects it via
   │         vocab_common_prefix_search. (A hybrid rule could gate
   │         confirmation on KWJA dep arc to avoid cross-clause false
   │         matches.)
   │
   ├── Furigana display correctness for a homograph kanji
   │     │
   │     └── Hybrid rule. Today: kwja_reading_refinement.rs gates
   │         KWJA's reading override on a vocab-table corroboration.
   │         Same pattern for new homograph cases.
   │
   ├── Honorific / humble / polite gloss selection
   │     │
   │     └── Hybrid rule. Today: sense_pick::register_for_span uses
   │         KWJA BP 敬語 feature. Generalisable to other BP feature
   │         labels.
   │
   └── BM25 retrieval atoms for cross-passage search
         │
         └── search_text.rs encoder. Reads parse_tree directly.
```

### The most common architectural mistake

**The optimizer doing the matchers' job by pre-merging multi-token patterns.**

Concretely: merging `〜ている`, `〜てしまう`, `〜て来る`, `〜による` etc. in the optimizer makes the constituent morphemes invisible to (a) the vocab matcher (which can no longer surface `来る` as a vocab item the learner is studying) and (b) the grammar matcher (which can no longer recognise the productive pattern across tokens). The merged token has no home — it's not a dictionary entry, it's not a grammar pattern, it's a hybrid that nothing downstream knows what to do with.

The right choice for productive multi-token patterns: **leave them as separate tokens in the optimizer; let the appropriate matcher pick them up downstream**.

### When the optimizer SHOULD merge

The optimizer should merge when the merge produces a token that maps to **exactly one entry in either the vocab table or the grammar table**, and that entry is what the learner needs to see:

| Merge | Why optimizer is the right home |
|---|---|
| `食べ + て → 食べて` | Conjugation. The learner needs to find `食べる` via this inflected form. |
| `食べ + ます + でし + た → 食べました` (lemma `食べる`) | Politeness/tense conjugation chain. One vocab unit. |
| `お + 寿司 → お寿司` | Honorific prefix is part of the vocab entry. |
| `三 + 本 → 三本` | Number + counter is one quantitative unit. |
| `先生 + 様 → 先生様` (when Sudachi over-split) | Bound suffix attached to a noun. |

The discriminator is: **does the resulting token exist as exactly one entry in the vocab or grammar table?** If yes, optimizer merge is correct. If the resulting token would be a productive multi-token pattern (like `〜ている`), the merge is wrong — leave the tokens separate and let the matcher handle it.

### A useful test

When deciding whether a multi-token construction belongs as an optimizer merge, a vocab table entry, or a grammar table entry:

- **Optimizer merge** if it's a conjugation chain, bound morpheme, or fixed lexicalisation that is *one dictionary entry*.
- **Vocab table entry** if it's a multi-word expression with a non-compositional or fixed meaning (idioms, set phrases, locutions like `確かに`, `手を抜く`).
- **Grammar table entry** if it's a productive structural pattern that combines with arbitrary content words (`〜ている` aspect, `〜なくてはならない` obligation, `〜によって` instrumental, `〜として` role).

The vocab and grammar tables are populated independently of the optimizer. Adding entries to them does not require changing optimizer rules — the matchers will pick them up via trie search.

## Audit of the existing 28 optimizer rules

Categorising every current rule by the decision rules above. Headlines:

- **All splits, repairs, and cleanups**: KEEP — they correct Sudachi morpheme-level errors that no other layer can fix.
- **Most combines**: KEEP — they produce single-vocab-entry tokens that the matchers consume.
- **Two combines need scope reduction**: `combine/auxiliary` and `combine/verb_dependant` are merging productive grammar patterns that would be better recognised by the GrammarNgramMatcher. Justification: not "let KWJA do it" (KWJA doesn't), but "let the grammar matcher walk the flat token stream and detect the pattern".
- **One repair needs extension**: `repair/vowel_elongation` should grow to handle cross-word ー cases.

### Split phase — KEEP all 5

Splits Sudachi over-merges. KWJA cannot un-merge what it's been handed. Splits MUST happen at the optimizer layer.

| Rule | Job | Decision |
|---|---|---|
| `split/compound_auxiliary_verbs` | Split compound verbs Sudachi over-merged | KEEP |
| `split/proper_noun_with_particle` | Split a "proper noun" containing a particle | KEEP |
| `split/tan_suffix` | Split たん(suffix) + だ/です | KEEP |
| `split/tatte_particle` | Split tatte/datte particle from miscategorisation | KEEP |
| `split/tawake_noun` | Split たわけ misanalysed as 戯け noun | KEEP |

### Repair phase — KEEP all 11; EXTEND vowel_elongation

These fix Sudachi morpheme-level errors. None are bunsetsu-shaped or multi-token-pattern-shaped.

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
| `repair/vowel_elongation` | Repair morphemes broken by elongated vowel | **EXTEND** — add cross-word ー handling for `あなたー`, `じゃなーい`, `来るー` (heuristic: if stripping the ー produces a known UniDic word, do so) |

### Combine phase — KEEP most; SCOPE DOWN auxiliary + verb_dependant

This is where the discipline matters most.

| Rule | Job | Decision |
|---|---|---|
| `combine/inflections` | Iteratively merge a base inflectable + suffixes | KEEP — produces one vocab-entry-resolvable token |
| `combine/tte` | Glue `Xっ + て...` | KEEP — te-form lemma resolution |
| `combine/conjunctive_particle` | Glue て/で/ちゃ/ば onto verb stems | KEEP — conjugation merge |
| `combine/auxiliary_verb_stem` | Glue auxiliary verb stem (そう, etc.) | KEEP — `〜そう` is one vocab item ("looks like X") |
| `combine/prefixes` | Glue Prefix morphemes onto the following base | KEEP — bound morpheme |
| `combine/suffix` | Glue specific Suffix morphemes onto the preceding base | KEEP — bound morpheme |
| `combine/amounts` | Glue numeric morpheme onto a known counter | KEEP — number + counter is one quantitative unit |
| `combine/to_naru` | Re-merge `と + なる` from Sudachi over-split | KEEP — fixes a Sudachi error; result is one verb form |
| `combine/final_` | Final pass: merge ば onto verbs + sentence-final particles | KEEP — conjugation + lemma |
| `combine/adverbial_particle` | Glue だり/たり onto verbs | KEEP — these are part of conjugation chains for lemma lookup |
| `combine/particles` | Glue specific particle pairs and the negative particle | **REVIEW per-pair** — pairs that produce a single grammar-table entry (e.g. `では` as a topic-particle compound) might be better as grammar-table multi-token entries; pairs that are conjugation merges should stay |
| `combine/auxiliary` | Merge Auxiliary morphemes (た, ます, ている, ...) | **SCOPE DOWN** — keep merging genuine conjugation aux (た past, ます polite, でし polite-past). **Stop merging productive aux constructions** (`〜ている`, `〜てしまう`, `〜てある`, `〜ておく`). Those should remain as separate morphemes so the GrammarNgramMatcher (`grammar.rs`) can recognise them as productive grammar patterns via flat-stream matching, and the vocab matcher can independently surface the head verb. |
| `combine/verb_dependant` | Run four sub-passes to merge various dependants onto verbs | **SCOPE DOWN** — stop merging productive auxiliary verbs (`〜て来る`, `〜て行く`, `〜てみる`, `〜ていく`). Same reason: leave them as separate tokens, populate the grammar table with the patterns, the matcher will detect them. Keep merging genuine compound verbs that produce a single dictionary entry (`〜やすい`, `〜にくい`, `〜過ぎる`, `〜始める`, `〜終わる`). |

### Cleanup phase — KEEP both

| Rule | Job | Decision |
|---|---|---|
| `cleanup/filter_misparse` | Battery of POS reclassifications for surfaces | KEEP |
| `cleanup/reclassify_orphaned_suffixes` | Reclassify Suffix morphemes that lost their head | KEEP |

### Disambiguation phase — KEEP

| Rule | Job | Decision |
|---|---|---|
| `disambiguation/fix_reading_ambiguity` | Resolve kanji-homograph reading using neighbouring context | KEEP — runs in Phase 1 (before KWJA). Phase 2.5 `kwja_reading_refinement` is the second pass that uses KWJA's contextual reading where it improves on Sudachi+optimizer's choice. The two are complementary, not redundant. |

## Recommended new work

### New optimizer rules (Sudachi morpheme errors, no other layer can fix)

The runnable harness for the documented Jiten failure cases is at `crates/sudachi-optimizer/examples/jiten_regression.rs`.

1. **Extend `repair/vowel_elongation`** — handle ー glued across word boundaries when stripping ー produces a known UniDic word. Fixes `あなたーそこ` (currently `[あ, なたー, そこ, ...]`) and `じゃなーい` (currently `[じゃ, ない]`).

2. **New `split/interrogative_counter`** — split `何 + counter` when Sudachi merges them as a rare surname. Fixes `何本ぐらい`. Generalises to `誰`, `どれ`, `どこ` + suffixes.

3. **New `split/passive_compound_noun`** — split contextual cases like `足蹴 + られた` where the noun reading is wrong because passive `られる` follows. Gate carefully — `足蹴` is a real noun in `足蹴にする`. Rule should fire only when the next morpheme is the passive auxiliary AND the verb interpretation is well-formed.

4. **Scope down `combine/auxiliary`** — see audit above. Stop merging productive aux constructions so `GrammarNgramMatcher` can detect them.

5. **Scope down `combine/verb_dependant`** — see audit above. Stop merging productive aux verbs.

### NOT new optimizer rules — populate the vocab and grammar tables instead

These are tempting to add as optimizer rules but belong in the data layer, where consumers already exist:

- **Compound particles** — `によって`, `として`, `について`, `に関して`, `にとって`, `に対して`, `ながら`, `ばかり`, `くらい`, `ように`, `みたいに`, `ところ`, `わけ`, `はず`. These are productive structural patterns combining with arbitrary content words. **Add them as multi-token entries in the grammar table.** `GrammarNgramMatcher` (`jisho-core/src/analysis/matchers/grammar.rs`) walks the flat token stream and matches multi-token grammar patterns via the grammar trie. The optimizer must keep `に + よって` as separate tokens for the matcher to find them.

- **Idioms / multi-word expressions** — `手を抜く`, `気がつく`, `腹が立つ`, `気を付ける`, `仕方がない`, `世話になる`, `気にする`. These are non-compositional fixed expressions. **Add them as multi-token vocab entries.** `ExpressionSpanMatcher` (`jisho-core/src/analysis/matchers/expression.rs:53-102`) detects multi-token vocab via `vocab_common_prefix_search()` on the vocab trie. Already works for entries like `確かに` and `道を聞く`; the work is data, not architecture. The optimizer must keep the constituent morphemes as separate tokens.

- **Set grammar phrases** — `〜なければならない`, `〜てはいけない`, `〜ことができる`, `〜ようとする`. Same story — populate the grammar table; the matcher handles detection.

The architectural significance: **adding new entries to the vocab and grammar tables does not require new optimizer rules and does not require a new architectural layer.** The matchers are already in place, the trie indices are already built, the comprehension scorer already classifies these match types. The only thing missing is curated data.

### Candidate new hybrid rules (KWJA signal currently unused)

KWJA produces several signals that are computed and persisted but not yet consumed. With KWJA always present, each of these is a candidate for a new hybrid rule in `jisho-core` that consumes both Sudachi+optimizer output and `parse_tree`:

| KWJA signal | Candidate hybrid rule | What it would solve |
|---|---|---|
| **NE spans** (B-PERSON / B-LOC / B-ORG runs) | Augment proper-noun detection: surface a `ProperNounSpan` when KWJA NE labels a token run AND the proper_noun trie didn't already cover it. | Novel character names / show-specific entities in YouTube subtitles, light novels, manga — content where the trie can't keep up. |
| **Dependency edges** (head / dep_type per BP) | Validate `ExpressionSpanMatcher` matches: confirm a multi-token vocab idiom only when KWJA's dep arc connects the constituent morphemes (vs them being coincidentally adjacent across a clause boundary). | Lets the vocab table be populated aggressively with idioms (`手を抜く`, `気がつく`) without cross-clause false positives. |
| **BP grouping** (which morphemes are in the same bunsetsu) | Aggregate compound-particle spans: when `に + よって` are in the same BP, attach a span pointing to the grammar-table entry `〜によって`. | Solves Jiten test cases #01 (`によって`) and #02 (`として`) without requiring an optimizer merge that would lose the constituent vocab lookups. |
| **Word-feature labels** beyond 敬語 | Generalise `sense_pick::register_for_span`'s pattern to other BP features (e.g. tone, formality dimensions beyond keigo). | Better sense-picking for non-keigo register variation. |
| **Discourse relations** | Cross-sentence connective detection for passage-level comprehension scoring (does the learner understand the discourse marker between sentences?). | Currently zero consumers; would need a downstream comprehension model that operates above sentence level. |
| **PAS in BP relations** | Encoded as BM25 atoms but never structurally traversed. Could power "find sentences with this argument structure" queries. | Cross-passage retrieval beyond text similarity. |

The architecture for adding any of these is **a new hybrid rule in `jisho-core`** that reads both signals — no new crate, no architectural restructure. The pattern to follow: `kwja_reading_refinement.rs` (gates KWJA evidence on a corroborating Sudachi+optimizer or vocab-table signal) and `sense_pick::register_for_span` (reads a specific BP feature, applies it to attribute selection).

### Future architecture: hybrid rules and span-influencing rules

The current architecture runs all hybrid rules **after** Phase 1 spans are final, because today's hybrid rules (reading refinement, sense bias) only refine attributes of already-matched spans. There is no fundamental reason a hybrid rule couldn't influence span boundaries — that ordering would just need to change.

Two evolution paths are open if and when they become useful:

1. **Span-validating hybrid rules (low-risk, additive).** Run after Phase 1 matching; can suppress, confirm, or annotate matches based on KWJA evidence. The dependency-validation idiom rule above is an example. Doesn't change the matcher logic; just filters the output. Migrate this first.

2. **Span-influencing hybrid rules (higher-stakes, restructure).** Would need to run as part of (or before) Phase 1, possibly altering tokenisation or proposing spans the matchers wouldn't otherwise find. The compound-particle aggregation rule above is an example. Requires deciding how to merge proposals from multiple sources (KWJA-proposed spans + matcher-found spans + their conflicts). Migrate this only when there's a concrete win that span-validation can't capture.

Both paths fit the additive feature-set model: rules consume the signals they need; the runner orchestrates them; nothing requires conditional fallback logic because both signals are always there.

Until specific hybrid rules are designed and committed, do not pre-emptively restructure the pipeline. Add hybrid rules incrementally — `kwja_reading_refinement.rs` is the template; new rules are siblings.

## Why three crates and not one

- **sudachi-optimizer** is rule-based and deterministic. CPU-bound, in-process. Updates whenever a tokenisation bug is discovered. Tested with golden fixtures and the Jiten regression harness. Loads ~70 MB Sudachi dictionary.
- **sudachi-kwja** is neural and deterministic-per-checkpoint. GPU-bound (fp16 on CUDA in production); the Rust port took boot time from seconds (Python GIL bottleneck) to ~50 ms. Run via gRPC service (`jisho-parse`) to keep the GPU process pool out of every consumer's address space. Updates when the upstream KWJA model is retrained. Tested with argmax-equivalence vs the upstream Python reference. Loads ~1 GB of safetensors checkpoints + tokenizer artifacts.
- **sudachi-morphology** is rule-based, deterministic, no external assets, runs on tiny inputs. Updates when a new conjugation pattern is documented. Tested with ~4,800 golden cases.

Putting them in one crate would conflate dependency graphs, runtime requirements, and test discipline. Keeping them separate lets each have the right toolchain and lets consumers depend on only what they need.

Operationally, **both Sudachi+optimizer output and KWJA output are persisted** alongside the analyzed text in the `passage_parse_tree` table. Re-analysing a stored passage is a cache hit on both signals; no recomputation. KWJA is treated as always-present infrastructure (100% coverage on the ~8,900 Anki-card passages, for example), not as a fallback-on-failure dependency. Hybrid rules can rely on that.

## Cross-references

- [`crates/sudachi-optimizer/CLAUDE.md`](crates/sudachi-optimizer/CLAUDE.md) — internals of the rule pipeline, stage-by-stage breakdown, gateway role
- [`crates/sudachi-morphology/CLAUDE.md`](crates/sudachi-morphology/CLAUDE.md) — bidirectional conjugator design, verb-class taxonomy
- [`crates/sudachi-kwja/CLAUDE.md`](crates/sudachi-kwja/CLAUDE.md) — KWJA inference port, head architecture, hot path for pre-tokenized input
- [`CLAUDE.md`](CLAUDE.md) — workspace overview, dependency invariants, two-product-surfaces explanation
- `~/CODE/jisho/schema/CLAUDE.md` — downstream consumer (jisho-core) domain model: vocab, grammar, kanji, acquisition, scoring, card, etc.
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/analyze.rs` — Phase 1/2/2.5/3 orchestration; the canonical reference for layer ordering
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/matchers/` — grammar, expression, vocab matchers (all walk flat token stream)
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/sense_pick.rs` — the only place KWJA BP features influence learner-visible output
- `~/CODE/jisho/packages/rs/jisho-core/src/analysis/kwja_reading_refinement.rs` — Phase 2.5 reading refinement
- `~/CODE/jisho/proto/parse.proto` — gRPC contract between jisho-core and jisho-parse (KWJA output shape)
- `crates/sudachi-optimizer/examples/jiten_regression.rs` — runnable harness for the 13 documented Jiten failure cases
