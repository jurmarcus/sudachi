//! KWJA custom head modules.
//!
//! Each head is a small computation on top of the DeBERTa-v2 hidden states
//! (or pooled word states). We port only the heads jisho needs: pos / subpos
//! / reading / conjtype / conjform / ne / word_feature / base_phrase_feature
//! / dependency_parser. cohesion_analyzer / discourse_relation_analyzer /
//! dependency_type_parser are out of v0.1 scope.
//!
//! Tagger anatomy (from KWJA checkpoint dump):
//!   nn.Sequential([
//!     Linear(hidden, hidden),  // index 0
//!     <activation>,            // index 1 (no params)
//!     <dropout>,               // index 2 (no params, eval-time no-op)
//!     Linear(hidden, n_labels) // index 3
//!   ])
//! → 4 tensors per head (.0.{weight,bias}, .3.{weight,bias}).

use crate::Result;
use crate::error::Error;
use candle_core::{Module, Tensor};
use candle_nn::{Linear, VarBuilder, linear};

/// 2-linear MLP head with GELU between, used by pos/subpos/reading/conjtype/
/// conjform/ne taggers. Matches KWJA's `nn.Sequential([Linear, GELU,
/// Dropout, Linear])` minus the dropout (no-op at inference).
pub struct SequentialMlpHead {
    fc0: Linear,
    fc3: Linear,
}

impl SequentialMlpHead {
    pub fn from_var_builder(vb: VarBuilder, hidden: usize, num_labels: usize) -> Result<Self> {
        let fc0 = linear(hidden, hidden, vb.pp("0")).map_err(Error::from)?;
        let fc3 = linear(hidden, num_labels, vb.pp("3")).map_err(Error::from)?;
        Ok(Self { fc0, fc3 })
    }

    pub fn forward(&self, hidden: &Tensor) -> Result<Tensor> {
        let h = self.fc0.forward(hidden).map_err(Error::from)?;
        let h = h.gelu_erf().map_err(Error::from)?;
        self.fc3.forward(&h).map_err(Error::from)
    }
}

/// Pairwise word-selection head — generalizes KWJA's biaffine pattern to N
/// output labels. Matches KWJA-Python's `WordSelectionHead`:
///
/// ```text
/// h_source = self.l_source(pooled)                              # (B, S, H)
/// h_target = self.l_target(pooled)                              # (B, S, H)
/// hidden = activation(h_source.unsqueeze(2) + h_target.unsqueeze(1))  # (B, S, S, H)
/// return self.output_layer(hidden)                              # (B, S, S, num_labels)
/// ```
///
/// Used by:
///   * `dependency_parser`: num_labels=1, output_layer has no bias.
///   * `discourse_relation_analyzer`: num_labels=N, output_layer has bias.
///
/// Stored shapes:
///   l_source.weight (H, H), l_source.bias (H,)
///   l_target.weight (H, H), l_target.bias (H,)
///   output_layer.weight (num_labels, H), [output_layer.bias (num_labels,)]
pub struct WordSelectionHead {
    l_source: Linear,
    l_target: Linear,
    output_layer: Linear,
}

impl WordSelectionHead {
    pub fn from_var_builder(
        vb: VarBuilder,
        hidden: usize,
        num_labels: usize,
        with_bias: bool,
    ) -> Result<Self> {
        let l_source = linear(hidden, hidden, vb.pp("l_source")).map_err(Error::from)?;
        let l_target = linear(hidden, hidden, vb.pp("l_target")).map_err(Error::from)?;
        let output_layer = if with_bias {
            linear(hidden, num_labels, vb.pp("output_layer")).map_err(Error::from)?
        } else {
            candle_nn::linear_no_bias(hidden, num_labels, vb.pp("output_layer"))
                .map_err(Error::from)?
        };
        Ok(Self { l_source, l_target, output_layer })
    }

