//! KWJA typo correction module port.
//!
//! KWJA's `TypoModule` (kwja/modules/typo.py) is:
//!   encoder: DeBERTa-v2-base-wwm (PreTrainedModel)
//!   kdr_tagger: SequentialMlpHead → vocab_size labels (Keep / Delete / Replace)
//!   ins_tagger: SequentialMlpHead → vocab_size + extended_vocab labels (Insert)
//!
//! Forward returns `kdr_logits` and `ins_logits` per **char** position
//! (subwords are 1:1 with chars for this tokenizer). The decoder
//! (`TypoModel::correct`) converts these to edit operations and applies
//! them to recover the post-edit text.

use crate::Result;
use crate::checkpoint::Checkpoint;
use crate::error::Error;
use crate::model::deberta::{DebertaBackbone, checkpoint_var_builder};
use crate::model::heads::SequentialMlpHead;
use crate::tokenizer::typo::{TypoEncoded, TypoTokenizer};
use candle_core::Tensor;
use std::path::Path;

pub struct TypoModel {
    tokenizer: TypoTokenizer,
    encoder: DebertaBackbone,
    kdr_tagger: SequentialMlpHead,
    ins_tagger: SequentialMlpHead,
    /// Extended insertion vocabulary (multi_char_vocab.txt). Insert
    /// classifications above `vocab_size` index into this list.
    extended_vocab: Vec<String>,
    /// Base vocab size of the encoder tokenizer; the boundary between
    /// "regular tokens" and the extended insertion vocabulary.
    vocab_size: usize,
}

pub struct TypoLogits {
    pub kdr_logits: Tensor,    // (1, T_subwords, vocab_size)
    pub ins_logits: Tensor,    // (1, T_subwords, vocab_size + extended_vocab)
    pub encoded: TypoEncoded,
}

impl TypoModel {
    /// Load TypoModel from a converted `typo.safetensors` checkpoint.
    ///
    /// Args:
    /// - `checkpoint`: loaded safetensors (any device).
    /// - `vocab_path`: path to the typo `vocab.txt` (char-level Bert vocab).
    /// - `multi_char_vocab_path`: path to KWJA's `multi_char_vocab.txt`,
    ///   the extended insertion vocabulary used by the ins_tagger.
    pub fn load(
        checkpoint: &Checkpoint,
        vocab_path: &Path,
        multi_char_vocab_path: &Path,
    ) -> Result<Self> {
        let tokenizer = TypoTokenizer::load(vocab_path)?;
        let encoder = DebertaBackbone::from_checkpoint(checkpoint)?;
        let vb = checkpoint_var_builder(checkpoint, encoder.dtype())?;
        let h = crate::constants::HIDDEN_SIZE;

        // The kdr_tagger output dim equals the encoder vocab_size; read it
        // from the head's output linear weight rather than relying on a
        // hard-coded constant.
        let vocab_size = checkpoint
            .get("kdr_tagger.3.weight")?
            .dim(0)
            .map_err(Error::from)?;
        let ins_dim = checkpoint
            .get("ins_tagger.3.weight")?
            .dim(0)
            .map_err(Error::from)?;
        let kdr_tagger = SequentialMlpHead::from_var_builder(vb.pp("kdr_tagger"), h, vocab_size)?;
        let ins_tagger = SequentialMlpHead::from_var_builder(vb.pp("ins_tagger"), h, ins_dim)?;

        let vocab_text = std::fs::read_to_string(multi_char_vocab_path).map_err(|e| {
            Error::Tokenizer(format!("read {}: {e}", multi_char_vocab_path.display()))
        })?;
        let extended_vocab: Vec<String> =
            vocab_text.lines().filter(|l| !l.is_empty()).map(String::from).collect();

        // Sanity: ins_tagger output dim should equal vocab_size + extended_vocab.len().
        if ins_dim != vocab_size + extended_vocab.len() {
            return Err(Error::InvalidInput(format!(
                "typo ins_tagger output dim {ins_dim} != vocab_size {vocab_size} + extended_vocab \
                 len {} ({})",
                extended_vocab.len(),
                vocab_size + extended_vocab.len()
            )));
        }

        Ok(Self { tokenizer, encoder, kdr_tagger, ins_tagger, extended_vocab, vocab_size })
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }
    pub fn extended_vocab(&self) -> &[String] {
        &self.extended_vocab
    }
    pub fn tokenizer(&self) -> &TypoTokenizer {
        &self.tokenizer
    }

