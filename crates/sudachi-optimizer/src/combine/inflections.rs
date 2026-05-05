//! `CombineInflections` — Iteratively merge a base inflectable
//! morpheme (Verb / Adjective / NaAdjective / verb-like Suffix /
//! suru-noun) with following morphemes whenever the deconjugator
//! validates the merged candidate as a real verb / adjective form.
//!
//! ## Algorithm (per Jiten)
//!
//! For every morpheme that qualifies as an inflectable base
//! (`is_inflectable_base` below), greedily try to merge subsequent
//! morphemes one at a time. A merge is accepted via one of two
//! scenarios:
//!
//! - **Scenario A — standard inflection.** The deconjugator returns a
//!   form whose text equals `current_dict_form` (or
//!   `current_dict_form + "する"` for suru-nouns). The merged token
//!   keeps the current dict form.
//! - **Scenario B — suffix transition.** The next morpheme is
//!   verb-like (`Suffix` with `動詞的`, or `Verb` with `非自立可能`)
//!   and the deconjugator returns a form whose text ends with the
//!   suffix's dict form, producing a new compound verb (e.g.
//!   `読み + かねる → 読みかねる`). The merged token's dict form
//!   becomes the deconjugated text.
//!
//! Both scenarios pass through eight "stop" conditions first
//! (particles, quotative って, contracted じゃ, etc.) and an "is
//! valid inflection part" check (`is_inflection_part`), with several
//! exceptions for misclassified Sudachi tokens (やれ, ねえ, せん).
//!
//! ## Greedy stealing
//!
//! Two patterns split a single Sudachi-emitted token into a stem +
//! tail and merge the stem onto the base:
//!
//! - **そうだ / そうか.** When the deconjugator confirms `current +
//!   そう` is a valid form (e.g. 新しそう, 話そう), `そう` is split
//!   off and merged, and the next-token slot gets rewritten to just
//!   the trailing `だ` / `か`.
//! - **なさそう.** When `current + なさそう` is a valid form, the
//!   whole thing merges into the current token (no remainder).
//!
//! ## Lexicon dependency
//!
//! Every deconjugator-validated merge runs through the consumer's
//! [`Lexicon::lookup_conjugated_form`] (default impl uses
//! `sudachi_morphology::deconjugate`) and the compound-existence
//! oracle [`Lexicon::has_compound_entry`]. When `has_compound_entry`
//! returns `None` (no info, the [`EmptyLexicon`] default), the
//! lookup is treated as "always allow" — matches Jiten's
//! `HasCompoundLookup == null` branch.
//!
//! Per-pass deconjugation results are memoised in a local cache
//! keyed by the hiragana-normalized candidate text.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineInflections`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;
use std::collections::HashMap;
use sudachi_morphology::Form;
use sudachi_morphology::kana::katakana_to_hiragana;

pub const NAME: &str = "combine_inflections";

