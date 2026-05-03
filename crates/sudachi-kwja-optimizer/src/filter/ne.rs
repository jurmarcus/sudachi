//! `FilterNe` — drop spurious NE feature entries from BasePhrases.
//!
//! KWJA emits NE annotations as a `KeyValue { key: "NE", value:
//! "{TYPE}:{surface}" }` on the BasePhrase that contains the entity
//! head (see `sudachi-kwja/src/pipeline.rs:1018-1043`). Possible
//! types per `resources/labels.json`:
//!
//!   PERSON, LOCATION, ORGANIZATION, ARTIFACT,
//!   DATE, TIME, MONEY, PERCENT
//!
//! The model over-tags in known type-specific ways:
//!
//! - **Proper-noun tags** (PERSON / LOCATION / ORGANIZATION): false
//!   positives are usually pure-hiragana common nouns. Real proper
//!   nouns are written in kanji, katakana, or Latin script.
//! - **ARTIFACT**: noisiest tag. Single-kanji ARTIFACT spans are
//!   almost always common nouns (本, 車, 家). Pure-hiragana ARTIFACT
//!   spans are also typically misclassifications.
//! - **Temporal/numeric tags** (DATE / TIME / MONEY / PERCENT): can
//!   legitimately be pure hiragana ("きのう" = yesterday). We don't
//!   apply the script-based filters to these.
//!
//! Universal filters apply to all types: drop empty surfaces, drop
//! malformed `value` strings (anything not matching `TYPE:SURFACE`).
//!
//! ## Why this is layer (2) and not layer (3)
//!
//! All filtering decisions are mechanical surface heuristics — no
//! vocab corroboration, no learner state. A jisho-aware augmentation
//! ("surface NE-tagged spans as ProperNounSpans when the trie didn't
//! catch them") is a separate (3) hybrid rule that consumes the
//! cleaned NE output this stage produces.

use crate::doc_features::DocumentFeatures;
use crate::document::Document;
use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};

/// Stage name used in audit trails.
pub const NAME: &str = "filter_ne";

/// Build the canonical stage for the pipeline.
pub fn stage() -> Stage {
    Stage::new(
        NAME,
        Phase::Filter,
        DocumentFeatures::HAS_NE_FEATURES,
        |doc, lex| apply(doc, lex),
    )
}

/// Filter NE feature entries from every BasePhrase in `doc`.
///
/// Walks every BP and removes any `NE` feature entry that fails the
/// heuristics. Other features on the BP (`敬語`, BP feature labels,
/// etc.) are untouched. Structural shape (sentence/phrase/BP/morpheme
/// counts) is preserved — only the contents of `bp.features` may
/// shrink.
pub fn apply(mut doc: Document, _lexicon: &dyn Lexicon) -> Document {
    for sentence in &mut doc.sentences {
        for bp in &mut sentence.base_phrases {
            bp.features.retain(|kv| {
                if kv.key != "NE" {
                    return true;
                }
                ne_value_is_real(&kv.value)
            });
        }
    }
    doc
}

/// Decide whether an `NE` value string represents a real named
/// entity. Returns `false` for spans that should be dropped.
///
/// Pulled out as a free function so each heuristic has its own focused
/// unit test.
fn ne_value_is_real(value: &str) -> bool {
    let Some((tag, surface)) = parse_ne_value(value) else {
        return false; // malformed — drop
    };
    if surface.is_empty() {
        return false;
    }
    if surface
        .chars()
        .all(|c| c.is_ascii() && c.is_ascii_punctuation())
    {
        return false;
    }

    match tag {
        // Proper-noun tags: drop pure-hiragana surfaces (real proper
        // nouns are written in kanji/katakana/Latin).
        "PERSON" | "LOCATION" | "ORGANIZATION" => {
            if is_pure_hiragana(surface) {
                return false;
            }
            true
        }
        // ARTIFACT: noisiest tag. Drop both pure-hiragana surfaces AND
        // single-kanji surfaces (almost always common nouns).
        "ARTIFACT" => {
            if is_pure_hiragana(surface) {
                return false;
            }
            if surface.chars().count() == 1
                && surface.chars().next().is_some_and(is_cjk_unified_ideograph)
            {
                return false;
            }
            true
        }
        // Temporal / numeric tags: keep almost everything. These can
        // legitimately be pure hiragana ("きのう"), single kanji ("年"
        // as a duration), etc.
        "DATE" | "TIME" | "MONEY" | "PERCENT" => true,
        // Unknown tag — drop defensively. KWJA's tag set is fixed; an
        // unknown tag means model drift or an upstream change we
        // haven't accounted for.
        _ => false,
    }
}

