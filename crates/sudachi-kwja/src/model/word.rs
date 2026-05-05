//! Word module: POS, subpos, conjtype, conjform, reading, NER, and dependency parse.
//!
//! KWJA's WordModule has 13 output heads. v0.1 ships the 7 heads jisho needs
//! for the proto's `Morpheme` + `BasePhrase` structures:
//!   - reading_tagger     (operates on subwords, not pooled)
//!   - pos_tagger
//!   - subpos_tagger
//!   - conjtype_tagger
//!   - conjform_tagger
//!   - ne_tagger
//!   - dependency_parser  (biaffine; produces (B, S, S, 1) scores)
//!
//! Out of v0.1 scope: word_feature_tagger, base_phrase_feature_tagger
//! (LoRA heads — needed for richer BasePhrase features), dependency_type_parser,
//! cohesion_analyzer, discourse_relation_analyzer.
//!
//! Forward path:
//!   text → DebertaTokenizer → backbone → reading_logits (subwords)
//!                                     → pool_subwords → pooled
//!                                     → pos/subpos/conjtype/conjform/ne_logits
//!                                     → dependency_scores (biaffine)
//!
//! WARNING: KWJA's WordModule.forward expects `special_token_indices`,
//! `subword_map`, `dependency_mask`, `dependency_labels`, `cohesion_mask`
//! batch keys constructed by its WordDataset. v0.1 does NOT replicate the
//! datamodule logic — we pass stock attention through candle's encoder.
//! This means argmax may differ from KWJA-Python at sentence boundaries
//! (where the special-token mask matters). E2E equivalence at Task 22 will
//! quantify the divergence; if too large we re-add the mask via candle's
//! `relative_pos` parameter.

use crate::Result;
use crate::checkpoint::Checkpoint;
use crate::error::Error;
use crate::model::deberta::{DebertaBackbone, checkpoint_var_builder};
use crate::model::heads::{
    BiaffineDependencyHead, LoRARelationWiseWordSelectionHead, LoRASequenceMultiLabelingHead,
    SequentialMlpHead, WordSelectionHead,
};
use crate::model::pool::pool_subwords;
use crate::tokenizer::{DebertaTokenizer, Encoded};
use candle_core::Tensor;
use std::path::Path;

pub struct WordModel {
    tokenizer: DebertaTokenizer,
    backbone: DebertaBackbone,
    reading_tagger: SequentialMlpHead,
    pos_tagger: SequentialMlpHead,
    subpos_tagger: SequentialMlpHead,
    conjtype_tagger: SequentialMlpHead,
    conjform_tagger: SequentialMlpHead,
    ne_tagger: SequentialMlpHead,
    dependency_parser: BiaffineDependencyHead,
    dependency_type_parser: SequentialMlpHead,
    word_feature_tagger: LoRASequenceMultiLabelingHead,
    base_phrase_feature_tagger: LoRASequenceMultiLabelingHead,
    cohesion_analyzer: LoRARelationWiseWordSelectionHead,
    discourse_relation_analyzer: WordSelectionHead,
}

/// All logits produced by WordModel.forward, named to match KWJA's emit dict.
pub struct WordLogits {
    /// (1, T_subwords, num_reading_classes) — KWJA reading classifier
    /// operates on subwords; classes are reading-prediction targets.
    pub reading_logits: Tensor,
    /// (1, T_words, 14)
    pub pos_logits: Tensor,
    /// (1, T_words, 35)
    pub subpos_logits: Tensor,
    /// (1, T_words, 33)
    pub conjtype_logits: Tensor,
    /// (1, T_words, 81)
    pub conjform_logits: Tensor,
    /// (1, T_words, 17)
    pub ne_logits: Tensor,
    /// (1, T_words, T_words, 1) — pairwise dependency scores.
    pub dependency_scores: Tensor,
    /// (1, T_words, num_dep_types) — per-word dep_type prediction.
    /// Argmax over the trailing dim gives the dep_type label index for each
    /// word; the BasePhrase-level dep_type is read at the BP's head morpheme.
    pub dependency_type_logits: Tensor,
    /// (1, T_words, num_word_features) — per-word multi-label sigmoid
    /// probabilities for KWJA's word_feature_tagger output.
    pub word_feature_probs: Tensor,
    /// (1, T_words, num_base_phrase_features) — per-word multi-label
    /// sigmoid probabilities for KWJA's base_phrase_feature_tagger.
    /// The BP-level features come from the BP's head morpheme.
    pub bp_feature_probs: Tensor,
    /// (1, T_words, T_words, num_cohesion_relations) — pairwise PAS /
    /// bridging / coreference scores. Argmax along the target axis (per
    /// source word, per relation type) gives the predicted target word.
    pub cohesion_logits: Tensor,
    /// (1, T_words, T_words, num_discourse_relations) — pairwise discourse
    /// scores across the WHOLE encoded element (multi-sentence input).
    /// Cross-sentence discourse decoding (Phase D follow-up) iterates
    /// predicate BPs across decoded Sentences and looks up
    /// discourse_logits[head_word_i, head_word_j] for cross-sentence
    /// pairs.
    pub discourse_logits: Tensor,
    /// Encoded subword info (offsets, word_ids) for callers that need to
    /// map argmax positions back to text.
    pub encoded: Encoded,
    /// Number of words after pooling — equals max(word_id)+1.
    pub num_words: usize,
}

