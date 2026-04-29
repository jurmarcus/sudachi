//! [`OptimizerToken`] — the working token type used by every rule.
//!
//! Wraps Sudachi's per-morpheme output (surface, reading, dictionary
//! form, POS) with two additions the optimizer pipeline needs:
//!
//! 1. [`SemanticPos`] — the POS classification re-encoded as an enum
//!    rather than the raw `Vec<String>` Sudachi returns. Lets rules
//!    pattern-match `match token.semantic_pos { Suffix => ... }`
//!    instead of repeatedly inspecting `pos[0] == "接尾辞"` strings.
//!
//! 2. `applied_rules` — names of optimizer stages that touched this
//!    token. The downstream consumer (e.g., jisho-core's
//!    `passage_span_decisions`) audits this trail to understand why
//!    a span ended up the way it did.

use std::ops::Range;

/// A single token flowing through the optimizer pipeline.
///
/// Constructed from Sudachi's `Morpheme` via [`from_morpheme`], or
/// synthesised by Split rules that fabricate new tokens.
#[derive(Debug, Clone)]
pub struct OptimizerToken {
    /// Surface form as it appears in the source text.
    pub surface: String,
    /// Reading in hiragana. Empty string when Sudachi returns no
    /// reading (rare — usually only for symbols and OOV).
    pub reading: String,
    /// Dictionary (lemma) form: `食べ` → `食べる`.
    pub dictionary_form: String,
    /// Normalized form (Sudachi's `normalized_form()`). Same as
    /// dictionary form for most tokens, differs for kana-variant
    /// inputs (`れすとらん` → `レストラン`).
    pub normalized_form: String,
    /// Raw Sudachi POS (length 6 for UniDic). Kept alongside
    /// `semantic_pos` because rules occasionally need the original
    /// fine-grained sub-POS.
    pub pos: Vec<String>,
    /// Cached semantic classification of `pos`. See [`SemanticPos`].
    pub semantic_pos: SemanticPos,
    /// Character-offset range in the source text. For tokens
    /// fabricated by Split rules, this points to the character span
    /// of the resulting fragment.
    pub char_range: Range<usize>,
    /// Names of stages that have touched this token, in order.
    /// Empty for raw Sudachi output; gets pushed to as rules apply.
    pub applied_rules: Vec<&'static str>,
}

impl OptimizerToken {
    /// Build a token directly from a Sudachi `Morpheme`. Generic over
    /// the dictionary handle type (`Morpheme<'_, T>`) so callers can
    /// pass either an `Arc<JapaneseDictionary>` or a borrowed handle.
    pub fn from_morpheme<T: ::sudachi::analysis::stateless_tokenizer::DictionaryAccess>(
        m: &::sudachi::analysis::morpheme::Morpheme<'_, T>,
    ) -> Self {
        let surface: String = m.surface().to_string();
        let pos_slice: &[String] = m.part_of_speech();
        let pos: Vec<String> = pos_slice.iter().cloned().collect();
        let semantic_pos = SemanticPos::from_pos(&pos);
        let begin = m.begin_c();
        let end = m.end_c();
        Self {
            surface,
            reading: m.reading_form().to_string(),
            dictionary_form: m.dictionary_form().to_string(),
            normalized_form: m.normalized_form().to_string(),
            pos,
            semantic_pos,
            char_range: begin..end,
            applied_rules: Vec::new(),
        }
    }

    /// Convenience for rules that fabricate tokens.
    pub fn synthesize(
        surface: impl Into<String>,
        reading: impl Into<String>,
        dictionary_form: impl Into<String>,
        pos: Vec<String>,
        char_range: Range<usize>,
    ) -> Self {
        let surface = surface.into();
        let reading = reading.into();
        let dictionary_form = dictionary_form.into();
        let normalized_form = dictionary_form.clone();
        let semantic_pos = SemanticPos::from_pos(&pos);
        Self {
            surface,
            reading,
            dictionary_form,
            normalized_form,
            pos,
            semantic_pos,
            char_range,
            applied_rules: Vec::new(),
        }
    }

