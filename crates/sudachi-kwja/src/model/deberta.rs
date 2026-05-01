//! DeBERTa-v2 backbone wrapper around candle-transformers.
//!
//! Loads KWJA's checkpoint into candle's stock `DebertaV2Embeddings` +
//! `DebertaV2Encoder`. Tensor names follow KWJA's `encoder.embeddings.*`
//! and `encoder.encoder.*` layout (the outer `encoder` is WordModule's
//! field name; the inner `encoder` is DebertaV2Model's encoder).
//!
//! ## Special-token relative-position mask
//!
//! KWJA forks DebertaV2 to mask special tokens (CLS/SEP) in the relative
//! position matrix — at those rows/cols the relative position is set to
//! the sentinel `-position_buckets` so attention doesn't bucket special
//! tokens with regular text.
//!
//! We compute KWJA-style `relative_pos` ourselves and pass it via candle's
//! `DebertaV2Encoder::forward(relative_pos=Some(...))`. This bypasses
//! candle's internal `get_rel_pos` (which has no special-token mask).
//!
//! ## Conv layer
//!
//! KWJA's checkpoint includes a conv layer (kernel=3) which candle 0.8.4
//! left as `todo!()`. We patch candle locally via `[patch.crates-io]` →
//! `~/code/candle` (cloned 0.8.4 + ConvLayer.forward port from HF Python).
//! Without that patch the encoder forward panics on layer 0.