impl WordModel {
    pub fn load(checkpoint: &Checkpoint, tokenizer_path: &Path) -> Result<Self> {
        let tokenizer = DebertaTokenizer::load(tokenizer_path)?;
        let backbone = DebertaBackbone::from_checkpoint(checkpoint)?;
        let vb = checkpoint_var_builder(checkpoint, backbone.dtype())?;

        let labels = &*crate::constants::LABELS;

        // Reading-tagger num classes is read from checkpoint shape (KWJA
        // doesn't expose it as a constant — reading targets are dataset-
        // specific). Inspect the head's output linear weight to learn the
        // count.
        let reading_classes = checkpoint
            .get("reading_tagger.3.weight")?
            .dim(0)
            .map_err(Error::from)?;

        let h = crate::constants::HIDDEN_SIZE;
        Ok(Self {
            tokenizer,
            backbone,
            reading_tagger: SequentialMlpHead::from_var_builder(
                vb.pp("reading_tagger"), h, reading_classes,
            )?,
            pos_tagger: SequentialMlpHead::from_var_builder(
                vb.pp("pos_tagger"), h, labels.pos.len(),
            )?,
            subpos_tagger: SequentialMlpHead::from_var_builder(
                vb.pp("subpos_tagger"), h, labels.subpos.len(),
            )?,
            conjtype_tagger: SequentialMlpHead::from_var_builder(
                vb.pp("conjtype_tagger"), h, labels.conjtype.len(),
            )?,
            conjform_tagger: SequentialMlpHead::from_var_builder(
                vb.pp("conjform_tagger"), h, labels.conjform.len(),
            )?,
            ne_tagger: SequentialMlpHead::from_var_builder(
                vb.pp("ne_tagger"), h, labels.ne.len(),
            )?,
            dependency_parser: BiaffineDependencyHead::from_var_builder(
                vb.pp("dependency_parser"), h, 1, /* with_bias */ false,
            )?,
            // dep_type_parser is unusual: KWJA wires it to take CONCATENATED
            // (source_hidden, head_hidden) for each predicted dependency edge —
            // input dim = 2 * hidden (1536), not hidden (768). We load with
            // the right shape; the proper forward (gather head states from
            // predicted dep_parents → concat → MLP) is a follow-up. For now
            // we don't call its forward in the batched path; dep_type falls
            // back to the per-BP default.
            dependency_type_parser: SequentialMlpHead::from_var_builder(
                vb.pp("dependency_type_parser"), 2 * h, labels.dependency_types.len(),
            )?,
            word_feature_tagger: LoRASequenceMultiLabelingHead::from_var_builder(
                vb.pp("word_feature_tagger"), h, labels.word_features.len(), 4,
            )?,
            base_phrase_feature_tagger: LoRASequenceMultiLabelingHead::from_var_builder(
                vb.pp("base_phrase_feature_tagger"), h, labels.base_phrase_features.len(), 4,
            )?,
            // cohesion uses rank=1 (KWJA hparam); word/bp_feature use rank=4.
            // See kwja/modules/word.py: LoRARelationWiseWordSelectionHead(...,
            // rank=1, ...).
            cohesion_analyzer: LoRARelationWiseWordSelectionHead::from_var_builder(
                vb.pp("cohesion_analyzer"), h, labels.cohesion_relations.len(), 1,
            )?,
            // discourse output_layer has NO bias, same as dependency_parser.
            // Confirmed by inspecting the checkpoint:
            //   discourse_relation_analyzer.output_layer.weight (7, 768) — no .bias
            discourse_relation_analyzer: WordSelectionHead::from_var_builder(
                vb.pp("discourse_relation_analyzer"),
                h,
                labels.discourse_relations.len(),
                /* with_bias */ false,
            )?,
        })
    }

