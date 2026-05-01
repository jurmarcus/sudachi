//! Typo tokenizer: char-level Bert-style tokenizer for KWJA's typo model.
//!
//! KWJA's typo model uses `ku-nlp/deberta-v2-base-japanese-char-wwm`, which
//! ships as `BertJapaneseTokenizer` with `subword_tokenizer_type=character`
//! — i.e. each input character maps to one vocab id. The tokenizer is NOT
//! distributed as a `tokenizer.json` (HF fast format), so we can't reuse the
//! `tokenizers` crate path used by the word module. Instead we read
//! `vocab.txt` directly and tokenize char-by-char.
//!
//! Encoded layout for input `"今日は"`:
//!     input_ids: [CLS, 今, 日, は, SEP]
//!     attention_mask: [1, 1, 1, 1, 1]
//!     char_offsets: [None, Some((0, 0)), Some((0, 1)), Some((1, 2)), None]
//!
//! `char_offsets[i]` records the **char index** range this token covers in
//! the original string (not byte offsets). The decoder uses these to map
//! per-token KDR/INS predictions back to the input chars.

use crate::Result;
use crate::error::Error;
use std::collections::HashMap;
use std::path::Path;

/// IDs of the special tokens; matches KWJA's vocab.txt layout (lines 1-5).
pub const PAD_ID: u32 = 0;
pub const CLS_ID: u32 = 1;
pub const SEP_ID: u32 = 2;
pub const UNK_ID: u32 = 3;
pub const MASK_ID: u32 = 4;

const PAD_TOK: &str = "[PAD]";
const CLS_TOK: &str = "[CLS]";
const SEP_TOK: &str = "[SEP]";
const UNK_TOK: &str = "[UNK]";
const MASK_TOK: &str = "[MASK]";

pub struct TypoTokenizer {
    /// vocab token → id
    pub vocab: HashMap<String, u32>,
    /// id → token (parallel to vocab; size == vocab.len())
    pub id_to_token: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TypoEncoded {
    pub input_ids: Vec<u32>,
    pub attention_mask: Vec<u32>,
    /// Per-token char index in the original input. None for special tokens
    /// (CLS / SEP). Length matches input_ids.
    pub char_offsets: Vec<Option<usize>>,
}

impl TypoTokenizer {
    /// Load from a `vocab.txt` (one token per line, line N maps to id N-0).
    /// KWJA's vocab is 22012 entries: PAD/CLS/SEP/UNK/MASK + ▁ + chars.
    pub fn load(vocab_path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(vocab_path)
            .map_err(|e| Error::Tokenizer(format!("read {}: {e}", vocab_path.display())))?;
        let mut id_to_token: Vec<String> = Vec::with_capacity(22_500);
        for line in text.lines() {
            id_to_token.push(line.to_string());
        }
        let mut vocab = HashMap::with_capacity(id_to_token.len());
        for (i, t) in id_to_token.iter().enumerate() {
            vocab.insert(t.clone(), i as u32);
        }
        // Sanity-check special-token ids match the layout we assume.
        for (expected_id, tok) in [
            (PAD_ID, PAD_TOK),
            (CLS_ID, CLS_TOK),
            (SEP_ID, SEP_TOK),
            (UNK_ID, UNK_TOK),
            (MASK_ID, MASK_TOK),
        ] {
            if vocab.get(tok).copied() != Some(expected_id) {
                return Err(Error::Tokenizer(format!(
                    "typo vocab.txt: expected {tok} at id {expected_id}, found {:?}",
                    vocab.get(tok)
                )));
            }
        }
        Ok(Self { vocab, id_to_token })
    }

    /// Look up a token id; returns UNK_ID for unknown tokens.
    pub fn token_id(&self, tok: &str) -> u32 {
        self.vocab.get(tok).copied().unwrap_or(UNK_ID)
    }

    /// Encode a single text. Each input char becomes one token; CLS / SEP
    /// bracket. Returns the encoded representation.
    pub fn encode(&self, text: &str) -> TypoEncoded {
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        let mut input_ids = Vec::with_capacity(n + 2);
        let mut attention_mask = Vec::with_capacity(n + 2);
        let mut char_offsets: Vec<Option<usize>> = Vec::with_capacity(n + 2);

        input_ids.push(CLS_ID);
        attention_mask.push(1);
        char_offsets.push(None);

        for (i, c) in chars.iter().enumerate() {
            let s = c.to_string();
            let id = self.token_id(&s);
            input_ids.push(id);
            attention_mask.push(1);
            char_offsets.push(Some(i));
        }

        input_ids.push(SEP_ID);
        attention_mask.push(1);
        char_offsets.push(None);

        TypoEncoded { input_ids, attention_mask, char_offsets }
    }

    /// Batched encode. Each text gets independent encoding; padding is the
    /// caller's responsibility (TypoModel forward pads to T_max).
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<TypoEncoded> {
        texts.iter().map(|t| self.encode(t)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn vocab_path() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local/share/jisho/checkpoints/typo_tokenizer/vocab.txt")
    }

    #[test]
    fn typo_tokenizer_loads_and_encodes() {
        let p = vocab_path();
        if !p.exists() {
            eprintln!("skip: {p:?} not present");
            return;
        }
        let tok = TypoTokenizer::load(&p).unwrap();
        assert!(tok.vocab.len() >= 22_000);
        // Special-token slots match layout.
        assert_eq!(tok.token_id("[CLS]"), CLS_ID);
        assert_eq!(tok.token_id("[SEP]"), SEP_ID);

        let encoded = tok.encode("今日は");
        // Length: CLS + 3 chars + SEP.
        assert_eq!(encoded.input_ids.len(), 5);
        assert_eq!(encoded.input_ids[0], CLS_ID);
        assert_eq!(*encoded.input_ids.last().unwrap(), SEP_ID);
        assert_eq!(encoded.attention_mask.iter().sum::<u32>(), 5);
        assert_eq!(encoded.char_offsets[0], None);
        assert_eq!(encoded.char_offsets[1], Some(0));
        assert_eq!(encoded.char_offsets[2], Some(1));
        assert_eq!(encoded.char_offsets[3], Some(2));
        assert_eq!(*encoded.char_offsets.last().unwrap(), None);
    }
}