    /// Forward on a single text. Returns kdr_logits + ins_logits over the
    /// CLS-bracketed subword sequence.
    pub fn forward(&self, text: &str) -> Result<TypoLogits> {
        let encoded = self.tokenizer.encode(text);
        let device = self.encoder.device().clone();
        let input_ids = Tensor::new(vec![encoded.input_ids.clone()], &device).map_err(Error::from)?;
        let attn = Tensor::new(vec![encoded.attention_mask.clone()], &device).map_err(Error::from)?;
        let attn_f32 = attn.to_dtype(self.encoder.dtype()).map_err(Error::from)?;
        let hidden = self.encoder.forward(&input_ids, &attn_f32, None)?;
        let kdr_logits = self.kdr_tagger.forward(&hidden)?;
        let ins_logits = self.ins_tagger.forward(&hidden)?;
        Ok(TypoLogits { kdr_logits, ins_logits, encoded })
    }

    /// Apply typo correction. Returns the corrected text + a `changed` flag.
    /// Mirrors KWJA-Python's `convert_typo_predictions_into_tags` +
    /// `apply_edit_operations` (kwja/callbacks/utils.py).
    ///
    /// `confidence_threshold` is the per-position softmax-max threshold
    /// below which a position emits "no edit" (Keep / no insert). Default
    /// in KWJA is 0.9.
    pub fn correct(&self, text: &str, confidence_threshold: f32) -> Result<CorrectionResult> {
        let logits = self.forward(text)?;
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        // T = CLS + n chars + SEP. kdr aligns with positions 1..=n; ins
        // with positions 1..=n+1 (the trailing one is for end-of-text inserts).
        let t = logits.encoded.input_ids.len();
        if t != n + 2 {
            // Defensive: encoder's char-level tokenization should always
            // produce CLS+n+SEP. If it doesn't, return input unchanged.
            return Ok(CorrectionResult { corrected: text.to_string(), changed: false });
        }

        let kdr_argmax = softmax_argmax_with_max(&logits.kdr_logits)?;
        let ins_argmax = softmax_argmax_with_max(&logits.ins_logits)?;

        // Decode per-position to KDR / INS tags. KDR vocab is vocab_size
        // wide; INS is vocab_size + extended_vocab.
        let kdr_tags: Vec<KdrTag> = (1..=n)
            .map(|i| self.decode_kdr(kdr_argmax[i].0, kdr_argmax[i].1, confidence_threshold))
            .collect();
        // n+1 ins tags (one before each char + one trailing).
        let ins_tags: Vec<InsTag> = (1..=n + 1)
            .map(|i| {
                if i >= ins_argmax.len() {
                    InsTag::None
                } else {
                    self.decode_ins(ins_argmax[i].0, ins_argmax[i].1, confidence_threshold)
                }
            })
            .collect();

        // Apply edits left-to-right.
        let mut out = String::with_capacity(text.len());
        for (i, c) in chars.iter().enumerate() {
            if let InsTag::Insert(s) = &ins_tags[i] {
                out.push_str(s);
            }
            match &kdr_tags[i] {
                KdrTag::Keep => out.push(*c),
                KdrTag::Delete => {}
                KdrTag::Replace(s) => out.push_str(s),
            }
        }
        if let InsTag::Insert(s) = &ins_tags[n] {
            out.push_str(s);
        }
        let changed = out != text;
        Ok(CorrectionResult { corrected: out, changed })
    }

    fn decode_kdr(&self, idx: u32, prob: f32, threshold: f32) -> KdrTag {
        if prob < threshold {
            return KdrTag::Keep;
        }
        let id = idx as usize;
        if id >= self.tokenizer.id_to_token.len() {
            return KdrTag::Keep;
        }
        let tok = self.tokenizer.id_to_token[id].as_str();
        match tok {
            "<k>" | "_" | "[PAD]" | "[CLS]" | "[SEP]" | "[MASK]" | "[UNK]" => KdrTag::Keep,
            "<d>" => KdrTag::Delete,
            "<_>" => KdrTag::Keep,
            _ => KdrTag::Replace(tok.to_string()),
        }
    }