    /// Forward on pre-tokenized input — words come from Sudachi (or any
    /// upstream tokenizer). Each word becomes a "word" in the HF tokenizer's
    /// view, so `word_ids` align 1:1 with the input word indices. This is
    /// the path KWJA-Python uses (Juman++ pretokenizes); without it, HF's
    /// tokenizer puts every Japanese char into a single word group and
    /// pool_subwords averages everything together.
    ///
    /// Thin wrapper around `forward_pretokenized_batch` for the single-row
    /// case. Prefer the batch variant when you have multiple sentences —
    /// it runs ONE GPU forward instead of N, which is the difference between
    /// rs being faster than KWJA-Python and rs being slower at large batches.
    pub fn forward_pretokenized(&self, words: &[&str]) -> Result<WordLogits> {
        let owned: Vec<&str> = words.to_vec();
        let mut batched = self.forward_pretokenized_batch(&[owned])?;
        Ok(batched.remove(0))
    }

    /// Batched forward over multiple pre-tokenized inputs.
    ///
    /// Each row of `batches` is a sentence's worth of word surfaces. This
    /// runs a single backbone forward on the whole batch (padded to the
    /// longest row), then per-row slicing. Match KWJA-Python's
    /// `predict_step` batching shape — one (B, T_max, H) tensor through
    /// the encoder, then per-head linear ops.
    pub fn forward_pretokenized_batch(&self, batches: &[Vec<&str>]) -> Result<Vec<WordLogits>> {
        if batches.is_empty() {
            return Ok(vec![]);
        }
        let device = self.backbone.device().clone();
        let encodings = self.tokenizer.encode_batch_pretokenized(batches)?;
        let b = encodings.len();
        let t_max = encodings.iter().map(|e| e.input_ids.len()).max().unwrap();

        let pad_id = crate::tokenizer::deberta::DebertaTokenizer::PAD_TOKEN_ID;
        let mut input_ids_flat: Vec<u32> = Vec::with_capacity(b * t_max);
        let mut attn_flat: Vec<u32> = Vec::with_capacity(b * t_max);
        let mut word_ids_padded: Vec<Vec<Option<u32>>> = Vec::with_capacity(b);
        let mut num_words_per: Vec<usize> = Vec::with_capacity(b);

        for enc in &encodings {
            let pad_len = t_max - enc.input_ids.len();
            input_ids_flat.extend_from_slice(&enc.input_ids);
            input_ids_flat.extend(std::iter::repeat_n(pad_id, pad_len));
            attn_flat.extend_from_slice(&enc.attention_mask);
            attn_flat.extend(std::iter::repeat_n(0u32, pad_len));
            let mut wids = enc.word_ids.clone();
            wids.extend(std::iter::repeat_n(None, pad_len));
            let nw = wids
                .iter()
                .filter_map(|w| *w)
                .max()
                .map(|n| n as usize + 1)
                .unwrap_or(0);
            word_ids_padded.push(wids);
            num_words_per.push(nw);
        }

        let w_max = num_words_per.iter().copied().max().unwrap_or(0);
        let input_ids = Tensor::from_vec(input_ids_flat, (b, t_max), &device).map_err(Error::from)?;
        let attn = Tensor::from_vec(attn_flat, (b, t_max), &device).map_err(Error::from)?;
        let attn_f32 = attn.to_dtype(self.backbone.dtype()).map_err(Error::from)?;

        let hidden = self.backbone.forward(&input_ids, &attn_f32, None)?;
        let reading_logits = self.reading_tagger.forward(&hidden)?;

        let pooled = if w_max == 0 {
            Tensor::zeros((b, 0, crate::constants::HIDDEN_SIZE), self.backbone.dtype(), &device)
                .map_err(Error::from)?
        } else {
            pool_subwords(&hidden, &word_ids_padded, w_max)?
        };

        let pos = self.pos_tagger.forward(&pooled)?;
        let subpos = self.subpos_tagger.forward(&pooled)?;
        let conjtype = self.conjtype_tagger.forward(&pooled)?;
        let conjform = self.conjform_tagger.forward(&pooled)?;
        let ne = self.ne_tagger.forward(&pooled)?;
        let dep = self.dependency_parser.forward(&pooled)?;
        // dep_type_parser: KWJA conditions dep_type on the predicted edge.
        //   1. dep argmax (B, W) — best parent per source.
        //   2. gather pooled[batch, parent_idx] → head_hidden (B, W, H).
        //   3. concat source + head along H → (B, W, 2H).
        //   4. MLP → (B, W, num_dep_types).
        let dep_type = if w_max > 0 {
            let dep_squeezed = dep.squeeze(3).map_err(Error::from)?; // (B, W, W)
            let dep_argmax = dep_squeezed.argmax(2).map_err(Error::from)?; // (B, W) u32
            // Broadcast argmax to (B, W, H) and gather along the W axis.
            let h = pooled.dim(2).map_err(Error::from)?;
            let argmax_3d = dep_argmax
                .unsqueeze(2)
                .map_err(Error::from)?
                .broadcast_as((b, w_max, h))
                .map_err(Error::from)?
                .contiguous()
                .map_err(Error::from)?;
            let head_hidden = pooled.gather(&argmax_3d, 1).map_err(Error::from)?;
            // (B, W, 2H) — KWJA uses concat order [source, head].
            let combined = Tensor::cat(&[&pooled, &head_hidden], 2).map_err(Error::from)?;
            self.dependency_type_parser.forward(&combined)?
        } else {
            Tensor::zeros(
                (b, 0, crate::constants::LABELS.dependency_types.len()),
                self.backbone.dtype(),
                &device,
            )
            .map_err(Error::from)?
        };
        let word_feature_probs = self.word_feature_tagger.forward(&pooled)?;
        let bp_feature_probs = self.base_phrase_feature_tagger.forward(&pooled)?;
        let cohesion_logits = self.cohesion_analyzer.forward(&pooled)?;
        let discourse_logits = self.discourse_relation_analyzer.forward(&pooled)?;

        // Per-row slice: each row's logits at indices 0..num_words_per[i] are valid;
        // the rest is zero-init padding from pool_subwords.
        let mut out = Vec::with_capacity(b);
        for (i, enc) in encodings.into_iter().enumerate() {
            let nw = num_words_per[i];
            let t_actual = enc.input_ids.len();
            out.push(WordLogits {
                reading_logits: subwords_slice(&reading_logits, i, t_actual)?,
                pos_logits: pooled_slice(&pos, i, nw)?,
                subpos_logits: pooled_slice(&subpos, i, nw)?,
                conjtype_logits: pooled_slice(&conjtype, i, nw)?,
                conjform_logits: pooled_slice(&conjform, i, nw)?,
                ne_logits: pooled_slice(&ne, i, nw)?,
                dependency_scores: pairwise_slice(&dep, i, nw)?,
                dependency_type_logits: pooled_slice(&dep_type, i, nw)?,
                word_feature_probs: pooled_slice(&word_feature_probs, i, nw)?,
                bp_feature_probs: pooled_slice(&bp_feature_probs, i, nw)?,
                cohesion_logits: cohesion_pairwise_slice(&cohesion_logits, i, nw)?,
                discourse_logits: cohesion_pairwise_slice(&discourse_logits, i, nw)?,
                encoded: enc,
                num_words: nw,
            });
        }
        Ok(out)
    }
}

