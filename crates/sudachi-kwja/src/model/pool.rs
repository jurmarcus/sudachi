//! Subword pooling — collapse subword hidden states into word-level states.
//!
//! KWJA's WordModule receives subword-tokenized input from DeBERTa (HF
//! tokenizer) but emits one tag per word. `pool_subwords` averages subword
//! hidden states for each word group, with words identified by `word_ids`
//! (None for special tokens like CLS/SEP).
//!
//! Pooling strategy: MEAN. KWJA upstream uses `PoolingStrategy.FIRST` (take
//! first subword) for some heads — switch to `mean` here matches the
//! per-stage equivalence test design (caller can choose). v0.1 uses MEAN
//! since it's the default in our forward path.

use crate::Result;
use crate::error::Error;
use candle_core::{Device, Tensor};

/// Pool subword hidden states into word-level hidden states.
///
/// Args:
/// - `hidden`: (B, S, H) subword hidden states (output of DeBERTa encoder).
/// - `word_ids`: Vec of length B; each inner Vec has length S. `Some(w)`
///   means subword belongs to word `w`; `None` means special token (skipped).
/// - `num_words`: number of words to pool to per batch element.
///
/// Returns:
/// - (B, num_words, H) pooled hidden states.
///
/// Materializes hidden to host once. For inference batches in the 1-100
/// sentence range this is fine. If profiled hot, swap for an `index_add` /
/// scatter-style GPU kernel.
pub fn pool_subwords(
    hidden: &Tensor,
    word_ids: &[Vec<Option<u32>>],
    num_words: usize,
) -> Result<Tensor> {
    let (b, s, h) = hidden.dims3().map_err(Error::from)?;
    if word_ids.len() != b {
        return Err(Error::InvalidInput(format!(
            "word_ids batch dim {} != hidden batch dim {b}",
            word_ids.len()
        )));
    }
    let device = hidden.device().clone();
    // pool_subwords does its arithmetic in f32 (even for fp16 backbones)
    // because the candle to_vec3 path is dtype-specific and the averaging
    // benefits from f32 precision. Result is cast back to the source dtype
    // at the end to avoid up-casting downstream tensors.
    let original_dtype = hidden.dtype();
    let hidden_f32 = if original_dtype == candle_core::DType::F32 {
        hidden.clone()
    } else {
        hidden.to_dtype(candle_core::DType::F32).map_err(Error::from)?
    };
    let hidden_vec: Vec<Vec<Vec<f32>>> = hidden_f32
        .to_vec3::<f32>()
        .map_err(Error::from)?;

    let mut out = Vec::with_capacity(b);
    for batch_idx in 0..b {
        if word_ids[batch_idx].len() != s {
            return Err(Error::InvalidInput(format!(
                "word_ids[{batch_idx}].len() = {} != hidden seq dim {s}",
                word_ids[batch_idx].len()
            )));
        }
        let mut sums: Vec<Vec<f32>> = vec![vec![0.0; h]; num_words];
        let mut counts: Vec<u32> = vec![0; num_words];

        for (sub_idx, &wid) in word_ids[batch_idx].iter().enumerate() {
            let Some(w) = wid else { continue };
            let w = w as usize;
            if w >= num_words {
                continue;
            }
            for d in 0..h {
                sums[w][d] += hidden_vec[batch_idx][sub_idx][d];
            }
            counts[w] += 1;
        }

        let mut means = Vec::with_capacity(num_words * h);
        for w in 0..num_words {
            let c = counts[w].max(1) as f32;
            for d in 0..h {
                means.push(sums[w][d] / c);
            }
        }
        out.push(Tensor::from_vec(means, (num_words, h), &device).map_err(Error::from)?);
    }

    let stacked = Tensor::stack(&out, 0).map_err(Error::from)?;
    if original_dtype == candle_core::DType::F32 {
        Ok(stacked)
    } else {
        stacked.to_dtype(original_dtype).map_err(Error::from)
    }
}

/// Convenience: build word_ids from an iterator of `Option<u32>`. Returns an
/// owned `Vec<Vec<Option<u32>>>` so it can outlive the iterator's source.
pub fn collect_word_ids<'a, I>(per_batch: I) -> Vec<Vec<Option<u32>>>
where
    I: IntoIterator<Item = &'a [Option<u32>]>,
{
    per_batch.into_iter().map(|s| s.to_vec()).collect()
}

#[allow(dead_code)]
fn _device_doc(_d: &Device) {}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    #[test]
    fn pools_three_words_from_five_subwords() {
        // Hidden: 5 subwords, dim 4
        let hidden = Tensor::new(
            &[[
                [1.0f32, 2.0, 3.0, 4.0],   // word 0, subword 0
                [2.0, 3.0, 4.0, 5.0],      // word 0, subword 1
                [10.0, 20.0, 30.0, 40.0],  // word 1
                [100.0, 200.0, 300.0, 400.0],
                [200.0, 300.0, 400.0, 500.0],
            ]],
            &Device::Cpu,
        )
        .unwrap();
        let word_ids = vec![vec![Some(0u32), Some(0), Some(1), Some(2), Some(2)]];
        let pooled = pool_subwords(&hidden, &word_ids, 3).unwrap();
        assert_eq!(pooled.dims(), &[1, 3, 4]);
        let v = pooled.to_vec3::<f32>().unwrap();
        assert!((v[0][0][0] - 1.5).abs() < 1e-5); // mean of 1.0, 2.0
        assert!((v[0][1][0] - 10.0).abs() < 1e-5);
        assert!((v[0][2][0] - 150.0).abs() < 1e-5);
    }

    #[test]
    fn special_tokens_are_skipped() {
        let hidden = Tensor::new(
            &[[[1.0f32, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0]]],
            &Device::Cpu,
        )
        .unwrap();
        // word_ids: CLS, word0, word1, SEP — only middle two contribute.
        let word_ids = vec![vec![None, Some(0u32), Some(1), None]];
        let pooled = pool_subwords(&hidden, &word_ids, 2).unwrap();
        let v = pooled.to_vec3::<f32>().unwrap();
        assert!((v[0][0][0] - 2.0).abs() < 1e-5);
        assert!((v[0][1][0] - 3.0).abs() < 1e-5);
    }

    #[test]
    fn empty_word_groups_get_zeros() {
        // word_ids only mentions word 0; word 1 has no subwords.
        let hidden = Tensor::new(&[[[5.0f32, 5.0], [5.0, 5.0]]], &Device::Cpu).unwrap();
        let word_ids = vec![vec![Some(0u32), Some(0)]];
        let pooled = pool_subwords(&hidden, &word_ids, 2).unwrap();
        let v = pooled.to_vec3::<f32>().unwrap();
        assert!((v[0][0][0] - 5.0).abs() < 1e-5);
        assert_eq!(v[0][1][0], 0.0);
    }
}
