//! HuggingFace `tokenizers`-backed wrapper around KWJA's word DeBERTa tokenizer.
//!
//! KWJA's word module uses `ku-nlp/deberta-v2-base-japanese`'s tokenizer.json,
//! a sentencepiece BPE-derived fast tokenizer. We load the file once at
//! Pipeline construction and produce per-input `Encoded` records for the
//! Rust forward path.

use crate::Result;
use crate::error::Error;
use std::path::Path;
use tokenizers::Tokenizer;

pub struct DebertaTokenizer {
    inner: Tokenizer,
}

#[derive(Debug, Clone)]
pub struct Encoded {
    pub input_ids: Vec<u32>,
    pub attention_mask: Vec<u32>,
    pub type_ids: Vec<u32>,
    /// Per-token (byte_start, byte_end) into the original text. Special
    /// tokens have (0, 0).
    pub offsets: Vec<(usize, usize)>,
    /// Per-token word index (None for special tokens). Used by the word
    /// module's subword pooling — same as HF's `Encoding::word_ids()`.
    pub word_ids: Vec<Option<u32>>,
}

impl DebertaTokenizer {
    pub fn load(path: &Path) -> Result<Self> {
        let inner = Tokenizer::from_file(path).map_err(|e| {
            Error::Tokenizer(format!("load {}: {e}", path.display()))
        })?;
        Ok(Self { inner })
    }

    pub fn encode(&self, text: &str) -> Result<Encoded> {
        let enc = self
            .inner
            .encode(text, true)
            .map_err(|e| Error::Tokenizer(e.to_string()))?;
        Ok(Encoded {
            input_ids: enc.get_ids().to_vec(),
            attention_mask: enc.get_attention_mask().to_vec(),
            type_ids: enc.get_type_ids().to_vec(),
            offsets: enc.get_offsets().to_vec(),
            word_ids: enc.get_word_ids().to_vec(),
        })
    }

    /// Encode a pretokenized input — a list of word surfaces. The
    /// tokenizer treats each surface as a single "word" and produces
    /// `word_ids` that map subwords back to their source word index.
    /// This is the path KWJA-Python uses (Juman++ pretokenizes before
    /// HF tokenization); for jisho we pass Sudachi morpheme surfaces.
    pub fn encode_pretokenized(&self, words: &[&str]) -> Result<Encoded> {
        use tokenizers::tokenizer::InputSequence;
        let enc = self
            .inner
            .encode(InputSequence::PreTokenized(words.into()), true)
            .map_err(|e| Error::Tokenizer(e.to_string()))?;
        Ok(Encoded {
            input_ids: enc.get_ids().to_vec(),
            attention_mask: enc.get_attention_mask().to_vec(),
            type_ids: enc.get_type_ids().to_vec(),
            offsets: enc.get_offsets().to_vec(),
            word_ids: enc.get_word_ids().to_vec(),
        })
    }

    /// Batched pretokenized encoding. Each entry is the surface list for one
    /// sentence; outputs are returned in input order. The tokenizer is
    /// thread-safe internally but we call sequentially — encoding cost is
    /// negligible vs the GPU forward.
    pub fn encode_batch_pretokenized(&self, batches: &[Vec<&str>]) -> Result<Vec<Encoded>> {
        batches
            .iter()
            .map(|words| self.encode_pretokenized(words))
            .collect()
    }

    /// Pad token id used to fill batched input_ids past each row's natural
    /// length. KWJA's tokenizer uses [PAD] = 0; the attention mask
    /// distinguishes real tokens from padding.
    pub const PAD_TOKEN_ID: u32 = 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tokenizer_path() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local/share/jisho/checkpoints/kwja-tokenizer/tokenizer.json")
    }

    #[test]
    fn encodes_japanese_sentence() {
        let path = tokenizer_path();
        if !path.exists() {
            eprintln!("skipping: {path:?} not present");
            return;
        }
        let tok = DebertaTokenizer::load(&path).unwrap();
        let enc = tok.encode("今日は晴れです").unwrap();

        // Lengths consistent across all per-token vectors.
        assert!(enc.input_ids.len() >= 3, "expected at least CLS + token + SEP");
        assert_eq!(enc.input_ids.len(), enc.attention_mask.len());
        assert_eq!(enc.input_ids.len(), enc.type_ids.len());
        assert_eq!(enc.input_ids.len(), enc.offsets.len());
        assert_eq!(enc.input_ids.len(), enc.word_ids.len());

        // CLS and SEP are special tokens — word_ids[0] and word_ids[-1] are None.
        assert_eq!(enc.word_ids[0], None);
        assert_eq!(*enc.word_ids.last().unwrap(), None);

        // Some interior tokens have word_ids.
        assert!(enc.word_ids.iter().any(|w| w.is_some()));
    }
}