impl WordLogits {
    /// Slice the word axis to `[start..end]`. Used by
    /// `decode_element_from_logits` to decode each per-sentence chunk
    /// from a single multi-sentence forward.
    ///
    /// Tensors split:
    ///   - pos / subpos / conjtype / conjform / ne / dep_type /
    ///     word_feature / bp_feature  (1, W, ...): narrow dim 1
    ///   - dependency_scores / cohesion_logits  (1, W, W, R): narrow dim 1 + 2
    ///   - discourse_logits: kept WHOLE — discourse is intentionally
    ///     cross-sentence; the decoder reads from the full tensor at
    ///     document level.
    ///   - reading_logits / encoded: kept whole; subword-aligned, not
    ///     trivially per-word sliceable. Reading is passed through from
    ///     Sudachi anyway.
    pub fn slice_word_axis(&self, start: usize, end: usize) -> Result<Self> {
        let nw = end - start;
        let dt = self.pos_logits.dtype();
        let _ = dt;

        let p1 = |t: &Tensor| -> Result<Tensor> {
            t.narrow(1, start, nw).map_err(Error::from)
        };
        let p2 = |t: &Tensor| -> Result<Tensor> {
            // (1, W, W, _): narrow both word axes 1 and 2.
            let a = t.narrow(1, start, nw).map_err(Error::from)?;
            a.narrow(2, start, nw).map_err(Error::from)
        };
        Ok(WordLogits {
            // Reading is kept as-is; we don't decode it per-sentence.
            reading_logits: self.reading_logits.clone(),
            pos_logits: p1(&self.pos_logits)?,
            subpos_logits: p1(&self.subpos_logits)?,
            conjtype_logits: p1(&self.conjtype_logits)?,
            conjform_logits: p1(&self.conjform_logits)?,
            ne_logits: p1(&self.ne_logits)?,
            dependency_scores: p2(&self.dependency_scores)?,
            dependency_type_logits: p1(&self.dependency_type_logits)?,
            word_feature_probs: p1(&self.word_feature_probs)?,
            bp_feature_probs: p1(&self.bp_feature_probs)?,
            cohesion_logits: p2(&self.cohesion_logits)?,
            // Discourse stays whole — cross-sentence decoding reads
            // from this at document level.
            discourse_logits: self.discourse_logits.clone(),
            encoded: self.encoded.clone(),
            num_words: nw,
        })
    }
}