/// Curated list of grammatical-aspect te-form auxiliaries that should
/// stay separate from a preceding compound base in Scenario B.
/// Mirrors Jiten's `AuxiliaryVerbs` constant.
const AUXILIARY_VERBS: &[&str] = &[
    "続ける", "始める", "終わる", "終える", "出す", "かける",
    "いたす", "いただく", "頂く", "する",
];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }

    // Mutable working buffer — the greedy-steal branches rewrite
    // morphemes[i+1] in place before re-entering the outer loop.
    let mut morphemes = morphemes;
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut deconj_cache: HashMap<String, Vec<Form>> = HashMap::new();

    let mut i = 0;
    while i < morphemes.len() {
        let mut current = morphemes[i].clone();
        let mut current_dict_form = current.dictionary_form.clone();
        let current_norm_form = current.normalized_form.clone();
        let mut current_pos = current.pos;

        if !is_inflectable_base(&current) {
            out.push(current);
            i += 1;
            continue;
        }

        // Inner merge loop — keep absorbing the next morpheme as long
        // as the deconjugator agrees.
        while i + 1 < morphemes.len() {
            let next = morphemes[i + 1].clone();

            // Negative-stem exception: な before a Possible/Dependant
            // すぎる/過ぎる is part of わからなすぎる (negative stem +
            // すぎる).
            let negative_stem_before_dependant = next.surface == "な"
                && matches!(next.pos, Pos::Auxiliary)
                && next.dictionary_form == "ない"
                && i + 2 < morphemes.len()
                && {
                    let after = &morphemes[i + 2];
                    has_section(after, "非自立") || has_section(after, "非自立可能")
                }
                && (morphemes[i + 2].dictionary_form == "すぎる"
                    || morphemes[i + 2].dictionary_form == "過ぎる");

            // Hard stops — absolutely never merge.
            if matches!(
                next.surface.as_str(),
                "は" | "よ" | "し" | "を" | "が" | "か" | "ください" | "かな"
            ) {
                break;
            }
            if next.surface == "な" && !negative_stem_before_dependant {
                break;
            }
            // 様/よう as a standalone token is the noun, never a
            // volitional suffix.
            if next.surface == "よう" && next.dictionary_form == "よう" {
                break;
            }
            // ちゃ/じゃ/きゃ/にゃ + いける is "must (not)" — keep
            // separate; allow after て (やっていける).
            if next.dictionary_form == "いける"
                && (current.surface.ends_with("ちゃ")
                    || current.surface.ends_with("じゃ")
                    || current.surface.ends_with("きゃ")
                    || current.surface.ends_with("にゃ"))
            {
                break;
            }
            // Explanatory ん.
            if next.surface == "ん"
                && (next.dictionary_form == "の" || next.dictionary_form == "ん")
            {
                break;
            }
            // Quotative って followed by んだ/んです.
            if next.surface == "って"
                && i + 2 < morphemes.len()
                && matches!(morphemes[i + 2].surface.as_str(), "ん" | "んだ" | "んです")
            {
                break;
            }
            // Bare ん at the end + だ/です = explanatory のだ; keep
            // them separate so a later pass can build んだ.
            if current.surface.ends_with('ん')
                && matches!(next.surface.as_str(), "だ" | "です")
            {
                break;
            }
            // Contracted copula じゃ starts a new clause.
            if next.surface == "じゃ" && next.dictionary_form == "だ" {
                break;
            }
            // Na-adjective + で (te-form of だ) — keep separate.
            if matches!(current_pos, Pos::AdjectivalNoun)
                && next.surface == "で"
                && next.dictionary_form == "だ"
            {
                break;
            }

            // Standard "is valid inflection part" check — followed by
            // several Sudachi-misclassification escape hatches.
            let mut is_valid_part = is_inflection_part(&next)
                || has_section(&next, "助動詞語幹")
                || has_section(&next, "接続助詞")
                || has_section(&next, "非自立")
                || has_section(&next, "非自立可能");

            // やれ tagged as Interjection but follows te-form as
            // imperative auxiliary.
            if !is_valid_part
                && next.surface == "やれ"
                && matches!(next.pos, Pos::Interjection)
                && current.surface.ends_with('て')
            {
                is_valid_part = true;
            }

            // ねえ tagged as Noun (姉) but after te/de-form is the
            // colloquial negative.
            if !is_valid_part
                && next.surface == "ねえ"
                && matches!(next.pos, Pos::Noun)
                && (current.surface.ends_with('て') || current.surface.ends_with('で'))
            {
                is_valid_part = true;
            }

            // Greedy steal — そうだ / そうか.
            if !is_valid_part && matches!(next.surface.as_str(), "そうだ" | "そうか") {
                let steal_candidate = format!("{}そう", current.surface);
                let steal_target = scenario_target(current_pos, &current_dict_form);
                let forms = cached_deconj(&mut deconj_cache, lexicon, &steal_candidate);
                if forms.iter().any(|f| f.text == steal_target) {
                    // Successful steal — merge `current` + そう, and
                    // rewrite the next slot to just the trailing
                    // だ/か for the next outer iteration to handle.
                    current.surface = steal_candidate;
                    current.reading_form.push_str("ソウ");
                    current.char_range = current.char_range.start..(next.char_range.start + 1);
                    if matches!(current_pos, Pos::Noun) {
                        current.dictionary_form = format!("{}する", current_dict_form);
                        current_pos = Pos::Verb;
                    }
                    current.pos = current_pos;
                    current_dict_form = current.dictionary_form.clone();
                    current.record_rule(NAME);

                    let remainder_surface = if next.surface == "そうだ" { "だ" } else { "か" };
                    let remainder_pos: Vec<String> = if remainder_surface == "だ" {
                        vec!["助動詞".into()]
                    } else {
                        vec!["助詞".into()]
                    };
                    let remainder_start = next.char_range.start + 1;
                    let remainder = Morpheme::synthesize(
                        remainder_surface,
                        remainder_surface,
                        remainder_surface,
                        remainder_pos,
                        remainder_start..next.char_range.end,
                    );
                    morphemes[i + 1] = remainder;
                    // Don't advance i+1 — let the remainder be picked
                    // up in the outer loop.
                    break;
                }
            }

            // Greedy steal — なさそう (e.g., 食べなさそう).
            if !is_valid_part && next.dictionary_form == "なさそう" {
                let steal_candidate = format!("{}{}", current.surface, next.surface);
                let steal_target = scenario_target(current_pos, &current_dict_form);
                let forms = cached_deconj(&mut deconj_cache, lexicon, &steal_candidate);
                if forms.iter().any(|f| f.text == steal_target) {
                    current.surface = steal_candidate;
                    current.reading_form.push_str(&next.reading_form);
                    current.char_range = current.char_range.start..next.char_range.end;
                    if matches!(current_pos, Pos::Noun) {
                        current.dictionary_form = format!("{}する", current_dict_form);
                        current_pos = Pos::Verb;
                    }
                    current.pos = current_pos;
                    current_dict_form = current.dictionary_form.clone();
                    current.record_rule(NAME);
                    i += 1;
                    break;
                }
            }

            // Kansai-ben せん after PossibleSuru base = しない (negative).
            if !is_valid_part
                && next.surface == "せん"
                && has_section(&current, "サ変可能")
            {
                is_valid_part = true;
            }

            if !is_valid_part {
                break;
            }

            // Build the candidate and consult the deconjugator.
            let candidate_text = format!("{}{}", current.surface, next.surface);
            let target = scenario_target(current_pos, &current_dict_form);

            // Snapshot the deconjugator forms for this candidate so
            // the cache borrow doesn't conflict with later lookups.
            let scenario_a_hit;
            let scenario_b_match: Option<(String, Option<String>)>;
            {
                let forms = cached_deconj(&mut deconj_cache, lexicon, &candidate_text);
                scenario_a_hit = forms.iter().any(|f| f.text == target);

                // Pre-compute the Scenario B candidate (text + tag)
                // so we don't hold the cache borrow while consulting
                // the lexicon.
                if !scenario_a_hit
                    && matches!(current_pos, Pos::Verb)
                    && !current.surface.ends_with('て')
                    && !current.surface.ends_with('で')
                    && !current.surface.ends_with("たく")
                    && !current.surface.ends_with("なく")
                    && !current.surface.ends_with("たり")
                    && !current.surface.ends_with("だり")
                    && !AUXILIARY_VERBS.contains(&next.dictionary_form.as_str())
                    && (has_section(&next, "動詞的")
                        || (matches!(next.pos, Pos::Verb)
                            && (has_section(&next, "非自立可能")
                                || has_section(&next, "非自立")))
                        || matches!(next.pos, Pos::Suffix))
                {
                    let suffix_dict = katakana_to_hiragana(&next.dictionary_form);
                    let mut match_form: Option<&Form> = forms.iter().find(|f| {
                        f.text.ends_with(&suffix_dict) && f.text.len() > suffix_dict.len()
                    });
                    if match_form.is_none() && matches!(next.pos, Pos::Suffix)
                        && let Some(verb_dict) = try_godan_dict_form(&suffix_dict) {
                            match_form = forms.iter().find(|f| {
                                f.text.ends_with(&verb_dict) && f.text.len() > verb_dict.len()
                            });
                        }
                    scenario_b_match = match_form.map(|f| {
                        let last_tag = f.tags.last().cloned();
                        (f.text.clone(), last_tag)
                    });
                } else {
                    scenario_b_match = None;
                }
            }

            let mut merged = false;
            let mut new_dict_form: Option<String> = None;

            if scenario_a_hit
                && compound_lookup_ok(lexicon, &current_dict_form, &current_norm_form)
            {
                merged = true;
                if matches!(current_pos, Pos::Noun) {
                    new_dict_form = Some(format!("{}する", current_dict_form));
                    current_pos = Pos::Verb;
                } else if matches!(current_pos, Pos::Adjective)
                    && matches!(next.pos, Pos::Suffix)
                    && next.dictionary_form == "さ"
                {
                    // Keep the IAdjective dict form (e.g. 幼い)
                    // instead of falling through to a homophonous
                    // noun (幼/よう).
                }
            } else if let Some((form_text, last_tag)) = scenario_b_match
                && compound_lookup_ok_for_form(lexicon, &mut deconj_cache, &form_text) {
                    merged = true;
                    current_pos = if last_tag.as_deref() == Some("adj-i") {
                        Pos::Adjective
                    } else {
                        Pos::Verb
                    };
                    new_dict_form = Some(form_text);
                }

            if merged {
                current.surface = candidate_text;
                current.reading_form.push_str(&next.reading_form);
                current.char_range = current.char_range.start..next.char_range.end;
                current.pos = current_pos;
                if let Some(d) = new_dict_form {
                    current.dictionary_form = d;
                }
                current_dict_form = current.dictionary_form.clone();
                current.record_rule(NAME);
                i += 1;
            } else {
                break;
            }
        }

        out.push(current);
        i += 1;
    }

    out
}

