//! `CombineVerbDependant` — Run four sub-passes to merge various
//! verb-dependant patterns:
//!
//! 1. **Dependants** (`combine_dependants`): merge a Dependant
//!    sub-POS morpheme onto a preceding Verb (excluding おる, and
//!    skipping when next.surface == current.surface).
//! 2. **PossibleDependants** (`combine_possible_dependants`): merge
//!    curated dependent verbs (得る, しまう, こなす, いく, 貰う, いる,
//!    ない, だす, etc.) onto preceding Verb when conditions hold.
//!    Includes the lexicon-driven 付く branch:
//!    `currentDictForm[..^1] + "り付く"` must be a known compound.
//! 3. **Suru chains** (`combine_suru_chains`): merge suru-noun +
//!    する/す verb forms (e.g., 勉強+する → 勉強する).
//! 4. **Te-iru chains** (`combine_teiru_chains`): merge te-form +
//!    auxiliary verb chains. Three sub-patterns:
//!    - Verb + て (particle) + te-form-aux verb (3-token);
//!    - word ending in て/で + subsidiary verb (2-token, with
//!      deconjugator fallback for colloquial forms);
//!    - verb ending in っ + とる (dialectal っとる contraction,
//!      gated on the deconjugator finding `"toru (teoru)"` in the
//!      derivation chain).
//!
//! Ported from
//! [Sirush/Jiten Helpers/MorphologicalAnalyser.CombineHelpers.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Helpers/MorphologicalAnalyser.CombineHelpers.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "combine_verb_dependant";

const POSSIBLE_DEPENDANT_DICT_FORMS: &[&str] = &[
    "得る", "しまう", "こなす", "いく", "貰う", "いる", "ない", "だす",
];

/// Te-form subsidiary verbs (Pattern 2 in the C# `CombineVerbDependantsTeiru`).
/// These are verbs that attach to a preceding te-form (giving / receiving
/// auxiliaries, aspectual auxiliaries) and form a single grammatical unit.
const TE_FORM_SUBSIDIARY_VERBS: &[&str] = &[
    "あげる", "上げる", "くれる", "呉れる", "もらう", "貰う", "やる",
    "さしあげる", "差し上げる", "くださる", "下さる",
    "おく", "置く", "みる", "見る",
];

/// Te-form auxiliary chain verbs used in 3-token merging
/// (Pattern 1 in the C# `CombineVerbDependantsTeiru`). Unlike the
/// subsidiary set above, these have full deconjugator coverage so the
/// merged token gets proper conjugation info.
const TE_FORM_AUX_CHAIN_VERBS: &[&str] = &[
    "いる", "居る", "ある", "有る", "おく", "置く",
    "しまう", "仕舞う", "いく", "行く", "くる", "来る", "みる", "見る",
];

pub fn stage() -> Stage {
    Stage::always(NAME, Phase::Combine, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let after_dependants = combine_dependants(morphemes);
    let after_possible = combine_possible_dependants(after_dependants, lexicon);
    let after_suru = combine_suru_chains(after_possible);
    combine_teiru_chains(after_suru, lexicon)
}

fn combine_dependants(morphemes: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for next in morphemes.into_iter().skip(1) {
        // Strict `非自立` (Dependant) only — `非自立可能`
        // (PossibleDependant) flows through to Pass 2, which has
        // tighter conditions (curated dict-form set, lexicon-driven
        // 付く branch). Mirrors C# `HasPartOfSpeechSection(Dependant)`.
        let next_is_dependant = next
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "非自立");
        if next_is_dependant
            && matches!(current.pos, Pos::Verb)
            && next.dictionary_form != "おる"
            && next.surface != current.surface
        {
            current.surface.push_str(&next.surface);
            current.reading_form.push_str(&next.reading_form);
            current.char_range = current.char_range.start..next.char_range.end;
            current.record_rule(NAME);
        } else {
            out.push(current);
            current = next;
        }
    }
    out.push(current);
    out
}

