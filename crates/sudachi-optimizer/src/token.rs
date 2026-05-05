//! [`Morpheme`] — owned, optimizer-friendly mirror of Sudachi's
//! borrowed [`sudachi::Morpheme<'_, T>`](crate::sudachi::Morpheme).
//!
//! Field names match Sudachi's [`Morpheme`] method names (`surface`,
//! `reading_form`, `dictionary_form`, `normalized_form`,
//! `part_of_speech`) so a Sudachi user looking at this struct can
//! pattern-match what they already know. Two fields are this crate's
//! own additions:
//!
//! 1. [`pos`](Morpheme::pos) — Sudachi's `part_of_speech` re-encoded
//!    as a closed [`Pos`] enum, computed once from the raw POS strings.
//!    Lets stages pattern-match `match m.pos { Pos::Suffix => … }`
//!    instead of repeatedly inspecting `part_of_speech[0] == "接尾辞"`.
//!
//! 2. [`applied_rules`](Morpheme::applied_rules) — names of optimizer
//!    stages that touched this morpheme. The downstream consumer
//!    (e.g., jisho-core's `passage_span_decisions`) audits this trail
//!    to understand why a span ended up the way it did.

use std::ops::Range;

/// Owned morpheme flowing through the optimizer pipeline.
///
/// Constructed from a Sudachi [`sudachi::Morpheme`](crate::sudachi::Morpheme)
/// via [`Morpheme::from_sudachi`], or synthesised by Split phase
/// stages that fabricate new morphemes via [`Morpheme::synthesize`].
#[derive(Debug, Clone)]
pub struct Morpheme {
    /// Surface form as it appears in the source text.
    /// Mirrors [`sudachi::Morpheme::surface`](crate::sudachi::Morpheme).
    pub surface: String,
    /// Hiragana reading form. Empty when Sudachi returns no reading
    /// (rare — usually only for symbols and OOV input).
    /// Mirrors [`sudachi::Morpheme::reading_form`](crate::sudachi::Morpheme).
    pub reading_form: String,
    /// Dictionary (lemma) form: `食べ` → `食べる`.
    /// Mirrors [`sudachi::Morpheme::dictionary_form`](crate::sudachi::Morpheme).
    ///
    /// **Invariant for downstream consumers**: after the full pipeline,
    /// `dictionary_form` is the canonical lemma — even for merged
    /// morphemes. `飲ん + だ → 飲んだ` keeps `dictionary_form = 飲む`
    /// because [`combine`](crate::combine) and
    /// [`repair`](crate::repair) merge stages clone the head morpheme
    /// (which carries the lemma) and only update `surface`,
    /// `reading_form`, `normalized_form`, and `char_range` to the
    /// merged values. **Use this field for vocab/dictionary lookup.**
    pub dictionary_form: String,
    /// Normalized form. For raw Sudachi morphemes this is
    /// `dictionary_form` with kana-variant folding (`れすとらん` →
    /// `レストラン`). For morphemes that have been **merged** by
    /// [`combine`](crate::combine) / [`repair`](crate::repair) stages,
    /// this is rewritten to the **merged surface** (`飲ん + だ → 飲んだ`
    /// gives `normalized_form = 飲んだ`). Treat as a "canonical surface"
    /// — useful for display or surface-level dedupe, **not** for
    /// vocab lookup. Use [`dictionary_form`](Self::dictionary_form) for
    /// that.
    /// Mirrors [`sudachi::Morpheme::normalized_form`](crate::sudachi::Morpheme).
    pub normalized_form: String,
    /// Reading of the dictionary (lemma) form, in hiragana. For an
    /// inflected morpheme like `食べ` (surface) / `食べる` (dict form),
    /// `reading_form` is `たべ` and `dictionary_form_reading` is
    /// `たべる`. Looked up via Sudachi's `dictionary_form_word_id` in
    /// [`Morpheme::from_sudachi`]; defaults to `dictionary_form` when
    /// synthesised by Split stages (no Sudachi WordId available).
    pub dictionary_form_reading: String,
    /// Raw Sudachi POS (length 6 for UniDic). Kept alongside [`pos`]
    /// because some stages need the original fine-grained sub-POS.
    /// Mirrors [`sudachi::Morpheme::part_of_speech`](crate::sudachi::Morpheme).
    pub part_of_speech: Vec<String>,
    /// Cached semantic classification of [`part_of_speech`]. Optimizer
    /// addition — see [`Pos`].
    pub pos: Pos,
    /// Character-offset range in the source text. Equivalent to
    /// `begin_c()..end_c()` on the underlying Sudachi morpheme. For
    /// morphemes fabricated by Split stages, this points to the
    /// character span of the resulting fragment.
    pub char_range: Range<usize>,
    /// Names of stages that have touched this morpheme, in order.
    /// Empty for raw Sudachi output; gets pushed to as stages apply.
    /// Optimizer addition.
    pub applied_rules: Vec<&'static str>,
}