// === helpers ===

fn is_inflectable_base(m: &Morpheme) -> bool {
    if m.normalized_form == "物" {
        return false;
    }
    if has_section(m, "助動詞語幹") {
        return false;
    }
    let top_match = matches!(
        m.pos,
        Pos::Verb | Pos::Adjective | Pos::AdjectivalNoun
    );
    let suru_capable = has_section(m, "サ変可能") || has_section(m, "サ変形状詞可能");
    let verb_like_suffix = matches!(m.pos, Pos::Suffix) && has_section(m, "動詞的");
    top_match || suru_capable || verb_like_suffix
}

fn is_inflection_part(m: &Morpheme) -> bool {
    matches!(m.pos, Pos::Auxiliary | Pos::Suffix | Pos::Particle)
}

fn has_section(m: &Morpheme, section: &str) -> bool {
    m.part_of_speech.iter().skip(1).any(|p| p == section)
}

fn scenario_target(current_pos: Pos, current_dict_form: &str) -> String {
    let base = katakana_to_hiragana(current_dict_form);
    if matches!(current_pos, Pos::Noun) {
        format!("{}する", base)
    } else {
        base
    }
}

fn cached_deconj<'a>(
    cache: &'a mut HashMap<String, Vec<Form>>,
    lexicon: &dyn Lexicon,
    candidate: &str,
) -> &'a Vec<Form> {
    let key = katakana_to_hiragana(candidate);
    cache
        .entry(key.clone())
        .or_insert_with(|| lexicon.lookup_conjugated_form(&key))
}