    /// Compute pairwise scores. `pooled` is (B, S, H). Returns (B, S, S,
    /// num_labels). The trailing dim is 1 for the dependency parser and the
    /// number of relation classes for the discourse analyzer.
    pub fn forward(&self, pooled: &Tensor) -> Result<Tensor> {
        let src = self.l_source.forward(pooled).map_err(Error::from)?; // (B, S, H)
        let tgt = self.l_target.forward(pooled).map_err(Error::from)?; // (B, S, H)
        let (b, s, h) = src.dims3().map_err(Error::from)?;
        // Additive pairwise interaction matches KWJA-Python (the prior
        // multiplicative `broadcast_mul` was a porting bug — it produced
        // dep argmax noisy enough that pipeline.rs had to fall back to a
        // right-headed heuristic). After this fix we re-test if argmax can
        // replace the heuristic.
        let src_un = src.unsqueeze(2).map_err(Error::from)?;            // (B, S, 1, H)
        let tgt_un = tgt.unsqueeze(1).map_err(Error::from)?;            // (B, 1, S, H)
        let combined = src_un
            .broadcast_add(&tgt_un)
            .map_err(Error::from)?
            .gelu_erf()
            .map_err(Error::from)?;                                      // (B, S, S, H)
        let flat = combined.reshape((b * s * s, h)).map_err(Error::from)?;
        let scored = self.output_layer.forward(&flat).map_err(Error::from)?;
        let num_labels = scored.dim(1).map_err(Error::from)?;
        scored.reshape((b, s, s, num_labels)).map_err(Error::from)
    }
}

/// Backwards-compat alias for callers that still spell the old name. Drop
/// once no in-tree call sites refer to `BiaffineDependencyHead`.
pub type BiaffineDependencyHead = WordSelectionHead;

/// Low-rank adaptation delta — produces a (H, H, L) tensor parameterized
/// by two factor matrices `dense_a (H, R, L)` and `dense_b (R, H, L)`.
/// Used by both `LoRASequenceMultiLabelingHead` (word_feature /
/// base_phrase_feature taggers) and `LoRARelationWiseWordSelectionHead`
/// (cohesion analyzer).
///
/// KWJA-Python computes the delta as:
///     delta[h, i, l] = sum_r dense_a[h, r, l] * dense_b[r, i, l]   # (H, H, L)
///
/// Candle has no n-dim einsum, so we contract per-label in a small loop.
/// L is small in practice (5 word_features, 64 bp_features, ~15 cohesion
/// relations) so the loop overhead is negligible.
pub struct LoRADelta {
    dense_a: Tensor, // (H, R, L)
    dense_b: Tensor, // (R, H, L)
}

impl LoRADelta {
    pub fn from_var_builder(
        vb: VarBuilder,
        hidden: usize,
        rank: usize,
        num_labels: usize,
    ) -> Result<Self> {
        let dense_a = vb.get((hidden, rank, num_labels), "dense_a").map_err(Error::from)?;
        let dense_b = vb.get((rank, hidden, num_labels), "dense_b").map_err(Error::from)?;
        Ok(Self { dense_a, dense_b })
    }

    /// Returns (H, H, L). Single bulk matmul rather than the per-label loop:
    ///
    /// ```text
    /// dense_a: (H, R, L) → permute (L, H, R) → contig
    /// dense_b: (R, H, L) → permute (L, R, H) → contig
    /// matmul → (L, H, H) → permute (H, H, L)
    /// ```
    ///
    /// One CUDA kernel launch instead of L. KWJA uses L up to 64 for
    /// bp_features, so the launch-overhead saving is substantial on GPU.
    pub fn forward(&self) -> Result<Tensor> {
        let (_h, _r, l) = self.dense_a.dims3().map_err(Error::from)?;
        if l == 0 {
            // Edge case for tests; an empty LoRA is rare in practice.
            let h = _h;
            return Tensor::zeros(
                (h, h, 0),
                self.dense_a.dtype(),
                self.dense_a.device(),
            )
            .map_err(Error::from);
        }
        // Re-arrange to (L, H, R) and (L, R, H) so we can batch-matmul
        // across the leading L dimension.
        let a = self
            .dense_a
            .permute([2, 0, 1])
            .map_err(Error::from)?
            .contiguous()
            .map_err(Error::from)?; // (L, H, R)
        let b = self
            .dense_b
            .permute([2, 0, 1])
            .map_err(Error::from)?
            .contiguous()
            .map_err(Error::from)?; // (L, R, H)
        let mm = a.matmul(&b).map_err(Error::from)?; // (L, H, H)
        // → (H, H, L) for downstream consumers.
        mm.permute([1, 2, 0]).map_err(Error::from)?.contiguous().map_err(Error::from)
    }
}

