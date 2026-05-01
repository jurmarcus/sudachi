//! Label maps and model dimensions ported from KWJA-Python.
//!
//! Source of truth: KWJA upstream `kwja/utils/constants.py`. The label tuples
//! are dumped to `resources/labels.json` (also versioned in git) and loaded
//! at runtime via `serde_json::from_str(include_str!(...))`. This keeps the
//! Rust source compact and lets the same JSON be reused from tests and
//! tooling.
//!
//! KWJA v2.4 dimensions:
//!   pos: 14, subpos: 35, conjtype: 33, conjform: 81
//!   ne: 17, base_phrase_features: 64, dependency_types: 4
//!   sent_segmentation: 2 (B/I), word_segmentation: 2, word_norm_op: 6

use serde::Deserialize;
use std::sync::LazyLock;

const LABELS_JSON: &str = include_str!("../resources/labels.json");

#[derive(Debug, Deserialize)]
struct RawLabels {
    sent_segmentation_tags: Vec<String>,
    word_segmentation_tags: Vec<String>,
    word_norm_op_tags: Vec<String>,
    pos_tags: Vec<String>,
    subpos_tags: Vec<String>,
    conjtype_tags: Vec<String>,
    conjform_tags: Vec<String>,
    ne_tags: Vec<String>,
    base_phrase_features: Vec<String>,
    dependency_types: Vec<String>,
    word_features: Vec<String>,
    cohesion_relations: Vec<String>,
    discourse_relations: Vec<String>,
    ignore_index: i64,
}

pub struct Labels {
    pub sent_segmentation: Vec<String>,
    pub word_segmentation: Vec<String>,
    pub word_norm_op: Vec<String>,
    pub pos: Vec<String>,
    pub subpos: Vec<String>,
    pub conjtype: Vec<String>,
    pub conjform: Vec<String>,
    pub ne: Vec<String>,
    pub base_phrase_features: Vec<String>,
    pub dependency_types: Vec<String>,
    pub word_features: Vec<String>,
    /// 9 KWJA cohesion targets emitted by `cohesion_analyzer`, in classifier
    /// output order: 7 PAS cases + 1 bridging (ノ) + 1 coreference (=).
    pub cohesion_relations: Vec<String>,
    /// 7 discourse relation classes emitted by `discourse_relation_analyzer`.
    /// `談話関係なし` (NULL) is the first slot — argmax falling on it means
    /// "no discourse relation".
    pub discourse_relations: Vec<String>,
    pub ignore_index: i64,
}

impl From<RawLabels> for Labels {
    fn from(r: RawLabels) -> Self {
        Self {
            sent_segmentation: r.sent_segmentation_tags,
            word_segmentation: r.word_segmentation_tags,
            word_norm_op: r.word_norm_op_tags,
            pos: r.pos_tags,
            subpos: r.subpos_tags,
            conjtype: r.conjtype_tags,
            conjform: r.conjform_tags,
            ne: r.ne_tags,
            base_phrase_features: r.base_phrase_features,
            dependency_types: r.dependency_types,
            word_features: r.word_features,
            cohesion_relations: r.cohesion_relations,
            discourse_relations: r.discourse_relations,
            ignore_index: r.ignore_index,
        }
    }
}

pub static LABELS: LazyLock<Labels> = LazyLock::new(|| {
    let raw: RawLabels =
        serde_json::from_str(LABELS_JSON).expect("kwja resources/labels.json is malformed");
    raw.into()
});

// DeBERTa-v2 base config used by KWJA char + word modules.
// These match `ku-nlp/deberta-v2-base-japanese{,-char-wwm}` config.json.
pub const HIDDEN_SIZE: usize = 768;
pub const NUM_ATTENTION_HEADS: usize = 12;
pub const NUM_HIDDEN_LAYERS: usize = 12;
pub const VOCAB_SIZE_WORD: usize = 32000;
pub const VOCAB_SIZE_CHAR: usize = 22012;
pub const MAX_POSITION_EMBEDDINGS: usize = 512;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_load_from_resource_json() {
        // Force lazy init.
        let l = &*LABELS;
        assert_eq!(l.sent_segmentation, &["B", "I"]);
        assert_eq!(l.word_segmentation, &["B", "I"]);
        assert_eq!(l.word_norm_op.len(), 6);
        assert_eq!(l.pos.len(), 14);
        assert_eq!(l.subpos.len(), 35);
        assert_eq!(l.conjtype.len(), 33);
        assert_eq!(l.conjform.len(), 81);
        assert_eq!(l.ne.len(), 17);
        assert_eq!(l.base_phrase_features.len(), 64);
        assert_eq!(l.dependency_types.len(), 4);
        assert_eq!(l.ignore_index, -100);
    }

    #[test]
    fn pos_tags_first_entry_is_special() {
        // KWJA's POS hierarchy starts with "特殊" (special).
        assert_eq!(&LABELS.pos[0], "特殊");
    }
}