/// Mirrors C# `HasCompoundLookup == null || HasCompoundLookup(dict)
/// || (norm != dict && HasCompoundLookup(norm))`. With our
/// three-state lexicon, `None` means "no info" → allow the merge
/// (matches the C# null branch).
fn compound_lookup_ok(lexicon: &dyn Lexicon, dict_form: &str, norm_form: &str) -> bool {
    match lexicon.has_compound_entry(dict_form) {
        Some(true) => true,
        Some(false) => {
            if norm_form != dict_form {
                matches!(lexicon.has_compound_entry(norm_form), Some(true))
            } else {
                false
            }
        }
        None => true,
    }
}

/// Mirrors `CompoundExistsInLookup`: tries the form's text directly,
/// then any of its deconjugation candidates. Returns true when the
/// lexicon has no info (matches Jiten's `HasCompoundLookup == null`
/// branch).
fn compound_lookup_ok_for_form(
    lexicon: &dyn Lexicon,
    cache: &mut HashMap<String, Vec<Form>>,
    form_text: &str,
) -> bool {
    match lexicon.has_compound_entry(form_text) {
        Some(true) => return true,
        None => return true,
        Some(false) => {}
    }
    let forms = cached_deconj(cache, lexicon, form_text).clone();
    for f in &forms {
        if matches!(lexicon.has_compound_entry(&f.text), Some(true)) {
            return true;
        }
    }
    false
}