/// LoRA-augmented pairwise word-selection head, used by `cohesion_analyzer`
/// for predicate-argument structures, bridging reference, and coreference.
///
/// KWJA-Python forward (see kwja/modules/components/head.py):
///   h_source       = l_source(pooled)                                  # (B, S, H)
///   h_target       = l_target(pooled)                                  # (B, S, H)
///   delta_source_out = einsum("bsh,hil->bsli", pooled, delta_source()) # (B, S, R, H)
///   delta_target_out = einsum("bsh,hil->bsli", pooled, delta_target()) # (B, S, R, H)
///   source = h_source.unsqueeze(2) + delta_source_out                  # (B, S, R, H)
///   target = h_target.unsqueeze(2) + delta_target_out                  # (B, S, R, H)
///   hidden = act(source.unsqueeze(2) + target.unsqueeze(1))            # (B, S, S, R, H)
///   logits = einsum("bstlh,hl->bstl", hidden, classifier)              # (B, S, S, R)
///
/// Implementation note: candle has no n-dim einsum, so we contract via
/// reshape + matmul + per-relation broadcast. The pairwise (S, S) outer
/// product for relation-wise heads creates a (B, S, S, R, H) intermediate;
/// for typical S<=64 + R<=15 + H=768 this is ~30MB which is fine.
pub struct LoRARelationWiseWordSelectionHead {
    l_source: Linear,
    l_target: Linear,
    delta_source: LoRADelta,
    delta_target: LoRADelta,
    classifier: Tensor, // (H, R)
}

impl LoRARelationWiseWordSelectionHead {
    pub fn from_var_builder(
        vb: VarBuilder,
        hidden: usize,
        num_relations: usize,
        rank: usize,
    ) -> Result<Self> {
        let l_source = linear(hidden, hidden, vb.pp("l_source")).map_err(Error::from)?;
        let l_target = linear(hidden, hidden, vb.pp("l_target")).map_err(Error::from)?;
        let delta_source = LoRADelta::from_var_builder(vb.pp("delta_source"), hidden, rank, num_relations)?;
        let delta_target = LoRADelta::from_var_builder(vb.pp("delta_target"), hidden, rank, num_relations)?;
        let classifier = vb.get((hidden, num_relations), "classifier").map_err(Error::from)?;
        Ok(Self { l_source, l_target, delta_source, delta_target, classifier })
    }

    pub fn forward(&self, pooled: &Tensor) -> Result<Tensor> {
        let (b, s, h) = pooled.dims3().map_err(Error::from)?;
        let h_source = self.l_source.forward(pooled).map_err(Error::from)?; // (B, S, H)
        let h_target = self.l_target.forward(pooled).map_err(Error::from)?; // (B, S, H)
        let delta_s = self.delta_source.forward()?; // (H, H, R)
        let delta_t = self.delta_target.forward()?; // (H, H, R)
        let r = delta_s.dim(2).map_err(Error::from)?;

        // einsum("bsh,hil->bsli"):
        //   delta_*_out[b, s, l, i] = sum_h pooled[b, s, h] * delta[h, i, l]
        // Implemented as (B*S, H) × (H, H*R) → (B*S, H*R) → (B, S, H, R) → permute (B, S, R, H).
        let pf = pooled.reshape((b * s, h)).map_err(Error::from)?;
        let ds_flat = delta_s.reshape((h, h * r)).map_err(Error::from)?;
        let dt_flat = delta_t.reshape((h, h * r)).map_err(Error::from)?;
        let ds_out = pf.matmul(&ds_flat).map_err(Error::from)?
            .reshape((b, s, h, r)).map_err(Error::from)?
            .permute([0, 1, 3, 2]).map_err(Error::from)?
            .contiguous().map_err(Error::from)?; // (B, S, R, H)
        let dt_out = pf.matmul(&dt_flat).map_err(Error::from)?
            .reshape((b, s, h, r)).map_err(Error::from)?
            .permute([0, 1, 3, 2]).map_err(Error::from)?
            .contiguous().map_err(Error::from)?;

        // source = h_source.unsqueeze(2) + ds_out → (B, S, R, H)
        let source = h_source.unsqueeze(2).map_err(Error::from)?.broadcast_add(&ds_out).map_err(Error::from)?;
        let target = h_target.unsqueeze(2).map_err(Error::from)?.broadcast_add(&dt_out).map_err(Error::from)?;

        // hidden = act(source.unsqueeze(2) + target.unsqueeze(1)) → (B, S, S, R, H)
        let s_un = source.unsqueeze(2).map_err(Error::from)?;
        let t_un = target.unsqueeze(1).map_err(Error::from)?;
        let combined = s_un
            .broadcast_add(&t_un)
            .map_err(Error::from)?
            .gelu_erf()
            .map_err(Error::from)?;

        // einsum("bstlh,hl->bstl"): for each (b, src, tgt, l):
        //   logits[b, src, tgt, l] = sum_h combined[b, src, tgt, l, h] * classifier[h, l]
        // Build classifier broadcast: (1, 1, 1, R, H) — note we transpose
        // classifier (H, R) → (R, H) so the trailing dim aligns with combined's H axis.
        let cls_t = self.classifier.t().map_err(Error::from)?           // (R, H)
            .unsqueeze(0).map_err(Error::from)?                          // (1, R, H)
            .unsqueeze(0).map_err(Error::from)?                          // (1, 1, R, H)
            .unsqueeze(0).map_err(Error::from)?;                         // (1, 1, 1, R, H)
        let prod = combined.broadcast_mul(&cls_t).map_err(Error::from)?; // (B, S, S, R, H)
        prod.sum(4).map_err(Error::from) // sum over H → (B, S, S, R)
    }
}

