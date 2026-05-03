//! [`DocumentFeatures`] — bitflag set tracking which KWJA-emitted
//! signals are present in the current [`Document`].
//!
//! Mirrors `sudachi_optimizer::token_features::MorphemeFeatures` in
//! purpose. The pipeline runner scans the document once, computes
//! which features are present, then skips any stage whose
//! `required_features` aren't in the set. This avoids walking the
//! tree to no-op for stages whose triggers are absent.
//!
//! Add a new flag whenever a new stage needs gating on the presence
//! of a particular signal type (e.g. NE annotations, dep arcs, BP
//! features matching some predicate).

use crate::document::Document;
use bitflags::bitflags;

bitflags! {
    /// Which KWJA signals are present in a Document.
    ///
    /// Bits are union semantics: `HAS_NE_FEATURES` means *some*
    /// BasePhrase in the document has an `NE` feature key, not that
    /// every BP does. Stages that need to inspect every BP iterate
    /// regardless; stages that can short-circuit when a signal is
    /// entirely absent gate on the relevant flag.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DocumentFeatures: u32 {
        /// At least one BasePhrase has an `NE` key in `features`.
        const HAS_NE_FEATURES = 1 << 0;
        /// At least one BasePhrase has a non-empty `features` list
        /// (NE, 敬語, or any other key).
        const HAS_BP_FEATURES = 1 << 1;
        /// At least one Phrase or BasePhrase has a real dependency
        /// arc (`head >= 0`).
        const HAS_DEP_ARCS = 1 << 2;
        /// At least one Morpheme has a non-empty `features` list
        /// (KWJA word_feature_tagger output: 基本句-主辞 etc.).
        const HAS_MORPHEME_FEATURES = 1 << 3;
        /// At least one BasePhrase has a non-empty `relations` list
        /// (cohesion / PAS output).
        const HAS_RELATIONS = 1 << 4;
        /// At least one DiscourseRelation in the document.
        const HAS_DISCOURSE = 1 << 5;
    }
}

impl DocumentFeatures {
    /// Walk the document once and return the union of all
    /// per-element features observed. Cheap — single linear pass.
    pub fn scan(doc: &Document) -> Self {
        let mut f = Self::empty();

        if !doc.discourse_relations.is_empty() {
            f |= Self::HAS_DISCOURSE;
        }

        for sentence in &doc.sentences {
            for bp in &sentence.base_phrases {
                if !bp.features.is_empty() {
                    f |= Self::HAS_BP_FEATURES;
                    if bp.features.iter().any(|kv| kv.key == "NE") {
                        f |= Self::HAS_NE_FEATURES;
                    }
                }
                if !bp.relations.is_empty() {
                    f |= Self::HAS_RELATIONS;
                }
                if bp.head >= 0 {
                    f |= Self::HAS_DEP_ARCS;
                }
                for m in &bp.morphemes {
                    if !m.features.is_empty() {
                        f |= Self::HAS_MORPHEME_FEATURES;
                    }
                }
            }
            for phrase in &sentence.phrases {
                if phrase.head >= 0 {
                    f |= Self::HAS_DEP_ARCS;
                }
            }
        }

        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{BasePhrase, KeyValue, Morpheme, Phrase, Sentence};

    fn empty_doc() -> Document {
        Document {
            sentences: vec![],
            discourse_relations: vec![],
        }
    }

    fn mk_morph(surface: &str) -> Morpheme {
        Morpheme {
            surface: surface.into(),
            reading: surface.into(),
            lemma: surface.into(),
            pos: String::new(),
            subpos: String::new(),
            conjtype: String::new(),
            conjform: String::new(),
            semantics: vec![],
            features: vec![],
        }
    }

    fn mk_bp_with_ne() -> BasePhrase {
        BasePhrase {
            id: 0,
            surface: "山田".into(),
            head: -1,
            dep_type: "D".into(),
            morphemes: vec![mk_morph("山田")],
            features: vec![KeyValue {
                key: "NE".into(),
                value: "PERSON:山田".into(),
            }],
            relations: vec![],
        }
    }

    fn mk_phrase(head: i32) -> Phrase {
        Phrase {
            id: 0,
            surface: "test".into(),
            head,
            dep_type: "D".into(),
            base_phrases: vec![],
            morphemes: vec![],
        }
    }

    #[test]
    fn empty_document_has_no_features() {
        let f = DocumentFeatures::scan(&empty_doc());
        assert!(f.is_empty());
    }

    #[test]
    fn ne_feature_is_detected() {
        let doc = Document {
            sentences: vec![Sentence {
                text: "山田".into(),
                phrases: vec![],
                base_phrases: vec![mk_bp_with_ne()],
                morphemes: vec![mk_morph("山田")],
            }],
            discourse_relations: vec![],
        };
        let f = DocumentFeatures::scan(&doc);
        assert!(f.contains(DocumentFeatures::HAS_NE_FEATURES));
        assert!(f.contains(DocumentFeatures::HAS_BP_FEATURES));
    }

    #[test]
    fn dep_arc_detected_via_phrase_head() {
        let doc = Document {
            sentences: vec![Sentence {
                text: "x".into(),
                phrases: vec![mk_phrase(2)],
                base_phrases: vec![],
                morphemes: vec![],
            }],
            discourse_relations: vec![],
        };
        assert!(DocumentFeatures::scan(&doc).contains(DocumentFeatures::HAS_DEP_ARCS));
    }

    #[test]
    fn root_only_phrase_has_no_dep_arcs() {
        // head = -1 means "root of the tree", which is not a real arc.
        let doc = Document {
            sentences: vec![Sentence {
                text: "x".into(),
                phrases: vec![mk_phrase(-1)],
                base_phrases: vec![],
                morphemes: vec![],
            }],
            discourse_relations: vec![],
        };
        assert!(!DocumentFeatures::scan(&doc).contains(DocumentFeatures::HAS_DEP_ARCS));
    }
}