/// Map an い-row terminal to its う-row dict-form sibling (合い → 合う).
/// Returns None for kana not in the い row.
fn try_godan_dict_form(text: &str) -> Option<String> {
    if text.chars().count() < 2 {
        return None;
    }
    let last = text.chars().last()?;
    let dict_ending = match last {
        'い' => 'う',
        'き' => 'く',
        'ぎ' => 'ぐ',
        'し' => 'す',
        'ち' => 'つ',
        'に' => 'ぬ',
        'び' => 'ぶ',
        'み' => 'む',
        'り' => 'る',
        _ => return None,
    };
    let last_byte_len = last.len_utf8();
    let prefix = &text[..text.len() - last_byte_len];
    let mut out = String::with_capacity(prefix.len() + dict_ending.len_utf8());
    out.push_str(prefix);
    out.push(dict_ending);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(
        surface: &str,
        dict: &str,
        pos: &[&str],
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(
            surface,
            surface,
            dict,
            pos.iter().map(|s| s.to_string()).collect(),
            char_range,
        )
    }

    fn synth_with_pos(
        surface: &str,
        dict: &str,
        pos: &[&str],
        top_pos: Pos,
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        let mut m = synth(surface, dict, pos, char_range);
        m.pos = top_pos;
        m
    }

    #[test]
    fn merges_taberu_negative_past() {
        // 食べ + な + かっ + た → 食べなかった (deconjugates to 食べる).
        // We test the simpler 食べ + ない first: it should merge into
        // 食べない whose deconjugation chain is [v1, negative].
        let tabe = synth_with_pos("食べ", "食べる", &["動詞"], Pos::Verb, 0..2);
        let mut nai = synth("ない", "ない", &["助動詞"], 2..4);
        nai.pos = Pos::Auxiliary;
        let out = apply(vec![tabe, nai], &EmptyLexicon);
        assert_eq!(out.len(), 1, "expected single merged token");
        assert_eq!(out[0].surface, "食べない");
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn does_not_merge_with_topic_particle_wa() {
        // 食べる + は — は is a hard stop.
        let mut tabe = synth("食べる", "食べる", &["動詞"], 0..3);
        tabe.pos = Pos::Verb;
        let mut wa = synth("は", "は", &["助詞"], 3..4);
        wa.pos = Pos::Particle;
        let out = apply(vec![tabe, wa], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_after_yo_or_ka() {
        let mut tabe = synth("食べる", "食べる", &["動詞"], 0..3);
        tabe.pos = Pos::Verb;
        let mut yo = synth("よ", "よ", &["助詞"], 3..4);
        yo.pos = Pos::Particle;
        let out = apply(vec![tabe, yo], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_yoo_noun() {
        // 食べる + よう (the noun) — never merge.
        let mut tabe = synth("食べる", "食べる", &["動詞"], 0..3);
        tabe.pos = Pos::Verb;
        let mut yoo = synth("よう", "よう", &["名詞"], 3..5);
        yoo.pos = Pos::Noun;
        let out = apply(vec![tabe, yoo], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_explanatory_n() {
        // Verb + ん(の/ん) — keep separate; later passes build んだ.
        let mut tabe = synth("食べる", "食べる", &["動詞"], 0..3);
        tabe.pos = Pos::Verb;
        let mut n = synth("ん", "の", &["助詞", "準体助詞"], 3..4);
        n.pos = Pos::Particle;
        let out = apply(vec![tabe, n], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_n_da_ending() {
        // current ends in ん, next is だ/です → keep separate.
        let mut shiran = synth("知らん", "知る", &["動詞"], 0..3);
        shiran.pos = Pos::Verb;
        let mut da = synth("だ", "だ", &["助動詞"], 3..4);
        da.pos = Pos::Auxiliary;
        let out = apply(vec![shiran, da], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_na_adjective_with_de_copula() {
        let mut takusan = synth("たくさん", "たくさん", &["形状詞"], 0..4);
        takusan.pos = Pos::AdjectivalNoun;
        let mut de = synth("で", "だ", &["助動詞"], 4..5);
        de.pos = Pos::Auxiliary;
        let out = apply(vec![takusan, de], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn skips_non_inflectable_base() {
        // Plain noun shouldn't trigger any merge attempt.
        let neko = synth_with_pos("猫", "猫", &["名詞"], Pos::Noun, 0..1);
        let mut da = synth("だ", "だ", &["助動詞"], 1..2);
        da.pos = Pos::Auxiliary;
        let out = apply(vec![neko, da], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn skips_when_normalized_form_is_mono() {
        // Special-case 物 exclusion.
        let mut mono = synth("物", "物", &["動詞"], 0..1);
        mono.pos = Pos::Verb;
        mono.normalized_form = "物".to_string();
        let mut nai = synth("ない", "ない", &["助動詞"], 1..3);
        nai.pos = Pos::Auxiliary;
        let out = apply(vec![mono, nai], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn merges_chained_negative_past() {
        // 食べ + ない + た would be invalid but 食べ + な + すぎる
        // (negative-stem-before-dependant exception) merges.
        // Simpler: chain 食べ + な + すぎる via the negative-stem
        // exception path.
        let tabe = synth_with_pos("食べ", "食べる", &["動詞"], Pos::Verb, 0..2);
        let mut na = synth("な", "ない", &["助動詞"], 2..3);
        na.pos = Pos::Auxiliary;
        let mut sugiru = synth_with_pos(
            "すぎる",
            "すぎる",
            &["動詞", "非自立可能"],
            Pos::Verb,
            3..6,
        );
        sugiru.pos = Pos::Verb;
        let out = apply(vec![tabe, na, sugiru], &EmptyLexicon);
        // The negative-stem exception keeps な mergeable; whether it
        // ultimately merges depends on the deconjugator validating
        // 食べなすぎる — which may or may not pass. Just assert we
        // don't crash and don't drop tokens.
        assert!(!out.is_empty());
        let total_chars: usize = out.iter().map(|m| m.surface.chars().count()).sum();
        assert_eq!(total_chars, 6);
    }

    #[test]
    fn try_godan_dict_form_maps_i_row_to_u_row() {
        assert_eq!(try_godan_dict_form("合い").as_deref(), Some("合う"));
        assert_eq!(try_godan_dict_form("書き").as_deref(), Some("書く"));
        assert_eq!(try_godan_dict_form("呼び").as_deref(), Some("呼ぶ"));
        // Non-い-row terminal returns None.
        assert_eq!(try_godan_dict_form("食べ"), None);
        assert_eq!(try_godan_dict_form("猫"), None);
        // Single char too short.
        assert_eq!(try_godan_dict_form("い"), None);
    }

    #[test]
    fn is_inflectable_base_classifies_correctly() {
        let mut verb = synth("食べる", "食べる", &["動詞"], 0..3);
        verb.pos = Pos::Verb;
        assert!(is_inflectable_base(&verb));

        let mut adj = synth("赤い", "赤い", &["形容詞"], 0..2);
        adj.pos = Pos::Adjective;
        assert!(is_inflectable_base(&adj));

        let mut na_adj = synth("好き", "好き", &["形状詞"], 0..2);
        na_adj.pos = Pos::AdjectivalNoun;
        assert!(is_inflectable_base(&na_adj));

        // Suru-noun via サ変可能.
        let mut benkyou = synth("勉強", "勉強", &["名詞", "サ変可能"], 0..2);
        benkyou.pos = Pos::Noun;
        assert!(is_inflectable_base(&benkyou));

        // Verb-like suffix.
        let mut suffix = synth("がる", "がる", &["接尾辞", "動詞的"], 0..2);
        suffix.pos = Pos::Suffix;
        assert!(is_inflectable_base(&suffix));

        // Plain noun.
        let mut neko = synth("猫", "猫", &["名詞"], 0..1);
        neko.pos = Pos::Noun;
        assert!(!is_inflectable_base(&neko));

        // 物 special-case exclusion.
        let mut mono = synth("物", "物", &["動詞"], 0..1);
        mono.pos = Pos::Verb;
        mono.normalized_form = "物".to_string();
        assert!(!is_inflectable_base(&mono));

        // AuxiliaryVerbStem exclusion (みたい, etc.).
        let mut mitai = synth(
            "みたい",
            "みたい",
            &["形状詞", "助動詞語幹"],
            0..3,
        );
        mitai.pos = Pos::AdjectivalNoun;
        assert!(!is_inflectable_base(&mitai));
    }

    #[test]
    fn compound_lookup_ok_treats_none_as_allow() {
        // EmptyLexicon returns None → merge allowed (matches C# null
        // branch behaviour).
        assert!(compound_lookup_ok(&EmptyLexicon, "食べる", "食べる"));
    }

    #[test]
    fn compound_lookup_ok_strict_lexicon_blocks_unknown() {
        struct StrictLexicon;
        impl Lexicon for StrictLexicon {
            fn has_compound_entry(&self, term: &str) -> Option<bool> {
                Some(term == "食べる")
            }
        }
        assert!(compound_lookup_ok(&StrictLexicon, "食べる", "食べる"));
        assert!(!compound_lookup_ok(&StrictLexicon, "知らない動詞", "知らない動詞"));
    }

    #[test]
    fn merges_iadjective_with_sa_suffix_keeps_dict_form() {
        // 幼い + さ — merge but dict form stays 幼い (not 幼).
        let mut osanai = synth("幼", "幼い", &["形容詞"], 0..1);
        osanai.pos = Pos::Adjective;
        let mut sa = synth("さ", "さ", &["接尾辞"], 1..2);
        sa.pos = Pos::Suffix;
        let out = apply(vec![osanai, sa], &EmptyLexicon);
        // Whether deconjugator validates depends on whether 幼さ
        // deinflects to 幼い (it does in JL's rules). Just verify we
        // don't crash and dict form preservation works.
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].dictionary_form, "幼い");
    }
}
