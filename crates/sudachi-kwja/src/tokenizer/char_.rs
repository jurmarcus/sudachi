//! Character tokenizer for KWJA's char module.
//!
//! KWJA's `ku-nlp/deberta-v2-base-japanese-char-wwm` uses a BERT-style
//! WordPiece tokenizer — the vocab is one entry per line in `vocab.txt`,
//! starts with special tokens ([PAD]/[CLS]/[SEP]/[UNK]/[MASK]), then `▁`
//! (sentencepiece word marker), then individual characters.
//!
//! We don't load HuggingFace's full WordPiece machinery — the char model's
//! "tokenization" is just one-char-per-token plus CLS/SEP framing. For
//! out-of-vocab characters we emit [UNK] and let the model's argmax do
//! the right thing (KWJA's char module is robust to UNK at sentence
//! segmentation since context still flows through DeBERTa).

use crate::Result;
use crate::tokenizer::deberta::Encoded;
use std::collections::HashMap;
use std::path::Path;

pub struct CharTokenizer {
    cls_id: u32,
    sep_id: u32,
    unk_id: u32,
    char_to_id: HashMap<char, u32>,
}

impl CharTokenizer {
    /// Load from KWJA char-wwm vocab.txt. One token per line; line N → id N.
    pub fn load(vocab_path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(vocab_path)?;
        let mut cls_id = 1u32;
        let mut sep_id = 2u32;
        let mut unk_id = 3u32;
        let mut char_to_id = HashMap::new();

        for (idx, line) in text.lines().enumerate() {
            let id = idx as u32;
            match line {
                "[CLS]" => cls_id = id,
                "[SEP]" => sep_id = id,
                "[UNK]" => unk_id = id,
                "[PAD]" | "[MASK]" => continue,
                "▁" => continue, // sentencepiece marker, never emitted standalone
                tok => {
                    if let Some(ch) = tok.chars().next() {
                        // Some BERT vocab entries are multi-char (subwords);
                        // for char-level segmentation we map only the first
                        // char, mirroring KWJA's char-by-char input.
                        if tok.chars().count() == 1 {
                            char_to_id.insert(ch, id);
                        }
                    }
                }
            }
        }

        Ok(Self { cls_id, sep_id, unk_id, char_to_id })
    }

    pub fn encode(&self, text: &str) -> Result<Encoded> {
        let mut input_ids = Vec::with_capacity(text.chars().count() + 2);
        let mut offsets = Vec::with_capacity(input_ids.capacity());
        let mut word_ids: Vec<Option<u32>> = Vec::with_capacity(input_ids.capacity());

        input_ids.push(self.cls_id);
        offsets.push((0usize, 0usize));
        word_ids.push(None);

        for (byte_idx, ch) in text.char_indices() {
            let id = *self.char_to_id.get(&ch).unwrap_or(&self.unk_id);
            input_ids.push(id);
            offsets.push((byte_idx, byte_idx + ch.len_utf8()));
            // For sentence segmentation each char IS its own "word".
            word_ids.push(Some(input_ids.len() as u32 - 2));
        }

        let end = text.len();
        input_ids.push(self.sep_id);
        offsets.push((end, end));
        word_ids.push(None);

        let attention_mask = vec![1u32; input_ids.len()];
        let type_ids = vec![0u32; input_ids.len()];

        Ok(Encoded { input_ids, attention_mask, type_ids, offsets, word_ids })
    }

    pub fn vocab_size(&self) -> usize {
        // Largest known id + 1. Char map gives IDs in vocab order.
        self.char_to_id.values().max().copied().unwrap_or(0) as usize + 1
    }

    pub fn unk_id(&self) -> u32 {
        self.unk_id
    }

    /// Special-token ids — exposed for callers that need to mask special
    /// positions in subword pooling.
    pub fn special_token_ids(&self) -> [u32; 3] {
        [self.cls_id, self.sep_id, self.unk_id]
    }
}

impl Default for CharTokenizer {
    fn default() -> Self {
        Self {
            cls_id: 1,
            sep_id: 2,
            unk_id: 3,
            char_to_id: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn vocab_path() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local/share/jisho/checkpoints/kwja-char/vocab.txt")
    }

    #[test]
    fn loads_real_kwja_char_vocab() {
        let path = vocab_path();
        if !path.exists() {
            eprintln!("skipping: {path:?} not present");
            return;
        }
        let tok = CharTokenizer::load(&path).unwrap();
        // KWJA char-wwm vocab is ~22k entries.
        assert!(tok.vocab_size() > 5000, "expected real vocab loaded");
        // Common Japanese chars should be in the vocab.
        let enc = tok.encode("今日").unwrap();
        // CLS + 今 + 日 + SEP
        assert_eq!(enc.input_ids.len(), 4);
        // Center two tokens shouldn't be UNK.
        assert_ne!(enc.input_ids[1], tok.unk_id());
        assert_ne!(enc.input_ids[2], tok.unk_id());
    }

    #[test]
    fn unknown_chars_become_unk() {
        // Default tokenizer has empty vocab — every input char is UNK.
        let tok = CharTokenizer::default();
        let enc = tok.encode("abc").unwrap();
        assert_eq!(enc.input_ids.len(), 5);
        for id in &enc.input_ids[1..4] {
            assert_eq!(*id, tok.unk_id());
        }
    }

    #[test]
    fn empty_text_produces_cls_sep() {
        let tok = CharTokenizer::default();
        let enc = tok.encode("").unwrap();
        assert_eq!(enc.input_ids.len(), 2);
        assert_eq!(enc.offsets, vec![(0, 0), (0, 0)]);
    }
}
