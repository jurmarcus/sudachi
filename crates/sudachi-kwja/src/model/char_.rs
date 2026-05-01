//! Char module: sentence segmentation.
//!
//! KWJA's char module has three heads (sent_segmentation_tagger,
//! word_segmentation_tagger, word_norm_op_tagger). v0.1 only consumes
//! sentence segmentation — the other two are reserved for future use.
//!
//! Pipeline: text → CharTokenizer → DeBERTa encoder → sent_segmentation_tagger
//! → (B, S, 2) logits over (B, I) labels.

use crate::Result;
use crate::checkpoint::Checkpoint;
use crate::error::Error;
use crate::model::deberta::{DebertaBackbone, checkpoint_var_builder};
use crate::model::heads::SequentialMlpHead;
use crate::tokenizer::CharTokenizer;
use candle_core::{DType, Tensor};
use std::path::Path;

pub struct CharModel {
    tokenizer: CharTokenizer,
    backbone: DebertaBackbone,
    sent_segmentation: SequentialMlpHead,
}

impl CharModel {
    pub fn load(checkpoint: &Checkpoint, vocab_path: &Path) -> Result<Self> {
        let tokenizer = CharTokenizer::load(vocab_path)?;
        let backbone = DebertaBackbone::from_checkpoint(checkpoint)?;
        let vb = checkpoint_var_builder(checkpoint, backbone.dtype())?;
        let sent_segmentation = SequentialMlpHead::from_var_builder(
            vb.pp("sent_segmentation_tagger"),
            crate::constants::HIDDEN_SIZE,
            2, // B / I
        )?;
        Ok(Self { tokenizer, backbone, sent_segmentation })
    }

    /// Returns sent_segmentation logits of shape (1, T, 2). Used by
    /// equivalence tests.
    pub fn logits(&self, text: &str) -> Result<Tensor> {
        let enc = self.tokenizer.encode(text)?;
        let device = self.backbone.device().clone();
        let input_ids = Tensor::new(vec![enc.input_ids.clone()], &device).map_err(Error::from)?;
        let attention_mask =
            Tensor::new(vec![enc.attention_mask.clone()], &device).map_err(Error::from)?;
        let attention_mask_f32 = attention_mask
            .to_dtype(self.backbone.dtype())
            .map_err(Error::from)?;
        let hidden = self.backbone.forward(&input_ids, &attention_mask_f32, None)?;
        self.sent_segmentation.forward(&hidden)
    }

    /// Decode argmax along the labels dim → per-position 0/1 (B/I) tags.
    /// Returns one Vec<u32> per batch element, one entry per token (incl.
    /// CLS at position 0 and SEP at the last position).
    pub fn segment_labels(&self, text: &str) -> Result<Vec<u32>> {
        let logits = self.logits(text)?;
        let argmax = logits
            .argmax_keepdim(2)
            .map_err(Error::from)?
            .squeeze(2)
            .map_err(Error::from)?
            .squeeze(0)
            .map_err(Error::from)?;
        argmax
            .to_dtype(DType::U32)
            .map_err(Error::from)?
            .to_vec1()
            .map_err(Error::from)
    }

    /// Split `text` into sentences using the char model's B/I tags.
    /// `B` (label 0) = sentence boundary; `I` (label 1) = continuation.
    ///
    /// Returns owned `String`s rather than slices so the caller doesn't have
    /// to track byte offsets — KWJA's char tokenizer already handles UTF-8
    /// boundaries via `char_indices`. Empty input → empty Vec.
    pub fn split_sentences(&self, text: &str) -> Result<Vec<String>> {
        if text.is_empty() {
            return Ok(vec![]);
        }
        let labels = self.segment_labels(text)?;
        // labels[0] is CLS, labels[len-1] is SEP — strip them. Middle labels
        // align 1:1 with `text.chars()`.
        let chars: Vec<(usize, char)> = text.char_indices().collect();
        if labels.len() < chars.len() + 2 {
            // Truncated input (text exceeded MAX_POSITION_EMBEDDINGS); fall
            // back to whole-text-as-one-sentence rather than emitting
            // partial output.
            return Ok(vec![text.to_string()]);
        }
        let inner = &labels[1..chars.len() + 1];

        let mut sentences: Vec<String> = Vec::new();
        let mut current = String::new();
        for (i, (_byte, ch)) in chars.iter().enumerate() {
            if inner[i] == 0 && !current.is_empty() {
                // 'B' on a non-first char means a new sentence starts here.
                sentences.push(std::mem::take(&mut current));
            }
            current.push(*ch);
        }
        if !current.is_empty() {
            sentences.push(current);
        }
        Ok(sentences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn home() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap())
    }

    #[test]
    fn char_model_loads_and_forwards() {
        let ckpt_path = home().join(".local/share/jisho/checkpoints/char.safetensors");
        let vocab_path = home().join(".local/share/jisho/checkpoints/kwja-char/vocab.txt");
        if !ckpt_path.exists() || !vocab_path.exists() {
            eprintln!("skipping: char.safetensors or vocab.txt missing");
            return;
        }
        let cp = Checkpoint::load_with_device(&ckpt_path, Device::Cpu).unwrap();
        let model = CharModel::load(&cp, &vocab_path).unwrap();
        let logits = model.logits("今日は晴れです").unwrap();
        // CLS + 7 chars + SEP = 9 positions, 2 labels (B/I).
        assert_eq!(logits.dims(), &[1, 9, 2]);
    }

    #[test]
    fn char_model_decodes_segment_labels() {
        let ckpt_path = home().join(".local/share/jisho/checkpoints/char.safetensors");
        let vocab_path = home().join(".local/share/jisho/checkpoints/kwja-char/vocab.txt");
        if !ckpt_path.exists() || !vocab_path.exists() {
            return;
        }
        let cp = Checkpoint::load_with_device(&ckpt_path, Device::Cpu).unwrap();
        let model = CharModel::load(&cp, &vocab_path).unwrap();
        let labels = model.segment_labels("今日は晴れです").unwrap();
        assert_eq!(labels.len(), 9);
        // First label (CLS position) should be a B/I tag — argmax over 2 is 0 or 1.
        for l in &labels {
            assert!(*l < 2, "label out of range: {l}");
        }
    }
}