/// LoRA-augmented multi-label per-position head, used by
/// `word_feature_tagger` and `base_phrase_feature_tagger`.
///
/// KWJA-Python forward:
///     dense_out      = dense(pooled)                               # (B, S, H)
///     dense_delta    = delta()                                     # (H, H, L)
///     dense_delta_out= einsum("bsh,hil->bsil", pooled, dense_delta) # (B, S, H, L)
///     hidden         = act(dense_out.unsqueeze(3) + dense_delta_out)# (B, S, H, L)
///     logits         = einsum("bshl,hl->bsl", hidden, classifier_w) + classifier_b
///     return sigmoid(logits)                                        # (B, S, L)
///
/// The two einsums are implemented via reshape + matmul + reduce since
/// candle lacks n-dim einsum. L is small, so the overhead is negligible.
pub struct LoRASequenceMultiLabelingHead {
    dense: Linear,
    delta: LoRADelta,
    classifier_weight: Tensor, // (H, L)
    classifier_bias: Tensor,   // (L,)
}

impl LoRASequenceMultiLabelingHead {
    pub fn from_var_builder(
        vb: VarBuilder,
        hidden: usize,
        num_labels: usize,
        rank: usize,
    ) -> Result<Self> {
        let dense = linear(hidden, hidden, vb.pp("dense")).map_err(Error::from)?;
        let delta = LoRADelta::from_var_builder(vb.pp("delta"), hidden, rank, num_labels)?;
        let classifier_weight = vb
            .get((hidden, num_labels), "classifier_weight")
            .map_err(Error::from)?;
        let classifier_bias = vb.get(num_labels, "classifier_bias").map_err(Error::from)?;
        Ok(Self { dense, delta, classifier_weight, classifier_bias })
    }