impl WordModel {
    /// Raw-text path: HF tokenizer assigns word_ids automatically (one
    /// word per HF "word group"). For Japanese this often degenerates
    /// (everything → word 0). Prefer `forward_pretokenized` with Sudachi
    /// surfaces; this method exists for KWJA's classic char-pretokenize
    /// path and is exercised by the smoke test.
    pub fn forward(&self, text: &str) -> Result<WordLogits> {
        let encoded = self.tokenizer.encode(text)?;
        let num_words = encoded
            .word_ids
            .iter()
            .filter_map(|w| *w)
            .max()
            .map(|n| n as usize + 1)
            .unwrap_or(0);
        self.forward_from_encoded(encoded, num_words)
    }

    fn forward_from_encoded(&self, encoded: Encoded, num_words: usize) -> Result<WordLogits> {
        // Single-row path used only by `forward(text)`. Pretokenized callers
        // go through forward_pretokenized_batch directly.
        let device = self.backbone.device().clone();
        let input_ids = Tensor::new(vec![encoded.input_ids.clone()], &device).map_err(Error::from)?;
        let attention_mask = Tensor::new(vec![encoded.attention_mask.clone()], &device)
            .map_err(Error::from)?;
        let attention_mask_f32 = attention_mask
            .to_dtype(self.backbone.dtype())
            .map_err(Error::from)?;

        let hidden_subwords = self.backbone.forward(&input_ids, &attention_mask_f32, None)?;
        let reading_logits = self.reading_tagger.forward(&hidden_subwords)?;

        let word_ids_per_batch = vec![encoded.word_ids.clone()];
        let pooled = if num_words == 0 {
            Tensor::zeros((1, 0, crate::constants::HIDDEN_SIZE), self.backbone.dtype(), &device)
                .map_err(Error::from)?
        } else {
            pool_subwords(&hidden_subwords, &word_ids_per_batch, num_words)?
        };

        let pos_logits = self.pos_tagger.forward(&pooled)?;
        let subpos_logits = self.subpos_tagger.forward(&pooled)?;
        let conjtype_logits = self.conjtype_tagger.forward(&pooled)?;
        let conjform_logits = self.conjform_tagger.forward(&pooled)?;
        let ne_logits = self.ne_tagger.forward(&pooled)?;
        let dependency_scores = self.dependency_parser.forward(&pooled)?;
        // dep_type from concat(source, head) — see forward_pretokenized_batch.
        let dependency_type_logits = if num_words > 0 {
            let dep_sq = dependency_scores.squeeze(3).map_err(Error::from)?; // (1, W, W)
            let dep_argmax = dep_sq.argmax(2).map_err(Error::from)?; // (1, W)
            let h = pooled.dim(2).map_err(Error::from)?;
            let argmax_3d = dep_argmax
                .unsqueeze(2)
                .map_err(Error::from)?
                .broadcast_as((1, num_words, h))
                .map_err(Error::from)?
                .contiguous()
                .map_err(Error::from)?;
            let head_hidden = pooled.gather(&argmax_3d, 1).map_err(Error::from)?;
            let combined = Tensor::cat(&[&pooled, &head_hidden], 2).map_err(Error::from)?;
            self.dependency_type_parser.forward(&combined)?
        } else {
            Tensor::zeros(
                (1, 0, crate::constants::LABELS.dependency_types.len()),
                self.backbone.dtype(),
                &device,
            )
            .map_err(Error::from)?
        };
        let word_feature_probs = self.word_feature_tagger.forward(&pooled)?;
        let bp_feature_probs = self.base_phrase_feature_tagger.forward(&pooled)?;
        let cohesion_logits = self.cohesion_analyzer.forward(&pooled)?;
        let discourse_logits = self.discourse_relation_analyzer.forward(&pooled)?;

        Ok(WordLogits {
            reading_logits,
            pos_logits,
            subpos_logits,
            conjtype_logits,
            conjform_logits,
            ne_logits,
            dependency_scores,
            dependency_type_logits,
            word_feature_probs,
            bp_feature_probs,
            cohesion_logits,
            discourse_logits,
            encoded,
            num_words,
        })
    }
}

