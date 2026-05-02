//! `SplitProperNounWithParticle` — Split a "proper noun" token that
//! Sudachi UniDic registers as a single morpheme but is actually
//! `<word><particle><word>` (e.g., a song / movie / manga title).
//!
//! ## Why
//!
//! UniDic carries some surface-level proper nouns that span over
//! particles — typically song / film / book titles registered in the
//! catalog. The clearest example is `僕は君` (registered as
//! `名詞-固有名詞-一般`), which Sudachi emits as a single token on
//! input `僕は君が大好きです`. Downstream that single span ends up
//! Unresolved (no vocab/proper-noun entry) when the *components* —
//! `僕`, `は`, `君` — would each resolve cleanly.
//!
//! ## Trigger
//!
//! For each Noun morpheme tagged 固有名詞:
//! - Surface ≥ 3 characters.
//! - Surface contains a sentence particle (`は`, `が`, `を`, `に`,
//!   `で`, `と`, `も`, `へ`, `や`) at a non-edge position.
//! - Prefix (chars before the particle) and suffix (chars after) are
//!   each confirmed vocab entries via [`Lexicon::has_compound_entry`].
//!
//! When all three hold, emit `[prefix (Noun), particle (Particle),
//! suffix (Noun)]`. Otherwise leave the token alone — better to keep
//! a real proper noun intact than to over-split.
//!
//! ## Lexicon dependency
//!
//! Without a populated [`Lexicon`], this rule is a no-op (the
//! has_compound_entry guard rejects). Callers using
//! [`EmptyLexicon`](crate::EmptyLexicon) get the original Sudachi
//! tokens back.

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "split_proper_noun_with_particle";

/// Sentence particles that often appear inside fake-compound proper
/// nouns. Single-char only — multi-char particles (e.g., から, まで)
/// would need different handling.
const SPLITTABLE_PARTICLES: &[char] = &[
    'は', 'が', 'を', 'に', 'で', 'と', 'も', 'へ', 'や',
];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Split, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 2);
    for m in morphemes {
        match try_split(&m, lexicon) {
            Some(split) => out.extend(split),
            None => out.push(m),
        }
    }
    out
}

fn try_split(m: &Morpheme, lexicon: &dyn Lexicon) -> Option<Vec<Morpheme>> {
    if !matches!(m.pos, Pos::Noun) {
        return None;
    }
    // Only trigger on tokens Sudachi labelled as 固有名詞 — common
    // nouns containing particle chars are typically real entries
    // (`一日中` etc.) and shouldn't be touched here.
    if !m.part_of_speech.iter().any(|p| p == "固有名詞") {
        return None;
    }
    let chars: Vec<char> = m.surface.chars().collect();
    if chars.len() < 3 {
        return None;
    }
    // Walk interior positions; for each splittable particle, check
    // the surrounding parts as vocab compound entries. First
    // position that produces two confirmed-entry halves wins.
    for (i, &c) in chars.iter().enumerate().skip(1).take(chars.len().saturating_sub(2)) {
        if !SPLITTABLE_PARTICLES.contains(&c) {
            continue;
        }
        let prefix: String = chars[..i].iter().collect();
        let suffix: String = chars[i + 1..].iter().collect();
        if lexicon.has_compound_entry(&prefix) != Some(true) {
            continue;
        }
        if lexicon.has_compound_entry(&suffix) != Some(true) {
            continue;
        }
        // Verified split.
        let begin = m.char_range.start;
        let prefix_end = begin + i;
        let particle_end = prefix_end + 1;
        let total_end = m.char_range.end;

        let mut p_morph = Morpheme::synthesize(
            prefix.clone(),
            "",
            prefix,
            vec!["名詞".into(), "普通名詞".into(), "一般".into()],
            begin..prefix_end,
        );
        p_morph.record_rule(NAME);

        let particle_str = c.to_string();
        let mut particle_morph = Morpheme::synthesize(
            particle_str.clone(),
            "",
            particle_str,
            vec!["助詞".into()],
            prefix_end..particle_end,
        );
        particle_morph.record_rule(NAME);

        let mut s_morph = Morpheme::synthesize(
            suffix.clone(),
            "",
            suffix,
            vec!["名詞".into(), "普通名詞".into(), "一般".into()],
            particle_end..total_end,
        );
        s_morph.record_rule(NAME);

        return Some(vec![p_morph, particle_morph, s_morph]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;
    use std::collections::HashSet;

    fn synth_proper(surface: &str) -> Morpheme {
        let mut m = Morpheme::synthesize(
            surface,
            "",
            surface,
            vec!["名詞".into(), "固有名詞".into(), "一般".into()],
            0..surface.chars().count(),
        );
        m.normalized_form = surface.to_string();
        m.pos = Pos::Noun;
        m
    }

    struct PartialLex(HashSet<&'static str>);
    impl Lexicon for PartialLex {
        fn has_compound_entry(&self, t: &str) -> Option<bool> {
            Some(self.0.contains(t))
        }
    }

    #[test]
    fn splits_boku_wa_kimi_when_both_halves_are_vocab() {
        let lex = PartialLex(HashSet::from(["僕", "君"]));
        let m = synth_proper("僕は君");
        let out = apply(vec![m], &lex);
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].surface, "僕");
        assert_eq!(out[1].surface, "は");
        assert_eq!(out[2].surface, "君");
        assert!(out[0].applied_rules.contains(&NAME));
        assert!(out[2].applied_rules.contains(&NAME));
    }

    #[test]
    fn keeps_intact_when_halves_not_in_vocab() {
        let lex = PartialLex(HashSet::from([]));
        let m = synth_proper("僕は君");
        let out = apply(vec![m], &lex);
        // Without lexicon confirmation we don't speculate.
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "僕は君");
    }

    #[test]
    fn keeps_intact_with_empty_lexicon() {
        let m = synth_proper("僕は君");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn ignores_common_noun() {
        let lex = PartialLex(HashSet::from(["僕", "君"]));
        let mut m = synth_proper("僕は君");
        m.part_of_speech = vec!["名詞".into(), "普通名詞".into(), "一般".into()];
        let out = apply(vec![m], &lex);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn ignores_short_surface() {
        let lex = PartialLex(HashSet::from(["僕", "君"]));
        let m = synth_proper("僕は");
        let out = apply(vec![m], &lex);
        assert_eq!(out.len(), 1);
    }
}
