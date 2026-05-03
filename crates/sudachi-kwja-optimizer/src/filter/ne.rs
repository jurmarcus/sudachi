//! `FilterNe` — drop spurious NE feature entries from BasePhrases.
//!
//! KWJA emits NE annotations as a `KeyValue { key: "NE", value:
//! "{TYPE}:{surface}" }` on the BasePhrase that contains the entity
//! head (see `sudachi-kwja/src/pipeline.rs:1018-1043`). The model
//! over-tags in known ways:
//!
//! - Single-character spans tend to be false positives
//! - All-hiragana spans are usually noise (real PERSON / LOCATION /
//!   ORGANIZATION names are written in kanji / katakana)
//! - A few common surfaces get systematically misclassified
//!
//! This stage walks every BP and drops NE feature entries matching
//! these patterns. Other features on the BP are untouched.
//!
//! ## Why this is layer (2) and not layer (3)
//!
//! The filtering decisions are mechanical surface heuristics — no
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
/// TODO: real filtering logic lands in the next commit. For now this
/// is the identity transform — it just establishes the entry point
/// so the canonical pipeline compiles and the gating works.
pub fn apply(doc: Document, _lexicon: &dyn Lexicon) -> Document {
    doc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{BasePhrase, KeyValue, Sentence};
    use crate::lookup::EmptyLexicon;

    #[test]
    fn identity_on_empty_document() {
        let doc = Document {
            sentences: vec![],
            discourse_relations: vec![],
        };
        let out = apply(doc, &EmptyLexicon);
        assert!(out.sentences.is_empty());
    }

    #[test]
    fn identity_preserves_ne_feature_in_stub() {
        // Until real filtering lands, the stub passes NE features
        // through untouched. This test will be flipped when the
        // real implementation lands so a regression here is loud.
        let doc = Document {
            sentences: vec![Sentence {
                text: "山田".into(),
                phrases: vec![],
                base_phrases: vec![BasePhrase {
                    id: 0,
                    surface: "山田".into(),
                    head: -1,
                    dep_type: "D".into(),
                    morphemes: vec![],
                    features: vec![KeyValue {
                        key: "NE".into(),
                        value: "PERSON:山田".into(),
                    }],
                    relations: vec![],
                }],
                morphemes: vec![],
            }],
            discourse_relations: vec![],
        };
        let out = apply(doc, &EmptyLexicon);
        assert_eq!(out.sentences[0].base_phrases[0].features.len(), 1);
    }
}