// Per-row slice helpers for batched forward outputs. Each takes a (B, ...)
// tensor and returns a (1, ...) view covering one row's valid (un-padded)
// region. The hidden_subwords path uses `subwords_slice` (T cropped to the
// row's actual subword count); the pooled path uses `pooled_slice` (W
// cropped to the row's actual word count); pairwise ops use `pairwise_slice`.

fn subwords_slice(t: &Tensor, row: usize, t_actual: usize) -> Result<Tensor> {
    let r = t.narrow(0, row, 1).map_err(Error::from)?;
    r.narrow(1, 0, t_actual).map_err(Error::from)
}

fn pooled_slice(t: &Tensor, row: usize, nw: usize) -> Result<Tensor> {
    let r = t.narrow(0, row, 1).map_err(Error::from)?;
    r.narrow(1, 0, nw).map_err(Error::from)
}

fn pairwise_slice(t: &Tensor, row: usize, nw: usize) -> Result<Tensor> {
    let r = t.narrow(0, row, 1).map_err(Error::from)?;
    let r1 = r.narrow(1, 0, nw).map_err(Error::from)?;
    r1.narrow(2, 0, nw).map_err(Error::from)
}

/// Per-row slice for relation-wise pairwise heads: (B, S, S, R) → (1, nw, nw, R).
/// The trailing relation axis is preserved in full (R is the same for all rows).
fn cohesion_pairwise_slice(t: &Tensor, row: usize, nw: usize) -> Result<Tensor> {
    let r = t.narrow(0, row, 1).map_err(Error::from)?;
    let r1 = r.narrow(1, 0, nw).map_err(Error::from)?;
    r1.narrow(2, 0, nw).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use std::path::PathBuf;

    fn home() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap())
    }

    #[test]
    fn word_model_loads_and_forwards() {
        let ckpt_path = home().join(".local/share/jisho/checkpoints/word.safetensors");
        let tokenizer_path = home().join(".local/share/jisho/checkpoints/kwja-tokenizer/tokenizer.json");
        if !ckpt_path.exists() || !tokenizer_path.exists() {
            eprintln!("skipping: word.safetensors or tokenizer.json missing");
            return;
        }
        let cp = Checkpoint::load_with_device(&ckpt_path, Device::Cpu).unwrap();
        let model = WordModel::load(&cp, &tokenizer_path).unwrap();
        let out = model.forward("今日は晴れです").unwrap();

        assert_eq!(out.pos_logits.dims().len(), 3);
        assert_eq!(out.pos_logits.dim(0).unwrap(), 1);
        assert_eq!(*out.pos_logits.dims().last().unwrap(), 14);
        assert_eq!(*out.subpos_logits.dims().last().unwrap(), 35);
        assert_eq!(*out.conjtype_logits.dims().last().unwrap(), 33);
        assert_eq!(*out.conjform_logits.dims().last().unwrap(), 81);
        assert_eq!(*out.ne_logits.dims().last().unwrap(), 17);

        // Dependency scores are pairwise word-level: (1, W, W, 1).
        assert_eq!(out.dependency_scores.dims().len(), 4);
        assert_eq!(out.dependency_scores.dim(0).unwrap(), 1);
        assert_eq!(out.dependency_scores.dim(1).unwrap(), out.num_words);
        assert_eq!(out.dependency_scores.dim(2).unwrap(), out.num_words);
        assert_eq!(out.dependency_scores.dim(3).unwrap(), 1);
    }
}