impl Morpheme {
    /// Build an owned [`Morpheme`] from a borrowed Sudachi morpheme.
    /// Generic over Sudachi's `DictionaryAccess` so callers can pass
    /// any morpheme regardless of how the dictionary is held.
    ///
    /// `lexicon` is the Sudachi lexicon — usually obtained via
    /// `morpheme_list.dict().lexicon()`. It's needed to look up the
    /// dictionary form's reading via the `dictionary_form_word_id`
    /// stored on each morpheme. Passed in (rather than reached via
    /// `m.dict()`) because `Morpheme` itself doesn't expose the
    /// parent dictionary.
    pub fn from_sudachi<T: ::sudachi::analysis::stateless_tokenizer::DictionaryAccess>(
        m: &::sudachi::analysis::morpheme::Morpheme<'_, T>,
        lexicon: &::sudachi::dic::lexicon_set::LexiconSet<'_>,
    ) -> Self {
        use ::sudachi::dic::subset::InfoSubset;
        use ::sudachi::dic::word_id::WordId;

        let surface: String = m.surface().to_string();
        let part_of_speech: Vec<String> = m.part_of_speech().to_vec();
        let pos = Pos::from_part_of_speech(&part_of_speech);
        let begin = m.begin_c();
        let end = m.end_c();
        let reading_form = m.reading_form().to_string();

        // Lemma reading: when this morpheme IS the dictionary form
        // (`dfwi == -1`), reading_form already is the lemma reading.
        // Otherwise, look it up via the dictionary lexicon.
        let dictionary_form_reading = {
            let word_info = m.get_word_info();
            let dfwi = word_info.dictionary_form_word_id();
            if dfwi < 0 {
                reading_form.clone()
            } else {
                let dict_wid = WordId::from_raw(dfwi as u32);
                lexicon
                    .get_word_info_subset(dict_wid, InfoSubset::READING_FORM)
                    .map(|info| info.reading_form().to_string())
                    .unwrap_or_default()
            }
        };

        Self {
            surface,
            reading_form,
            dictionary_form: m.dictionary_form().to_string(),
            normalized_form: m.normalized_form().to_string(),
            dictionary_form_reading,
            part_of_speech,
            pos,
            char_range: begin..end,
            applied_rules: Vec::new(),
        }
    }

    /// Convenience for stages that fabricate morphemes (Split rules).
    /// Computes [`pos`] from the supplied `part_of_speech`. Defaults
    /// `dictionary_form_reading` to `dictionary_form` (no Sudachi
    /// WordId is available for fabricated morphemes).
    pub fn synthesize(
        surface: impl Into<String>,
        reading_form: impl Into<String>,
        dictionary_form: impl Into<String>,
        part_of_speech: Vec<String>,
        char_range: Range<usize>,
    ) -> Self {
        let surface = surface.into();
        let reading_form = reading_form.into();
        let dictionary_form = dictionary_form.into();
        let normalized_form = dictionary_form.clone();
        let dictionary_form_reading = dictionary_form.clone();
        let pos = Pos::from_part_of_speech(&part_of_speech);
        Self {
            surface,
            reading_form,
            dictionary_form,
            normalized_form,
            dictionary_form_reading,
            part_of_speech,
            pos,
            char_range,
            applied_rules: Vec::new(),
        }
    }

    /// Push a stage name into [`applied_rules`]. Idempotent — if the
    /// same stage fires twice on the same morpheme (rare, but happens
    /// in re-scan loops) we only record it once.
    pub fn record_rule(&mut self, name: &'static str) {
        if !self.applied_rules.contains(&name) {
            self.applied_rules.push(name);
        }
    }
}

/// Semantic top-level part-of-speech classification.
///
/// Computed once from raw Sudachi POS strings so downstream stages
/// can pattern-match on a closed enum rather than parse
/// `part_of_speech[0]` repeatedly. Rules wanting fine-grained
/// classification still read [`Morpheme::part_of_speech`] for
/// `pos[1..]` — this enum collapses only the top-level distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pos {
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
    /// 助数詞 — counter suffix (匹, 本, 個, …). Sudachi tags these
    /// as Suffix with a 助数詞 sub-POS; some optimizer stages
    /// re-classify into this distinct variant for clarity.
    Counter,
    /// 記号 / 補助記号 — punctuation, markup.
    Symbol,
    /// 空白 — whitespace.
    Whitespace,
    /// Anything else (URLs, foreign words, etc.).
    Other,
}

impl Pos {
    /// Classify the top-level Sudachi UniDic POS string into a
    /// semantic enum.
    pub fn from_part_of_speech(part_of_speech: &[String]) -> Self {
        let Some(top) = part_of_speech.first() else {
            return Pos::Other;
        };
        match top.as_str() {
            "名詞" => Pos::Noun,
            "動詞" => Pos::Verb,
            "形容詞" => Pos::Adjective,
            "形状詞" => Pos::AdjectivalNoun,
            "副詞" => Pos::Adverb,
            "連体詞" => Pos::Adnominal,
            "接続詞" => Pos::Conjunction,
            "感動詞" => Pos::Interjection,
            "助動詞" => Pos::Auxiliary,
            "助詞" => Pos::Particle,
            "接頭辞" => Pos::Prefix,
            "接尾辞" => Pos::Suffix,
            "代名詞" => Pos::Pronoun,
            "記号" | "補助記号" => Pos::Symbol,
            "空白" => Pos::Whitespace,
            _ => Pos::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_classifies_top_level() {
        assert_eq!(
            Pos::from_part_of_speech(&["名詞".into(), "普通名詞".into()]),
            Pos::Noun
        );
        assert_eq!(Pos::from_part_of_speech(&["助動詞".into()]), Pos::Auxiliary);
        assert_eq!(Pos::from_part_of_speech(&["接頭辞".into()]), Pos::Prefix);
        assert_eq!(Pos::from_part_of_speech(&[]), Pos::Other);
    }

    #[test]
    fn record_rule_is_idempotent() {
        let mut m = Morpheme::synthesize("猫", "ねこ", "猫", vec!["名詞".into()], 0..1);
        m.record_rule("test");
        m.record_rule("test");
        assert_eq!(m.applied_rules, vec!["test"]);
    }
}