/// Parse the `"TYPE:SURFACE"` value emitted by KWJA.
///
/// Returns `None` if the value is malformed (no colon, empty TYPE,
/// or any other shape we don't expect). Surface may contain colons
/// — we split on the FIRST colon only.
fn parse_ne_value(value: &str) -> Option<(&str, &str)> {
    let (tag, surface) = value.split_once(':')?;
    if tag.is_empty() {
        return None;
    }
    Some((tag, surface))
}

fn is_pure_hiragana(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_hiragana_char)
}

fn is_hiragana_char(c: char) -> bool {
    // Hiragana block: U+3040..U+309F. We exclude the prolonged sound
    // mark ー (U+30FC, in the Katakana block) and other punctuation.
    matches!(c, '\u{3041}'..='\u{3096}' | '\u{309D}'..='\u{309F}')
}

fn is_cjk_unified_ideograph(c: char) -> bool {
    matches!(
        c,
        '\u{4E00}'..='\u{9FFF}'        // CJK Unified Ideographs
        | '\u{3400}'..='\u{4DBF}'      // Extension A
        | '\u{20000}'..='\u{2A6DF}'    // Extension B
        | '\u{2A700}'..='\u{2EBEF}'    // Extensions C-F
        | '\u{F900}'..='\u{FAFF}'      // Compatibility Ideographs
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{BasePhrase, KeyValue, Sentence};
    use crate::lookup::EmptyLexicon;

    // ── parse_ne_value helpers ─────────────────────────────────────

    #[test]
    fn parse_ne_value_well_formed() {
        assert_eq!(
            parse_ne_value("PERSON:山田太郎"),
            Some(("PERSON", "山田太郎"))
        );
        assert_eq!(parse_ne_value("LOCATION:東京"), Some(("LOCATION", "東京")));
    }

    #[test]
    fn parse_ne_value_rejects_no_colon() {
        assert_eq!(parse_ne_value("PERSON山田"), None);
    }

    #[test]
    fn parse_ne_value_rejects_empty_tag() {
        assert_eq!(parse_ne_value(":山田"), None);
    }

    #[test]
    fn parse_ne_value_allows_colon_in_surface() {
        // Time stamps include colons; we split on the first only.
        assert_eq!(parse_ne_value("TIME:12:30"), Some(("TIME", "12:30")));
    }

    // ── script helpers ─────────────────────────────────────────────

    #[test]
    fn pure_hiragana_detects_correctly() {
        assert!(is_pure_hiragana("やまだ"));
        assert!(is_pure_hiragana("きのう"));
        assert!(!is_pure_hiragana("山田")); // kanji
        assert!(!is_pure_hiragana("ヤマダ")); // katakana
        assert!(!is_pure_hiragana("yamada")); // latin
        assert!(!is_pure_hiragana("やまだ太郎")); // mixed
        assert!(!is_pure_hiragana("")); // empty
    }

    #[test]
    fn cjk_ideograph_detects_correctly() {
        assert!(is_cjk_unified_ideograph('山'));
        assert!(is_cjk_unified_ideograph('田'));
        assert!(!is_cjk_unified_ideograph('や')); // hiragana
        assert!(!is_cjk_unified_ideograph('ヤ')); // katakana
        assert!(!is_cjk_unified_ideograph('a')); // latin
        assert!(!is_cjk_unified_ideograph('1')); // digit
    }

    // ── per-tag filter rules ───────────────────────────────────────

    #[test]
    fn person_real_kanji_name_is_kept() {
        assert!(ne_value_is_real("PERSON:山田太郎"));
    }

    #[test]
    fn person_katakana_name_is_kept() {
        assert!(ne_value_is_real("PERSON:ヤマダ"));
    }

    #[test]
    fn person_pure_hiragana_is_dropped() {
        // Common false positive: a pure-hiragana noun mistakenly
        // tagged as PERSON.
        assert!(!ne_value_is_real("PERSON:やまだ"));
    }

    #[test]
    fn location_pure_hiragana_is_dropped() {
        assert!(!ne_value_is_real("LOCATION:こうえん"));
    }

    #[test]
    fn organization_pure_hiragana_is_dropped() {
        assert!(!ne_value_is_real("ORGANIZATION:かいしゃ"));
    }

    #[test]
    fn artifact_single_kanji_is_dropped() {
        // 本 (book), 車 (car), 家 (house) — all common nouns
        // routinely misclassified as ARTIFACT.
        assert!(!ne_value_is_real("ARTIFACT:本"));
        assert!(!ne_value_is_real("ARTIFACT:車"));
        assert!(!ne_value_is_real("ARTIFACT:家"));
    }

    #[test]
    fn artifact_multi_char_kanji_is_kept() {
        // Real product/work names with multiple kanji should survive.
        assert!(ne_value_is_real("ARTIFACT:源氏物語"));
        assert!(ne_value_is_real("ARTIFACT:鬼滅の刃"));
    }

    #[test]
    fn artifact_pure_hiragana_is_dropped() {
        assert!(!ne_value_is_real("ARTIFACT:しんぶん"));
    }

    #[test]
    fn date_pure_hiragana_is_kept() {
        // "yesterday" / "today" / "tomorrow" are legitimate DATE
        // entities that happen to be pure hiragana.
        assert!(ne_value_is_real("DATE:きのう"));
        assert!(ne_value_is_real("DATE:きょう"));
    }

    #[test]
    fn time_with_colons_is_kept() {
        // Colons in surface (timestamps) parse correctly.
        assert!(ne_value_is_real("TIME:12:30"));
    }

    #[test]
    fn money_with_currency_is_kept() {
        assert!(ne_value_is_real("MONEY:¥1000"));
        assert!(ne_value_is_real("MONEY:1000円"));
    }

    #[test]
    fn percent_is_kept() {
        assert!(ne_value_is_real("PERCENT:50%"));
    }

    #[test]
    fn unknown_tag_is_dropped() {
        // Defensive: model drift or an upstream tag-set change should
        // produce a loud-fail signal (silently keeping unknown tags
        // would let bad data leak downstream).
        assert!(!ne_value_is_real("WEAPON:剣"));
        assert!(!ne_value_is_real("ANIMAL:猫"));
    }

    #[test]
    fn empty_surface_is_dropped() {
        assert!(!ne_value_is_real("PERSON:"));
    }

    #[test]
    fn malformed_value_is_dropped() {
        assert!(!ne_value_is_real("PERSON山田"));
        assert!(!ne_value_is_real(""));
        assert!(!ne_value_is_real(":山田"));
    }

    // ── end-to-end Document tests ──────────────────────────────────

    fn doc_with_ne(ne_value: &str) -> Document {
        Document {
            sentences: vec![Sentence {
                text: "test".into(),
                phrases: vec![],
                base_phrases: vec![BasePhrase {
                    id: 0,
                    surface: "test".into(),
                    head: -1,
                    dep_type: "D".into(),
                    morphemes: vec![],
                    features: vec![KeyValue {
                        key: "NE".into(),
                        value: ne_value.into(),
                    }],
                    relations: vec![],
                }],
                morphemes: vec![],
            }],
            discourse_relations: vec![],
        }
    }

    #[test]
    fn apply_drops_bad_ne_keeps_good_ne() {
        let doc = doc_with_ne("PERSON:やまだ"); // bad
        let out = apply(doc, &EmptyLexicon);
        assert_eq!(out.sentences[0].base_phrases[0].features.len(), 0);

        let doc = doc_with_ne("PERSON:山田"); // good
        let out = apply(doc, &EmptyLexicon);
        assert_eq!(out.sentences[0].base_phrases[0].features.len(), 1);
        assert_eq!(
            out.sentences[0].base_phrases[0].features[0].value,
            "PERSON:山田"
        );
    }

    #[test]
    fn apply_preserves_non_ne_features() {
        let mut doc = doc_with_ne("PERSON:やまだ");
        // Add a non-NE feature alongside the bad NE.
        doc.sentences[0].base_phrases[0].features.push(KeyValue {
            key: "敬語".into(),
            value: "尊敬語".into(),
        });

        let out = apply(doc, &EmptyLexicon);
        // NE dropped, 敬語 preserved.
        let features = &out.sentences[0].base_phrases[0].features;
        assert_eq!(features.len(), 1);
        assert_eq!(features[0].key, "敬語");
    }

    #[test]
    fn apply_handles_multiple_ne_per_bp() {
        // KWJA shouldn't produce multiple NE entries per BP today, but
        // be defensive.
        let mut doc = doc_with_ne("PERSON:山田");
        doc.sentences[0].base_phrases[0].features.push(KeyValue {
            key: "NE".into(),
            value: "PERSON:やまだ".into(), // bad
        });
        doc.sentences[0].base_phrases[0].features.push(KeyValue {
            key: "NE".into(),
            value: "ARTIFACT:鬼滅の刃".into(), // good
        });

        let out = apply(doc, &EmptyLexicon);
        let features = &out.sentences[0].base_phrases[0].features;
        let ne_values: Vec<&str> = features
            .iter()
            .filter(|kv| kv.key == "NE")
            .map(|kv| kv.value.as_str())
            .collect();
        assert_eq!(ne_values, vec!["PERSON:山田", "ARTIFACT:鬼滅の刃"]);
    }

    #[test]
    fn apply_is_identity_on_empty_document() {
        let doc = Document {
            sentences: vec![],
            discourse_relations: vec![],
        };
        let out = apply(doc, &EmptyLexicon);
        assert!(out.sentences.is_empty());
    }
}