    fn decode_ins(&self, idx: u32, prob: f32, threshold: f32) -> InsTag {
        if prob < threshold {
            return InsTag::None;
        }
        let id = idx as usize;
        if id < self.vocab_size {
            // Base vocab: [PAD] / [CLS] / [SEP] / [MASK] / [UNK] / <_> /
            // <k> all map to "no insert"; anything else inserts that token.
            let tok = match self.tokenizer.id_to_token.get(id) {
                Some(t) => t.as_str(),
                None => return InsTag::None,
            };
            match tok {
                "<_>" | "<k>" | "[PAD]" | "[CLS]" | "[SEP]" | "[MASK]" | "[UNK]" => InsTag::None,
                "<d>" => InsTag::None, // delete-marker doesn't apply to insert
                _ => InsTag::Insert(tok.to_string()),
            }
        } else {
            // Extended vocab (multi_char_vocab.txt) — multi-char inserts.
            let ext_idx = id - self.vocab_size;
            match self.extended_vocab.get(ext_idx) {
                Some(s) => InsTag::Insert(s.clone()),
                None => InsTag::None,
            }
        }
    }
}

/// One position's KDR decision after threshold.
#[derive(Debug, Clone)]
enum KdrTag {
    Keep,
    Delete,
    Replace(String),
}

/// One position's INS decision after threshold.
#[derive(Debug, Clone)]
enum InsTag {
    None,
    Insert(String),
}

/// Returned by `TypoModel::correct`.
pub struct CorrectionResult {
    pub corrected: String,
    pub changed: bool,
}

/// Compute softmax + argmax + max-prob over the trailing dim of a (1, T, V)
/// logits tensor. Returns `Vec<(argmax_idx, max_prob)>` of length T.
fn softmax_argmax_with_max(logits: &Tensor) -> Result<Vec<(u32, f32)>> {
    let probs = candle_nn::ops::softmax(logits, 2).map_err(Error::from)?;
    let v = probs
        .squeeze(0)
        .map_err(Error::from)?
        .to_dtype(candle_core::DType::F32)
        .map_err(Error::from)?
        .to_vec2::<f32>()
        .map_err(Error::from)?;
    Ok(v
        .into_iter()
        .map(|row| {
            let mut best_idx = 0u32;
            let mut best_p = f32::NEG_INFINITY;
            for (i, p) in row.into_iter().enumerate() {
                if p > best_p {
                    best_p = p;
                    best_idx = i as u32;
                }
            }
            (best_idx, best_p)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn paths() -> (PathBuf, PathBuf, PathBuf) {
        let home = PathBuf::from(std::env::var("HOME").unwrap());
        (
            home.join(".local/share/jisho/checkpoints/typo.safetensors"),
            home.join(".local/share/jisho/checkpoints/typo_tokenizer/vocab.txt"),
            home.join(".local/share/jisho/checkpoints/typo_resources/multi_char_vocab.txt"),
        )
    }

    #[test]
    fn typo_model_loads_and_forwards() {
        let (ckpt, vocab, multi) = paths();
        if !ckpt.exists() || !vocab.exists() || !multi.exists() {
            eprintln!("skip: typo resources missing (ckpt={}, vocab={}, multi={})",
                ckpt.exists(), vocab.exists(), multi.exists());
            return;
        }
        let cp = Checkpoint::load_with_device(&ckpt, Device::Cpu).unwrap();
        let model = TypoModel::load(&cp, &vocab, &multi).unwrap();
        let logits = model.forward("今日は天気がいいいいですね").unwrap();
        // kdr_logits shape: (1, T_subwords, vocab_size)
        assert_eq!(logits.kdr_logits.dims().len(), 3);
        assert_eq!(logits.kdr_logits.dim(0).unwrap(), 1);
        assert_eq!(logits.kdr_logits.dim(2).unwrap(), model.vocab_size());
        // ins_logits is wider: vocab_size + extended_vocab.len().
        assert_eq!(
            logits.ins_logits.dim(2).unwrap(),
            model.vocab_size() + model.extended_vocab().len()
        );
        // Length: CLS + chars + SEP.
        assert_eq!(logits.kdr_logits.dim(1).unwrap(), logits.encoded.input_ids.len());
    }
}