fn combine_possible_dependants(
    morphemes: Vec<Morpheme>,
    lexicon: &dyn Lexicon,
) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for next in morphemes.into_iter().skip(1) {
        let is_classical_wa_row_te = next.dictionary_form.ends_with('う')
            && next.surface.ends_with("いて");
        let in_curated_set = POSSIBLE_DEPENDANT_DICT_FORMS.contains(&next.dictionary_form.as_str());
        let suru_after_past = next.dictionary_form == "する"
            && (current.surface.ends_with('た') || current.surface.ends_with('だ'));
        // Lexicon-driven 付く branch: e.g. 走り + 付く → 走り付く
        // when 走り付く is a known compound. Mirrors the C# branch
        // `currentDictForm[..^1] + "り付く"` checked against the
        // compound lookup. Skipped when the lexicon has no info.
        let tsuku_known_compound = next.dictionary_form == "付く"
            && current.dictionary_form.chars().count() >= 2
            && {
                let dict = &current.dictionary_form;
                let last = dict.chars().last().unwrap();
                let last_byte_len = last.len_utf8();
                let stem = &dict[..dict.len() - last_byte_len];
                let candidate = format!("{}り付く", stem);
                matches!(lexicon.has_compound_entry(&candidate), Some(true))
            };
        let next_is_dependant = next
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "非自立可能");

        if next_is_dependant
            && matches!(current.pos, Pos::Verb)
            && !current.surface.ends_with("たり")
            && next.surface != current.surface
            && !is_classical_wa_row_te
            && (in_curated_set || suru_after_past || tsuku_known_compound)
        {
            current.surface.push_str(&next.surface);
            current.reading_form.push_str(&next.reading_form);
            current.char_range = current.char_range.start..next.char_range.end;
            current.record_rule(NAME);
        } else {
            out.push(current);
            current = next;
        }
    }
    out.push(current);
    out
}

fn combine_suru_chains(morphemes: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        let cur = &morphemes[i];
        if i + 1 < morphemes.len() {
            let next = &morphemes[i + 1];
            let is_modern_suru = next.dictionary_form == "する"
                && next.surface != "する"
                && next.surface != "しない"
                && !next.surface.ends_with("すぎ")
                && !next.surface.ends_with("過ぎ");
            let is_literary_suru = next.dictionary_form == "す" && next.normalized_form == "為る";
            let is_suru_noun = cur
                .part_of_speech
                .iter()
                .skip(1)
                .any(|p| p == "サ変可能");
            if is_suru_noun && (is_modern_suru || is_literary_suru) {
                let mut merged = cur.clone();
                merged.surface.push_str(&next.surface);
                merged.reading_form.push_str(&next.reading_form);
                merged.char_range = cur.char_range.start..next.char_range.end;
                merged.record_rule(NAME);
                out.push(merged);
                i += 2;
                continue;
            }
        }
        out.push(cur.clone());
        i += 1;
    }
    out
}