    /// Push a rule name into `applied_rules`. Idempotent — if the same
    /// rule fires twice on the same token (rare, but happens in
    /// re-scan loops) we only record it once.
    pub fn record_rule(&mut self, name: &'static str) {
        if !self.applied_rules.contains(&name) {
            self.applied_rules.push(name);
        }
    }
}

/// High-level POS classification. Computed once from raw Sudachi POS
/// strings so downstream rules can pattern-match on a closed enum
/// rather than parse `pos[0]` repeatedly.
///
/// Rules wanting fine-grained classification (`pos[1..]`) still read
/// from [`OptimizerToken::pos`] — this enum collapses only the
/// top-level distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticPos {
    /// 名詞 — noun.
    Noun,
    /// 動詞 — verb.
    Verb,
    /// 形容詞 — i-adjective.
    Adjective,
    /// 形状詞 — na-adjective stem (Sudachi UniDic name).
    AdjectivalNoun,
    /// 副詞 — adverb.
    Adverb,
    /// 連体詞 — pre-noun adjectival (この, その, …).
    Adnominal,
    /// 接続詞 — conjunction.
    Conjunction,
    /// 感動詞 — interjection.
    Interjection,
    /// 助動詞 — auxiliary verb (た, ます, れる, …).
    Auxiliary,
    /// 助詞 — particle.
    Particle,
    /// 接頭辞 — prefix (お, ご, …).
    Prefix,
    /// 接尾辞 — suffix (たち, さん, …).
    Suffix,
    /// 代名詞 — pronoun (彼, 私, …).
    Pronoun,
    /// 記号 / 補助記号 — punctuation, markup.
    Symbol,
    /// 空白 — whitespace.
    Whitespace,
    /// Anything else (URLs, foreign words, etc.).
    Other,
}

impl SemanticPos {
    /// Classify the top-level Sudachi UniDic POS string into a
    /// semantic enum.
    pub fn from_pos(pos: &[String]) -> Self {
        let Some(top) = pos.first() else {
            return SemanticPos::Other;
        };
        match top.as_str() {
            "名詞" => SemanticPos::Noun,
            "動詞" => SemanticPos::Verb,
            "形容詞" => SemanticPos::Adjective,
            "形状詞" => SemanticPos::AdjectivalNoun,
            "副詞" => SemanticPos::Adverb,
            "連体詞" => SemanticPos::Adnominal,
            "接続詞" => SemanticPos::Conjunction,
            "感動詞" => SemanticPos::Interjection,
            "助動詞" => SemanticPos::Auxiliary,
            "助詞" => SemanticPos::Particle,
            "接頭辞" => SemanticPos::Prefix,
            "接尾辞" => SemanticPos::Suffix,
            "代名詞" => SemanticPos::Pronoun,
            "記号" | "補助記号" => SemanticPos::Symbol,
            "空白" => SemanticPos::Whitespace,
            _ => SemanticPos::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_pos_classifies_top_level() {
        assert_eq!(
            SemanticPos::from_pos(&["名詞".into(), "普通名詞".into()]),
            SemanticPos::Noun
        );
        assert_eq!(
            SemanticPos::from_pos(&["助動詞".into()]),
            SemanticPos::Auxiliary
        );
        assert_eq!(
            SemanticPos::from_pos(&["接頭辞".into()]),
            SemanticPos::Prefix
        );
        assert_eq!(SemanticPos::from_pos(&[]), SemanticPos::Other);
    }

    #[test]
    fn record_rule_is_idempotent() {
        let mut t = OptimizerToken::synthesize("猫", "ねこ", "猫", vec!["名詞".into()], 0..1);
        t.record_rule("test");
        t.record_rule("test");
        assert_eq!(t.applied_rules, vec!["test"]);
    }
}
