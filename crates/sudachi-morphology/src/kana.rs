//! Kana row arithmetic — the foundational alphabet ops every godan
//! conjugation needs.
//!
//! Japanese verbs work by shifting the terminal kana across vowel
//! rows in the gojuuon table. `書く (書か→書こ→書き→書く→書け)` is
//! `か → こ → き → く → け` (a, o, i, u, e). This module gives us
//! `shift_to_a()`, `shift_to_e()`, etc. for each consonant column.

/// One of the five vowel rows in the gojuuon table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VowelRow {
    A,
    I,
    U,
    E,
    O,
}

impl VowelRow {
    /// Index in the gojuuon (0=A, 1=I, 2=U, 3=E, 4=O).
    pub fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::I => 1,
            Self::U => 2,
            Self::E => 3,
            Self::O => 4,
        }
    }
}

/// Shift the given U-row (dictionary-form) kana to a different vowel
/// row. Used for godan stem generation.
///
/// Returns `None` for kana that aren't U-row terminals (i.e. only
/// works on the 9 valid godan endings: う, く, ぐ, す, つ, ぬ, ぶ, む,
/// る).
pub fn shift_godan_terminal(u_kana: char, target_row: VowelRow) -> Option<char> {
    let row = godan_row_for_terminal(u_kana)?;
    Some(row[target_row.index()])
}

/// The five-kana row for one godan column. `[a, i, u, e, o]`.
fn godan_row_for_terminal(u_kana: char) -> Option<[char; 5]> {
    match u_kana {
        'う' => Some(['わ', 'い', 'う', 'え', 'お']),
        'く' => Some(['か', 'き', 'く', 'け', 'こ']),
        'ぐ' => Some(['が', 'ぎ', 'ぐ', 'げ', 'ご']),
        'す' => Some(['さ', 'し', 'す', 'せ', 'そ']),
        'つ' => Some(['た', 'ち', 'つ', 'て', 'と']),
        'ぬ' => Some(['な', 'に', 'ぬ', 'ね', 'の']),
        'ぶ' => Some(['ば', 'び', 'ぶ', 'べ', 'ぼ']),
        'む' => Some(['ま', 'み', 'む', 'め', 'も']),
        'る' => Some(['ら', 'り', 'る', 'れ', 'ろ']),
        _ => None,
    }
}

/// Strip the last char from a string; return (prefix, last_char).
/// Returns None if the string is empty or not on a char boundary.
pub fn split_last_char(s: &str) -> Option<(String, char)> {
    let last = s.chars().last()?;
    let last_byte_len = last.len_utf8();
    let prefix = &s[..s.len() - last_byte_len];
    Some((prefix.to_string(), last))
}

/// Replace the last char of `s` with `replacement`. Panics if `s` is
/// empty.
pub fn replace_last_char(s: &str, replacement: char) -> String {
    let (prefix, _) = split_last_char(s).expect("replace_last_char on empty string");
    let mut out = prefix;
    out.push(replacement);
    out
}

/// Append a kana suffix to a stem string.
pub fn append(stem: &str, suffix: &str) -> String {
    let mut out = String::with_capacity(stem.len() + suffix.len());
    out.push_str(stem);
    out.push_str(suffix);
    out
}

/// Convert every katakana code point in `s` to its hiragana counterpart.
/// Non-katakana characters (kanji, hiragana, ASCII, punctuation) are
/// left untouched. Useful before passing a surface into the
/// deconjugator, whose rule corpus is hiragana-only.
///
/// Covers `\u{30A1}–\u{30F6}` (small ァ through ヶ); the choonpu mark
/// `ー`, half-width katakana, and katakana iteration marks pass
/// through unchanged.
pub fn katakana_to_hiragana(s: &str) -> String {
    s.chars()
        .map(|c| {
            if ('\u{30A1}'..='\u{30F6}').contains(&c) {
                char::from_u32(c as u32 - 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_godan_terminal_works_for_every_consonant() {
        // 書く paradigm.
        assert_eq!(shift_godan_terminal('く', VowelRow::A), Some('か'));
        assert_eq!(shift_godan_terminal('く', VowelRow::I), Some('き'));
        assert_eq!(shift_godan_terminal('く', VowelRow::U), Some('く'));
        assert_eq!(shift_godan_terminal('く', VowelRow::E), Some('け'));
        assert_eq!(shift_godan_terminal('く', VowelRow::O), Some('こ'));

        // 買う paradigm — note わ for A row (not あ — historical).
        assert_eq!(shift_godan_terminal('う', VowelRow::A), Some('わ'));
        assert_eq!(shift_godan_terminal('う', VowelRow::I), Some('い'));
        assert_eq!(shift_godan_terminal('う', VowelRow::E), Some('え'));
        assert_eq!(shift_godan_terminal('う', VowelRow::O), Some('お'));

        // 死ぬ paradigm.
        assert_eq!(shift_godan_terminal('ぬ', VowelRow::A), Some('な'));
        assert_eq!(shift_godan_terminal('ぬ', VowelRow::I), Some('に'));

        // 走る paradigm.
        assert_eq!(shift_godan_terminal('る', VowelRow::A), Some('ら'));
        assert_eq!(shift_godan_terminal('る', VowelRow::E), Some('れ'));
    }

    #[test]
    fn split_last_char_handles_kanji_and_kana() {
        assert_eq!(
            split_last_char("食べる"),
            Some(("食べ".to_string(), 'る'))
        );
        assert_eq!(split_last_char("書く"), Some(("書".to_string(), 'く')));
        assert_eq!(split_last_char("ある"), Some(("あ".to_string(), 'る')));
        assert_eq!(split_last_char(""), None);
    }

    #[test]
    fn replace_last_char_does_what_it_says() {
        assert_eq!(replace_last_char("書く", 'か'), "書か");
        assert_eq!(replace_last_char("買う", 'わ'), "買わ");
        assert_eq!(replace_last_char("食べる", 'ら'), "食べら"); // pretend
    }

    #[test]
    fn returns_none_for_non_godan_terminals() {
        assert_eq!(shift_godan_terminal('あ', VowelRow::A), None);
        assert_eq!(shift_godan_terminal('き', VowelRow::A), None);
    }

    #[test]
    fn katakana_to_hiragana_converts_full_width_only() {
        assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
        assert_eq!(katakana_to_hiragana("食べた"), "食べた");
        assert_eq!(katakana_to_hiragana("ケドー"), "けどー");
        // 小書き (small kana) covered.
        assert_eq!(katakana_to_hiragana("チャ"), "ちゃ");
    }
}