    pub fn forward(&self, pooled: &Tensor) -> Result<Tensor> {
        let (b, s, h) = pooled.dims3().map_err(Error::from)?;
        let dense_out = self.dense.forward(pooled).map_err(Error::from)?; // (B, S, H)
        let delta = self.delta.forward()?;                                 // (H, H, L)
        let l = delta.dim(2).map_err(Error::from)?;

        // einsum("bsh,hil->bsil"): for each (b, s, i, l):
        //     out[b, s, i, l] = sum_h pooled[b, s, h] * delta[h, i, l]
        // Implemented via (B*S, H) × (H, H*L) → (B*S, H*L) → reshape.
        let pooled_flat = pooled.reshape((b * s, h)).map_err(Error::from)?;
        let delta_flat = delta.reshape((h, h * l)).map_err(Error::from)?;
        let dd = pooled_flat.matmul(&delta_flat).map_err(Error::from)?; // (B*S, H*L)
        let dd = dd.reshape((b, s, h, l)).map_err(Error::from)?;

        // hidden = act(dense_out.unsqueeze(3) + dd) → (B, S, H, L)
        let dense_un = dense_out.unsqueeze(3).map_err(Error::from)?;
        let combined = dense_un
            .broadcast_add(&dd)
            .map_err(Error::from)?
            .gelu_erf()
            .map_err(Error::from)?;

        // einsum("bshl,hl->bsl"): sum over h, multiplying by classifier_weight.
        // Reshape combined to (B*S, H, L); broadcast classifier_weight (H, L)
        // along the leading B*S dim; elementwise multiply; sum over H.
        let combined_flat = combined.reshape((b * s, h, l)).map_err(Error::from)?;
        let cw_un = self.classifier_weight.unsqueeze(0).map_err(Error::from)?; // (1, H, L)
        let prod = combined_flat
            .broadcast_mul(&cw_un)
            .map_err(Error::from)?; // (B*S, H, L)
        let summed = prod.sum(1).map_err(Error::from)?; // (B*S, L)
        let logits = summed.reshape((b, s, l)).map_err(Error::from)?;
        let bias_un = self
            .classifier_bias
            .unsqueeze(0)
            .map_err(Error::from)?
            .unsqueeze(0)
            .map_err(Error::from)?; // (1, 1, L)
        let with_bias = logits.broadcast_add(&bias_un).map_err(Error::from)?;
        candle_nn::ops::sigmoid(&with_bias).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{DType, Device};
    use candle_nn::VarBuilder;
    use std::collections::HashMap;

    fn random_vb(shape: (usize, usize), name: &str) -> (HashMap<String, Tensor>, &str) {
        let mut h = HashMap::new();
        h.insert(format!("{name}.weight"), Tensor::randn(0f32, 1., shape, &Device::Cpu).unwrap());
        h.insert(format!("{name}.bias"), Tensor::randn(0f32, 1., shape.0, &Device::Cpu).unwrap());
        (h, name)
    }

    fn build_mlp_vb(hidden: usize, num_labels: usize) -> VarBuilder<'static> {
        let mut tensors: HashMap<String, Tensor> = HashMap::new();
        tensors.insert("0.weight".into(), Tensor::randn(0f32, 1., (hidden, hidden), &Device::Cpu).unwrap());
        tensors.insert("0.bias".into(), Tensor::randn(0f32, 1., hidden, &Device::Cpu).unwrap());
        tensors.insert("3.weight".into(), Tensor::randn(0f32, 1., (num_labels, hidden), &Device::Cpu).unwrap());
        tensors.insert("3.bias".into(), Tensor::randn(0f32, 1., num_labels, &Device::Cpu).unwrap());
        VarBuilder::from_tensors(tensors, DType::F32, &Device::Cpu)
    }