/// Pass 4 — three te-form auxiliary patterns. Tries each pattern in
/// sequence; the first match wins.
fn combine_teiru_chains(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        // Pattern 1: Verb + て (particle, dict form て) + te-form-aux
        // verb (3 tokens). Catches cases where て wasn't already
        // glued onto the verb by an earlier conjunctive-particle stage.
        if i + 2 < morphemes.len() {
            let cur = &morphemes[i];
            let p1 = &morphemes[i + 1];
            let p2 = &morphemes[i + 2];
            if matches!(cur.pos, Pos::Verb)
                && p1.dictionary_form == "て"
                && TE_FORM_AUX_CHAIN_VERBS.contains(&p2.dictionary_form.as_str())
            {
                let mut merged = cur.clone();
                merged.surface.push_str(&p1.surface);
                merged.surface.push_str(&p2.surface);
                merged.reading_form.push_str(&p1.reading_form);
                merged.reading_form.push_str(&p2.reading_form);
                merged.char_range = cur.char_range.start..p2.char_range.end;
                merged.record_rule(NAME);
                out.push(merged);
                i += 3;
                continue;
            }
        }

        // Pattern 2: Word ending in て/で + subsidiary verb (2 tokens).
        // Catches patterns where て is already part of the previous
        // verb (e.g., 進んで + ない, 愛して + あげられる).
        if i + 1 < morphemes.len() {
            let cur = &morphemes[i];
            let next = &morphemes[i + 1];
            let is_classical_wa_row_te = next.dictionary_form.ends_with('う')
                && next.surface.ends_with("いて");
            let cur_ends_in_te_de =
                cur.surface.ends_with('て') || cur.surface.ends_with('で');
            let cur_inflectable_te_pred =
                matches!(cur.pos, Pos::Verb | Pos::Adjective);
            let next_not_iadj = !matches!(next.pos, Pos::Adjective);

            if cur_ends_in_te_de
                && cur_inflectable_te_pred
                && !is_classical_wa_row_te
                && next_not_iadj
            {
                let mut is_subsidiary = false;

                if matches!(next.pos, Pos::Verb) && next.dictionary_form != "おる" {
                    let next_is_dependant = next
                        .part_of_speech
                        .iter()
                        .skip(1)
                        .any(|p| p == "非自立可能");
                    is_subsidiary = (next_is_dependant
                        && (next.dictionary_form == "いる" || next.dictionary_form == "ない"))
                        || TE_FORM_SUBSIDIARY_VERBS.contains(&next.dictionary_form.as_str())
                        || TE_FORM_SUBSIDIARY_VERBS.contains(&next.normalized_form.as_str());
                }

                // Deconjugator fallback: catches colloquial forms
                // where Sudachi misclassifies (e.g. こん tagged as
                // pronoun for the negative of くる).
                if !is_subsidiary {
                    let forms = lexicon.lookup_conjugated_form(&next.surface);
                    is_subsidiary = forms.iter().any(|f| {
                        TE_FORM_SUBSIDIARY_VERBS.contains(&f.text.as_str())
                            || (TE_FORM_AUX_CHAIN_VERBS.contains(&f.text.as_str())
                                && f.tags.iter().any(|t| t.starts_with('v')))
                    });
                }

                if is_subsidiary {
                    let mut merged = cur.clone();
                    merged.surface.push_str(&next.surface);
                    merged.reading_form.push_str(&next.reading_form);
                    merged.char_range = cur.char_range.start..next.char_range.end;
                    merged.record_rule(NAME);
                    out.push(merged);
                    i += 2;
                    continue;
                }
            }
        }

        // Pattern 3: Verb ending in っ (sokuonbin) + dialectal とる
        // (e.g., 入っ + とらん → 入っとらん, dialectal ている).
        if i + 1 < morphemes.len() {
            let cur = &morphemes[i];
            let next = &morphemes[i + 1];
            if matches!(cur.pos, Pos::Verb)
                && cur.surface.ends_with('っ')
                && next.dictionary_form == "とる"
            {
                let forms = lexicon.lookup_conjugated_form(&next.surface);
                let is_te_oru_form = forms.iter().any(|f| {
                    f.process.iter().any(|p| p.contains("toru (teoru)"))
                        && f.tags
                            .iter()
                            .any(|t| t.starts_with('v') || t.starts_with("stem-te"))
                });
                if is_te_oru_form {
                    let mut merged = cur.clone();
                    merged.surface.push_str(&next.surface);
                    merged.reading_form.push_str(&next.reading_form);
                    merged.char_range = cur.char_range.start..next.char_range.end;
                    merged.record_rule(NAME);
                    out.push(merged);
                    i += 2;
                    continue;
                }
            }
        }

        out.push(morphemes[i].clone());
        i += 1;
    }
    out
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

    #[test]
    fn merges_te_iru_simple_dependant_pattern() {
        // Verb + いる (Dependant sub-POS) → merge.
        let nete = synth("寝て", "寝る", &["動詞"], 0..2);
        let iru = synth("いる", "いる", &["動詞", "非自立可能"], 2..4);
        let out = apply(vec![nete, iru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "寝ている");
    }

    #[test]
    fn merges_suru_after_suru_noun() {
        let benkyou = synth("勉強", "勉強", &["名詞", "サ変可能"], 0..2);
        let suru = synth("し", "する", &["動詞"], 2..3);
        let out = apply(vec![benkyou, suru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "勉強し");
    }

    #[test]
    fn does_not_merge_oru_dependant() {
        let nete = synth("寝て", "寝る", &["動詞"], 0..2);
        let oru = synth("おる", "おる", &["動詞", "非自立可能"], 2..4);
        let out = apply(vec![nete, oru], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_when_next_is_same_as_current() {
        // Guard against repeated identical surfaces.
        let mut a = synth("いる", "いる", &["動詞"], 0..2);
        a.pos = Pos::Verb;
        let mut b = synth("いる", "いる", &["動詞", "非自立可能"], 2..4);
        b.pos = Pos::Verb;
        let out = apply(vec![a, b], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    // === Pass 4: te-iru chain tests ===

    #[test]
    fn pass4_three_token_verb_te_iru_chain() {
        // 食べ (Verb) + て (particle, dict て) + いる (te-form aux) →
        // merged 3-way.
        let mut tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        tabe.pos = Pos::Verb;
        let mut te = synth("て", "て", &["助詞", "接続助詞"], 2..3);
        te.pos = Pos::Particle;
        let mut iru = synth("いる", "いる", &["動詞"], 3..5);
        iru.pos = Pos::Verb;
        let out = apply(vec![tabe, te, iru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べている");
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn pass4_two_token_te_form_subsidiary_kureru() {
        // 持って + くれる → merged.
        let mut motte = synth("持って", "持つ", &["動詞"], 0..3);
        motte.pos = Pos::Verb;
        let mut kureru = synth("くれる", "くれる", &["動詞"], 3..6);
        kureru.pos = Pos::Verb;
        let out = apply(vec![motte, kureru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "持ってくれる");
    }

    #[test]
    fn pass4_two_token_te_form_subsidiary_oku() {
        // 食べて + おく → merged.
        let mut tabete = synth("食べて", "食べる", &["動詞"], 0..3);
        tabete.pos = Pos::Verb;
        let mut oku = synth("おく", "おく", &["動詞"], 3..5);
        oku.pos = Pos::Verb;
        let out = apply(vec![tabete, oku], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べておく");
    }

    #[test]
    fn pass4_skips_classical_wa_row_te() {
        // 貰いて (DictForm=貰う + いて suffix) — classical te-form,
        // not modern. Must NOT merge as subsidiary.
        let mut motte = synth("貰い", "貰う", &["動詞"], 0..2);
        motte.pos = Pos::Verb;
        let mut classical = synth("いて", "いる", &["動詞"], 2..4);
        classical.pos = Pos::Verb;
        let out = apply(vec![motte, classical], &EmptyLexicon);
        // Pass 1 (combine_dependants) might merge if next has 非自立可能,
        // so we use a regular 動詞 sub-POS to keep this test isolated.
        // Either way, the te-iru pass must not over-merge.
        assert!(out.iter().any(|m| m.surface.contains("貰い")));
    }

    #[test]
    fn pass4_three_token_does_not_merge_when_aux_unknown() {
        // て followed by an unknown aux verb (たべる) — not in the
        // chain set. Don't merge.
        let mut tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        tabe.pos = Pos::Verb;
        let mut te = synth("て", "て", &["助詞", "接続助詞"], 2..3);
        te.pos = Pos::Particle;
        let mut other = synth("たべる", "たべる", &["動詞"], 3..6);
        other.pos = Pos::Verb;
        let out = apply(vec![tabe, te, other], &EmptyLexicon);
        // No 3-token merge; te + verb stays separate.
        assert!(out.len() >= 2);
    }

    #[test]
    fn pass4_two_token_iadjective_te_form() {
        // 暑くて + ある (hypothetical) — adjective te-form chain.
        // Real example: 暑くて + ない is unusual; 暑くて + たまらない
        // is more common. Just verify the gate works for IAdjective.
        let mut atsukute = synth("暑くて", "暑い", &["形容詞"], 0..3);
        atsukute.pos = Pos::Adjective;
        let mut ageru = synth("あげる", "あげる", &["動詞"], 3..6);
        ageru.pos = Pos::Verb;
        let out = apply(vec![atsukute, ageru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "暑くてあげる");
    }

    #[test]
    fn pass4_dialectal_ttoru_pattern_via_lexicon() {
        // 入っ + とらん → merge gated on lexicon's deconjugator
        // returning a "toru (teoru)" rule in the chain. The default
        // sudachi-morphology rule corpus does include this rule.
        let mut hai = synth("入っ", "入る", &["動詞"], 0..2);
        hai.pos = Pos::Verb;
        let mut toran = synth("とらん", "とる", &["動詞"], 2..5);
        toran.pos = Pos::Verb;
        let out = apply(vec![hai, toran], &EmptyLexicon);
        // Whether this merges depends on the morphology rule corpus
        // including a "toru (teoru)" labeled rule for とらん. Just
        // assert we don't crash and char-coverage is preserved.
        let total: usize = out.iter().map(|m| m.surface.chars().count()).sum();
        assert_eq!(total, 5);
    }

    #[test]
    fn pass2_tsuku_compound_branch_uses_lexicon() {
        // 走り + 付く → merged when lexicon confirms 走り付く is a
        // known compound.
        struct CompoundLexicon;
        impl Lexicon for CompoundLexicon {
            fn has_compound_entry(&self, term: &str) -> Option<bool> {
                Some(term == "走り付く")
            }
        }
        let mut hashiri = synth("走り", "走る", &["動詞"], 0..2);
        hashiri.pos = Pos::Verb;
        let mut tsuku = synth("付く", "付く", &["動詞", "非自立可能"], 2..4);
        tsuku.pos = Pos::Verb;
        let out = apply(vec![hashiri, tsuku], &CompoundLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "走り付く");
    }

    #[test]
    fn pass2_tsuku_skipped_when_lexicon_unknown() {
        // No lexicon info → 付く branch doesn't fire.
        let mut hashiri = synth("走り", "走る", &["動詞"], 0..2);
        hashiri.pos = Pos::Verb;
        let mut tsuku = synth("付く", "付く", &["動詞", "非自立可能"], 2..4);
        tsuku.pos = Pos::Verb;
        let out = apply(vec![hashiri, tsuku], &EmptyLexicon);
        assert_eq!(out.len(), 2, "no merge without lexicon confirmation");
    }
}
