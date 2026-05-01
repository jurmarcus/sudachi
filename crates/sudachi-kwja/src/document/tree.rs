//! Document tree mirroring `proto/parse.proto`'s `ParseTree`.
//!
//! Three nested phrase layers per `Sentence`:
//!   Sentence → Phrase (bunsetsu) → BasePhrase (kihon-ku) → Morpheme.
//! Both `Phrase` and `BasePhrase` carry their own dependency edges; both
//! redundantly carry their flat morpheme list (matches Python emission).
//!
//! `BTreeMap` (not `HashMap`) for `features` and `semantics` so JSON/proto
//! serialization order is deterministic — required for byte-equal e2e
//! equivalence at Task 22.

use serde::{Deserialize, Serialize};

/// Semantic / feature tag — KWJA-Python emits these as `{"key": ..., "value": ...}`
/// objects in a list (not a JSON object), and the production JSONB stored
/// in `passage_parse_tree.tree` matches that shape. Using a tuple struct
/// preserves insertion order (which KWJA's emit relies on for stable
/// JSON byte equality).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub sentences: Vec<Sentence>,
    /// Cross-sentence discourse relations, predicted by KWJA's
    /// `discourse_relation_analyzer`. v1 limitation: this is empty when
    /// `parse_morphemes` is called per-sentence (we split at boundaries
    /// before forward, so the model never sees cross-sentence context).
    /// Populated when callers send multi-sentence input as one batch row.
    #[serde(default)]
    pub discourse_relations: Vec<DiscourseRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscourseRelation {
    pub from_sentence: u32,
    pub to_sentence: u32,
    pub from_base_phrase: u32,
    pub to_base_phrase: u32,
    /// One of `crate::constants::LABELS.discourse_relations`. Never
    /// emitted when argmax falls on `談話関係なし` (the NULL slot).
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sentence {
    pub text: String,
    pub phrases: Vec<Phrase>,
    pub base_phrases: Vec<BasePhrase>,
    pub morphemes: Vec<Morpheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Phrase {
    pub id: u32,
    pub surface: String,
    /// Parent phrase id; -1 if root.
    pub head: i32,
    /// "D" | "P" | "A" | "I" — KWJA dependency edge type.
    pub dep_type: String,
    pub base_phrases: Vec<BasePhrase>,
    pub morphemes: Vec<Morpheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasePhrase {
    pub id: u32,
    pub surface: String,
    pub head: i32,
    pub dep_type: String,
    pub morphemes: Vec<Morpheme>,
    /// KWJA feature tags. Serialized as `[{"key": ..., "value": ...}, ...]`
    /// to match production JSONB exactly.
    pub features: Vec<KeyValue>,
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Morpheme {
    pub surface: String,
    pub reading: String,
    pub lemma: String,
    pub pos: String,
    pub subpos: String,
    pub conjtype: String,
    pub conjform: String,
    /// Semantic tags. Serialized as `[{"key": "代表表記", "value": "今日/きょう"}, ...]`
    /// list-of-objects to match production JSONB.
    pub semantics: Vec<KeyValue>,
    /// KWJA word_feature_tagger output. Multi-label per morpheme; threshold 0.5.
    /// Possible values: 基本句-主辞, 基本句-区切, 文節-区切, 用言表記先頭, 用言表記末尾.
    #[serde(default)]
    pub features: Vec<String>,
}

/// Predicate-argument relation. KWJA's cohesion analyzer emits these from
/// the BasePhrase relation head; jisho's pipeline currently skips the
/// cohesion module, so this Vec is empty in practice — but the field is
/// part of the proto contract and must be present (as `[]`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relation {
    /// Relation type (e.g. "ガ", "ヲ", "ニ" — case roles). `r#type` matches
    /// proto field name.
    pub r#type: String,
    pub target: String,
    pub sid: String,
    pub id: String,
}

/// Mirrors `proto.ParseItem` — either a parsed tree or a per-item error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParseItem {
    Tree(Document),
    Error { kind: String, message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_morph(surface: &str, pos: &str, semantics: Vec<KeyValue>) -> Morpheme {
        Morpheme {
            surface: surface.into(),
            reading: surface.into(),
            lemma: surface.into(),
            pos: pos.into(),
            subpos: String::new(),
            conjtype: String::new(),
            conjform: String::new(),
            semantics,
            features: vec![],
        }
    }

    #[test]
    fn build_document_with_one_sentence() {
        let morphs = vec![
            mk_morph("今日", "名詞", vec![]),
            mk_morph("は", "助詞", vec![]),
        ];
        let bp = BasePhrase {
            id: 0,
            surface: "今日は".into(),
            head: -1,
            dep_type: "D".into(),
            morphemes: morphs.clone(),
            features: vec![],
            relations: vec![],
        };
        let phrase = Phrase {
            id: 0,
            surface: "今日は".into(),
            head: -1,
            dep_type: "D".into(),
            base_phrases: vec![bp.clone()],
            morphemes: morphs.clone(),
        };
        let doc = Document {
            sentences: vec![Sentence {
                text: "今日は".into(),
                phrases: vec![phrase],
                base_phrases: vec![bp],
                morphemes: morphs,
            }],
            discourse_relations: vec![],
        };
        assert_eq!(doc.sentences.len(), 1);
        assert_eq!(doc.sentences[0].morphemes.len(), 2);
        assert_eq!(doc.sentences[0].phrases.len(), 1);
        assert_eq!(doc.sentences[0].phrases[0].base_phrases.len(), 1);
    }

    #[test]
    fn semantics_serialize_as_list_of_objects() {
        // Production JSONB shape: morpheme.semantics = [{"key": ..., "value": ...}, ...]
        let m = mk_morph("夜", "名詞", vec![
            KeyValue { key: "代表表記".into(), value: "夜/よる".into() },
            KeyValue { key: "カテゴリ".into(), value: "時間".into() },
        ]);
        let json = serde_json::to_value(&m).unwrap();
        assert!(json["semantics"].is_array(), "semantics must be a JSON array, got {json}");
        let arr = json["semantics"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["key"], "代表表記");
        assert_eq!(arr[0]["value"], "夜/よる");
        assert_eq!(arr[1]["key"], "カテゴリ");
    }

    #[test]
    fn parse_item_serializes_as_tree_or_error() {
        let tree = ParseItem::Tree(Document { sentences: vec![], discourse_relations: vec![] });
        let err = ParseItem::Error {
            kind: "too_long".into(),
            message: "max 512".into(),
        };
        let _ = serde_json::to_string(&tree).unwrap();
        let _ = serde_json::to_string(&err).unwrap();
    }
}