    fn build_biaffine_vb(hidden: usize) -> VarBuilder<'static> {
        let mut tensors: HashMap<String, Tensor> = HashMap::new();
        for prefix in ["l_source", "l_target"] {
            tensors.insert(format!("{prefix}.weight"), Tensor::randn(0f32, 1., (hidden, hidden), &Device::Cpu).unwrap());
            tensors.insert(format!("{prefix}.bias"), Tensor::randn(0f32, 1., hidden, &Device::Cpu).unwrap());
        }
        tensors.insert("output_layer.weight".into(), Tensor::randn(0f32, 1., (1, hidden), &Device::Cpu).unwrap());
        VarBuilder::from_tensors(tensors, DType::F32, &Device::Cpu)
    }

    #[test]
    fn sequential_mlp_head_shape() {
        let vb = build_mlp_vb(768, 14);
        let head = SequentialMlpHead::from_var_builder(vb, 768, 14).unwrap();
        let hidden = Tensor::randn(0f32, 1., (1, 10, 768), &Device::Cpu).unwrap();
        let logits = head.forward(&hidden).unwrap();
        assert_eq!(logits.dims(), &[1, 10, 14]);
    }

    #[test]
    fn word_selection_head_biaffine_shape() {
        let vb = build_biaffine_vb(768);
        let head = WordSelectionHead::from_var_builder(vb, 768, 1, false).unwrap();
        let hidden = Tensor::randn(0f32, 1., (1, 10, 768), &Device::Cpu).unwrap();
        let scores = head.forward(&hidden).unwrap();
        assert_eq!(scores.dims(), &[1, 10, 10, 1]);
    }

    fn build_word_selection_with_bias_vb(hidden: usize, num_labels: usize) -> VarBuilder<'static> {
        let mut tensors: HashMap<String, Tensor> = HashMap::new();
        for prefix in ["l_source", "l_target"] {
            tensors.insert(format!("{prefix}.weight"), Tensor::randn(0f32, 1., (hidden, hidden), &Device::Cpu).unwrap());
            tensors.insert(format!("{prefix}.bias"), Tensor::randn(0f32, 1., hidden, &Device::Cpu).unwrap());
        }
        tensors.insert("output_layer.weight".into(), Tensor::randn(0f32, 1., (num_labels, hidden), &Device::Cpu).unwrap());
        tensors.insert("output_layer.bias".into(), Tensor::randn(0f32, 1., num_labels, &Device::Cpu).unwrap());
        VarBuilder::from_tensors(tensors, DType::F32, &Device::Cpu)
    }

    #[test]
    fn word_selection_head_multi_label_shape() {
        let vb = build_word_selection_with_bias_vb(64, 7);
        let head = WordSelectionHead::from_var_builder(vb, 64, 7, true).unwrap();
        let hidden = Tensor::randn(0f32, 1., (1, 5, 64), &Device::Cpu).unwrap();
        let scores = head.forward(&hidden).unwrap();
        assert_eq!(scores.dims(), &[1, 5, 5, 7]);
    }

    fn build_lora_seq_vb(hidden: usize, num_labels: usize, rank: usize) -> VarBuilder<'static> {
        let mut t: HashMap<String, Tensor> = HashMap::new();
        t.insert("dense.weight".into(), Tensor::randn(0f32, 1., (hidden, hidden), &Device::Cpu).unwrap());
        t.insert("dense.bias".into(), Tensor::randn(0f32, 1., hidden, &Device::Cpu).unwrap());
        t.insert("delta.dense_a".into(), Tensor::randn(0f32, 1., (hidden, rank, num_labels), &Device::Cpu).unwrap());
        t.insert("delta.dense_b".into(), Tensor::randn(0f32, 1., (rank, hidden, num_labels), &Device::Cpu).unwrap());
        t.insert("classifier_weight".into(), Tensor::randn(0f32, 1., (hidden, num_labels), &Device::Cpu).unwrap());
        t.insert("classifier_bias".into(), Tensor::randn(0f32, 1., num_labels, &Device::Cpu).unwrap());
        VarBuilder::from_tensors(t, DType::F32, &Device::Cpu)
    }

    fn build_lora_rel_vb(hidden: usize, num_relations: usize, rank: usize) -> VarBuilder<'static> {
        let mut t: HashMap<String, Tensor> = HashMap::new();
        for prefix in ["l_source", "l_target"] {
            t.insert(format!("{prefix}.weight"), Tensor::randn(0f32, 1., (hidden, hidden), &Device::Cpu).unwrap());
            t.insert(format!("{prefix}.bias"), Tensor::randn(0f32, 1., hidden, &Device::Cpu).unwrap());
        }
        for prefix in ["delta_source", "delta_target"] {
            t.insert(format!("{prefix}.dense_a"), Tensor::randn(0f32, 1., (hidden, rank, num_relations), &Device::Cpu).unwrap());
            t.insert(format!("{prefix}.dense_b"), Tensor::randn(0f32, 1., (rank, hidden, num_relations), &Device::Cpu).unwrap());
        }
        t.insert("classifier".into(), Tensor::randn(0f32, 1., (hidden, num_relations), &Device::Cpu).unwrap());
        VarBuilder::from_tensors(t, DType::F32, &Device::Cpu)
    }

    #[test]
    fn lora_rel_word_selection_shape() {
        let (b, s, h, r) = (1, 4, 64, 5);
        let vb = build_lora_rel_vb(h, r, 4);
        let head = LoRARelationWiseWordSelectionHead::from_var_builder(vb, h, r, 4).unwrap();
        let pooled = Tensor::randn(0f32, 1., (b, s, h), &Device::Cpu).unwrap();
        let scores = head.forward(&pooled).unwrap();
        assert_eq!(scores.dims(), &[b, s, s, r]);
    }

    #[test]
    fn lora_seq_multi_label_shape_and_sigmoid() {
        let (b, s, h, l) = (1, 5, 64, 7);
        let vb = build_lora_seq_vb(h, l, 4);
        let head = LoRASequenceMultiLabelingHead::from_var_builder(vb, h, l, 4).unwrap();
        let pooled = Tensor::randn(0f32, 1., (b, s, h), &Device::Cpu).unwrap();
        let probs = head.forward(&pooled).unwrap();
        assert_eq!(probs.dims(), &[b, s, l]);
        let v: Vec<f32> = probs.flatten_all().unwrap().to_vec1().unwrap();
        for x in v {
            assert!((0.0..=1.0).contains(&x), "sigmoid output out of [0,1]: {x}");
        }
    }

    // Ensure the unused random_vb helper isn't dead code in tests when we add
    // more head variants.
    #[allow(dead_code)]
    fn _use_random_vb() {
        let _ = random_vb((4, 4), "x");
    }
}