use crate::Result;
use crate::checkpoint::Checkpoint;
use crate::error::Error;
use candle_core::{DType, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::debertav2::{Config, DebertaV2Embeddings, DebertaV2Encoder};

pub struct DebertaBackbone {
    embeddings: DebertaV2Embeddings,
    encoder: DebertaV2Encoder,
    device: candle_core::Device,
    dtype: DType,
}

/// Build the candle `Config` matching KWJA's DeBERTa-v2 base. Vocab size
/// differs between word and char modules, so it's a parameter.
pub fn kwja_config(vocab_size: usize) -> Config {
    serde_json::from_value(serde_json::json!({
        "vocab_size": vocab_size,
        "hidden_size": 768,
        "num_hidden_layers": 12,
        "num_attention_heads": 12,
        "intermediate_size": 3072,
        "hidden_act": "gelu",
        "hidden_dropout_prob": 0.1,
        "attention_probs_dropout_prob": 0.1,
        "max_position_embeddings": 512,
        "type_vocab_size": 0,
        "initializer_range": 0.02,
        "relative_attention": true,
        "max_relative_positions": -1,
        "pad_token_id": 0,
        "position_biased_input": false,
        "pos_att_type": ["p2c", "c2p"],
        "position_buckets": 256,
        "share_att_key": true,
        "norm_rel_ebd": "layer_norm",
        "layer_norm_eps": 1.0e-7,
        "conv_kernel_size": 3,
        "conv_act": "gelu"
    })).expect("kwja_config: hand-built JSON parses cleanly")
}

impl DebertaBackbone {
    /// Load embeddings + encoder from a KWJA checkpoint. The outer prefix is
    /// `encoder` (WordModule's field name); embeddings live at
    /// `encoder.embeddings.*` and the encoder layers at `encoder.encoder.*`.
    ///
    /// Vocab size is read from the checkpoint's `word_embeddings.weight`
    /// shape rather than hardcoded — KWJA appends extra tokens (typo
    /// correction sentinels `<k>/<d>/<_>/<dummy>` etc.) so the actual
    /// vocab is larger than the base tokenizer's vocab.
    pub fn from_checkpoint(cp: &Checkpoint) -> Result<Self> {
        // Default dtype: F32. KWJA-Python casts both modules to F16 on CUDA
        // (see jisho-parse-py's kwja_patches.py: `cast char/word module to
        // fp16`), but candle-transformers' DeBERTa-v2 has internal F32
        // scalar additions that mismatch when the model dtype is F16,
        // producing a runtime "dtype mismatch in add" panic. Until
        // candle-transformers is patched, stick with F32. To experiment,
        // call `from_checkpoint_with_dtype(cp, DType::F16)` directly.
        Self::from_checkpoint_with_dtype(cp, DType::F32)
    }

    pub fn from_checkpoint_with_dtype(cp: &Checkpoint, dtype: DType) -> Result<Self> {
        let vocab_size = cp
            .get("encoder.embeddings.word_embeddings.weight")
            .map_err(|e| Error::Checkpoint(format!("read word_embeddings shape: {e}")))?
            .dim(0)
            .map_err(Error::from)?;

        let config = kwja_config(vocab_size);
        let vb = checkpoint_var_builder(cp, dtype)?;
        let embeddings = DebertaV2Embeddings::load(vb.pp("encoder.embeddings"), &config)
            .map_err(|e| Error::Checkpoint(format!("embeddings load: {e}")))?;
        let encoder = DebertaV2Encoder::load(vb.pp("encoder.encoder"), &config)
            .map_err(|e| Error::Checkpoint(format!("encoder load: {e}")))?;
        Ok(Self { embeddings, encoder, device: cp.device().clone(), dtype })
    }

    /// Device the model's parameter tensors live on. Inputs must match
    /// this device or candle will panic at the embedding lookup.
    pub fn device(&self) -> &candle_core::Device {
        &self.device
    }

    /// Dtype of the model's float parameter tensors. Used by callers to
    /// cast intermediate buffers (e.g. attention_mask) into the matching
    /// precision before forward.
    pub fn dtype(&self) -> DType {
        self.dtype
    }

    /// Run input_ids through embeddings + encoder. Returns hidden states of
    /// shape `(batch, seq, hidden)`.
    ///
    /// `attention_mask` is `(batch, seq)` of 1s/0s.
    /// `special_token_indices` is per-batch positions of CLS/SEP/etc. that
    /// should get the KWJA sentinel relative-position bucket. If `None`,
    /// defaults to `[0, seq_len-1]` (single sentence with CLS at start and
    /// SEP at end), which is what KWJA's data pipeline produces for the
    /// jisho path.
    pub fn forward(
        &self,
        input_ids: &Tensor,
        attention_mask: &Tensor,
        special_token_indices: Option<&[Vec<usize>]>,
    ) -> Result<Tensor> {
        let embeddings = self
            .embeddings
            .forward(Some(input_ids), None, None, Some(attention_mask), None)
            .map_err(Error::from)?;

        let (b, seq, _) = embeddings.dims3().map_err(Error::from)?;
        let default_specials: Vec<Vec<usize>> = (0..b).map(|_| vec![0, seq - 1]).collect();
        let specials = special_token_indices.unwrap_or(&default_specials);
        let rel_pos = kwja_relative_pos(seq, specials, embeddings.device())?;

        let hidden = self
            .encoder
            .forward(&embeddings, attention_mask, None, Some(&rel_pos))
            .map_err(Error::from)?;
        Ok(hidden)
    }
}

/// Build KWJA-style relative-position tensor with special-token sentinel mask.
///
/// Output shape: `(B, Q, K)` int64.
///
/// Special tokens (CLS, SEP) get position `-position_buckets` in BOTH their
/// row and their column (off-diagonal), preventing attention from bucketing
/// them with regular text. Diagonal positions are unaffected since they're
/// not in `special_token_indices`'s row + col cross product.
///
/// `position_buckets = 256` and `max_relative_positions = max_position_embeddings = 512`
/// match KWJA's base config.
fn kwja_relative_pos(
    seq_len: usize,
    special_indices: &[Vec<usize>],
    target_device: &candle_core::Device,
) -> Result<Tensor> {
    use candle_core::{D, DType};

    let position_buckets: i64 = 256;
    let max_position: i64 = 512;
    let sentinel: i64 = -position_buckets;
    let batch = special_indices.len();

    // CRITICAL: do this whole computation on CPU, then move the final
    // tensor to the target device. Two reasons:
    //   1) candle's CUDA kernel set doesn't include `uabs_i64` (signed
    //      64-bit absolute), which `make_log_bucket` needs.
    //   2) The output is tiny (Q × K, typically <40k elements) — moving
    //      one i64 tensor to GPU costs less than one CUDA-only abs kernel
    //      would have anyway.
    let device = &candle_core::Device::Cpu;

    // 1. Base relative_pos: (Q, K) where [q, k] = k - q.
    let positions = Tensor::arange(0i64, seq_len as i64, device).map_err(Error::from)?;
    let q = positions.unsqueeze(D::Minus1).map_err(Error::from)?; // (S, 1)
    let k = positions.unsqueeze(0).map_err(Error::from)?; // (1, S)
    let raw = k.broadcast_sub(&q).map_err(Error::from)?; // (S, S)

    // 2. Log-bucket the raw rel-pos values (mirror HF's make_log_bucket_position).
    let bucketed = make_log_bucket(&raw, position_buckets, max_position, device)?;

    // 3. Compute the per-row masked rel_pos.
    //
    // Fast path: when all rows share identical specials (the default case
    // hit by both single-row and batched forwards), we build just ONE (Q, K)
    // tensor and return it 2D. Candle's deberta-v2 handles 2D rel_pos via
    // its `2 => unsqueeze(0).unsqueeze(0)` branch which expands cleanly to
    // (B*num_heads, Q, K) for any batch size. Returning 3D (B, Q, K) for
    // B>1 hits a candle bug: the attention `squeeze(0)` + `expand` chain
    // assumes the leading dim is 1, and breaks for B>1 with a broadcast
    // error like `cannot broadcast [B, 1, S, S] to [B*num_heads, S, S]`.
    let all_same = special_indices
        .iter()
        .all(|s| s == &special_indices[0]);
    let build_one = |specials: &[usize]| -> Result<Tensor> {
        let mut is_special = vec![0i64; seq_len];
        for &i in specials {
            if i < seq_len {
                is_special[i] = 1;
            }
        }
        let is_special_t = Tensor::from_vec(is_special, seq_len, device).map_err(Error::from)?;
        let row_mask = is_special_t
            .unsqueeze(D::Minus1)
            .map_err(Error::from)?
            .broadcast_as((seq_len, seq_len))
            .map_err(Error::from)?;
        let col_mask = is_special_t
            .unsqueeze(0)
            .map_err(Error::from)?
            .broadcast_as((seq_len, seq_len))
            .map_err(Error::from)?;
        let mask_i64 = row_mask.add(&col_mask).map_err(Error::from)?;
        let mask_bool = mask_i64.gt(0i64).map_err(Error::from)?;

        let sentinel_t = Tensor::full(sentinel, (seq_len, seq_len), device).map_err(Error::from)?;
        let masked = mask_bool
            .where_cond(&sentinel_t, &bucketed)
            .map_err(Error::from)?
            .to_dtype(DType::I64)
            .map_err(Error::from)?;
        Ok(masked)
    };

    if all_same {
        // 2D (Q, K): candle expands across batch + heads itself.
        let cpu_t = build_one(&special_indices[0])?;
        return cpu_t.to_device(target_device).map_err(Error::from);
    }

    // Heterogeneous specials per row — fall back to per-row build + stack.
    // Note: this 3D path exposes a candle deberta-v2 attention bug at B>1;
    // it works fine at B=1. Heterogeneous specials are opt-in (caller
    // must explicitly pass `Some(&[Vec<usize>; B])` to the backbone) so
    // we leave the 3D behavior unchanged.
    let mut per_batch = Vec::with_capacity(batch);
    for specials in special_indices {
        per_batch.push(build_one(specials)?);
    }
    let cpu_t = Tensor::stack(&per_batch, 0).map_err(Error::from)?;
    cpu_t.to_device(target_device).map_err(Error::from)
}

/// Port of HF transformers' `make_log_bucket_position`. Returns int64.
fn make_log_bucket(
    relative_pos: &Tensor,
    bucket_size: i64,
    max_position: i64,
    device: &candle_core::Device,
) -> Result<Tensor> {
    use candle_core::DType;

    let mid = bucket_size / 2;
    let abs_pos = relative_pos.abs().map_err(Error::from)?;

    // condition = (rel_pos < mid) AND (rel_pos > -mid)
    let lt_mid = relative_pos.lt(mid).map_err(Error::from)?;
    let gt_neg_mid = relative_pos.gt(-mid).map_err(Error::from)?;
    let cond = lt_mid
        .to_dtype(DType::U8)
        .map_err(Error::from)?
        .mul(&gt_neg_mid.to_dtype(DType::U8).map_err(Error::from)?)
        .map_err(Error::from)?
        .gt(0u8)
        .map_err(Error::from)?;

    // Where in the small-distance range, output (mid - 1).
    let on_true = Tensor::full(mid - 1, relative_pos.shape(), device).map_err(Error::from)?;

    // Otherwise log-bucket: floor(log(abs_pos / mid) / log((max-1) / mid) * (bucket - mid)) + mid
    let abs_f32 = abs_pos.to_dtype(DType::F32).map_err(Error::from)?;
    let mid_f = mid as f32;
    let max_f = max_position as f32;
    let bucket_f = bucket_size as f32;
    let scale = ((max_f - 1.0) / mid_f).ln();
    let log_term = abs_f32
        .div(&Tensor::full(mid_f, abs_f32.shape(), device).map_err(Error::from)?)
        .map_err(Error::from)?
        .log()
        .map_err(Error::from)?
        .div(&Tensor::full(scale, abs_f32.shape(), device).map_err(Error::from)?)
        .map_err(Error::from)?;
    let on_false_f = log_term
        .mul(&Tensor::full(bucket_f - mid_f, abs_f32.shape(), device).map_err(Error::from)?)
        .map_err(Error::from)?
        .floor()
        .map_err(Error::from)?
        .add(&Tensor::full(mid_f, abs_f32.shape(), device).map_err(Error::from)?)
        .map_err(Error::from)?;

    // Sign-back: result = on_false_f * sign(rel_pos) (or on_true if cond)
    let sign_f = relative_pos
        .to_dtype(DType::F32)
        .map_err(Error::from)?
        .sign()
        .map_err(Error::from)?;
    let signed = on_false_f.mul(&sign_f).map_err(Error::from)?;
    let on_false = signed.to_dtype(DType::I64).map_err(Error::from)?;

    cond.where_cond(&on_true, &on_false).map_err(Error::from)
}

/// Build a VarBuilder backed by all tensors in the checkpoint. Materializes
/// every tensor up front (one-time cost at Pipeline construction). The
/// VarBuilder casts loaded tensors to `dtype` — pass F16 to match
/// KWJA-Python's CUDA inference path (its kwja_patches.py casts both
/// modules to fp16 on GPU), F32 for portability.
pub(crate) fn checkpoint_var_builder(cp: &Checkpoint, dtype: DType) -> Result<VarBuilder<'static>> {
    let mut tensors = std::collections::HashMap::new();
    for name in cp.tensor_names() {
        let mut t = cp.get(&name)?;
        // Pre-cast bool/u8/i32/i64 stays as-is (some buffers in the
        // checkpoint are masks/indices). Float tensors get the requested
        // dtype.
        match t.dtype() {
            DType::F32 | DType::F16 | DType::BF16 | DType::F64 => {
                if t.dtype() != dtype {
                    t = t.to_dtype(dtype).map_err(Error::from)?;
                }
            }
            _ => {}
        }
        tensors.insert(name.clone(), t);
    }
    Ok(VarBuilder::from_tensors(tensors, dtype, cp.device()))
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
    fn loads_word_backbone() {
        let path = home().join(".local/share/jisho/checkpoints/word.safetensors");
        if !path.exists() {
            eprintln!("skipping: {path:?} not present");
            return;
        }
        let cp = Checkpoint::load_with_device(&path, Device::Cpu).unwrap();
        let backbone = DebertaBackbone::from_checkpoint(&cp)
            .expect("word backbone should load");
        // Smoke-test forward: tiny synthetic input — CLS, two tokens, SEP.
        let input_ids = Tensor::new(&[[1u32, 100, 200, 2]], &Device::Cpu).unwrap();
        let attention_mask = Tensor::ones((1, 4), DType::U32, &Device::Cpu).unwrap();
        let hidden = backbone.forward(&input_ids, &attention_mask, None).unwrap();
        assert_eq!(hidden.dims(), &[1, 4, crate::constants::HIDDEN_SIZE]);
    }

    #[test]
    fn loads_char_backbone() {
        let path = home().join(".local/share/jisho/checkpoints/char.safetensors");
        if !path.exists() {
            eprintln!("skipping: {path:?} not present");
            return;
        }
        let cp = Checkpoint::load_with_device(&path, Device::Cpu).unwrap();
        let backbone = DebertaBackbone::from_checkpoint(&cp)
            .expect("char backbone should load");
        let input_ids = Tensor::new(&[[1u32, 100, 200, 2]], &Device::Cpu).unwrap();
        let attention_mask = Tensor::ones((1, 4), DType::U32, &Device::Cpu).unwrap();
        let hidden = backbone.forward(&input_ids, &attention_mask, None).unwrap();
        assert_eq!(hidden.dims(), &[1, 4, crate::constants::HIDDEN_SIZE]);
    }
}
