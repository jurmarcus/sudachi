//! Linear-chain CRF Viterbi decoder.
//!
//! Inference-only — KWJA's CRF (`crf.start_transitions`, `crf.end_transitions`,
//! `crf.transitions`) sits on top of an emission head. We decode the most
//! likely tag sequence given emissions and transition scores. Mirrors
//! pytorch-crf's algorithm.

use crate::Result;
use crate::error::Error;
use candle_core::Tensor;
use candle_nn::VarBuilder;

pub struct LinearChainCrf {
    /// (num_labels, num_labels) transition scores. transitions[i][j] is
    /// the score for going from tag i (prev) to tag j (current).
    pub transitions: Vec<Vec<f32>>,
    /// (num_labels,) start scores.
    pub start: Vec<f32>,
    /// (num_labels,) end scores.
    pub end: Vec<f32>,
}

impl LinearChainCrf {
    pub fn from_var_builder(vb: VarBuilder, num_labels: usize) -> Result<Self> {
        let transitions = vb
            .get((num_labels, num_labels), "transitions")
            .map_err(Error::from)?
            .to_vec2::<f32>()
            .map_err(Error::from)?;
        let start = vb
            .get(num_labels, "start_transitions")
            .map_err(Error::from)?
            .to_vec1::<f32>()
            .map_err(Error::from)?;
        let end = vb
            .get(num_labels, "end_transitions")
            .map_err(Error::from)?
            .to_vec1::<f32>()
            .map_err(Error::from)?;
        Ok(Self { transitions, start, end })
    }

    pub fn from_tensors(transitions: &Tensor, start: &Tensor, end: &Tensor) -> Result<Self> {
        Ok(Self {
            transitions: transitions.to_vec2::<f32>().map_err(Error::from)?,
            start: start.to_vec1::<f32>().map_err(Error::from)?,
            end: end.to_vec1::<f32>().map_err(Error::from)?,
        })
    }

    /// Decode the highest-scoring label sequence per batch element.
    ///
    /// `emissions`: (B, T, L) tensor of emission logits.
    /// Returns: `Vec<Vec<usize>>` of length B, each inner Vec length T.
    pub fn viterbi(&self, emissions: &Tensor) -> Result<Vec<Vec<usize>>> {
        let (b, t, l) = emissions.dims3().map_err(Error::from)?;
        let emissions = emissions.to_vec3::<f32>().map_err(Error::from)?;

        let mut paths = Vec::with_capacity(b);
        for batch_idx in 0..b {
            let mut dp = vec![vec![f32::NEG_INFINITY; l]; t];
            let mut back = vec![vec![0usize; l]; t];

            // Start
            for label in 0..l {
                dp[0][label] = self.start[label] + emissions[batch_idx][0][label];
            }

            // Recurrence
            for ti in 1..t {
                for cur in 0..l {
                    let mut best_score = f32::NEG_INFINITY;
                    let mut best_prev = 0usize;
                    for prev in 0..l {
                        let score = dp[ti - 1][prev] + self.transitions[prev][cur];
                        if score > best_score {
                            best_score = score;
                            best_prev = prev;
                        }
                    }
                    dp[ti][cur] = best_score + emissions[batch_idx][ti][cur];
                    back[ti][cur] = best_prev;
                }
            }

            // End: incorporate end_transitions and pick the best final tag.
            let mut last_label = 0usize;
            let mut last_score = f32::NEG_INFINITY;
            for label in 0..l {
                let final_score = dp[t - 1][label] + self.end[label];
                if final_score > last_score {
                    last_score = final_score;
                    last_label = label;
                }
            }

            // Backtrace
            let mut path = vec![0usize; t];
            path[t - 1] = last_label;
            for ti in (1..t).rev() {
                path[ti - 1] = back[ti][path[ti]];
            }
            paths.push(path);
        }

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    #[test]
    fn viterbi_picks_highest_scoring_path() {
        // 3 timesteps, 2 labels. Emissions favor 0,1,0; transitions zeroed.
        let emissions = Tensor::new(
            &[[[10.0f32, 0.0], [0.0, 10.0], [10.0, 0.0]]],
            &Device::Cpu,
        )
        .unwrap();
        let crf = LinearChainCrf {
            transitions: vec![vec![0.0, 0.0], vec![0.0, 0.0]],
            start: vec![0.0, 0.0],
            end: vec![0.0, 0.0],
        };
        let path = crf.viterbi(&emissions).unwrap();
        assert_eq!(path, vec![vec![0, 1, 0]]);
    }

    #[test]
    fn viterbi_respects_transitions() {
        // Both labels have equal emissions everywhere. Transitions strongly
        // prefer 0->0 over 0->1, so the optimal path should stay at 0.
        let emissions = Tensor::new(
            &[[[1.0f32, 1.0], [1.0, 1.0], [1.0, 1.0]]],
            &Device::Cpu,
        )
        .unwrap();
        let crf = LinearChainCrf {
            transitions: vec![vec![10.0, -10.0], vec![-10.0, 10.0]],
            start: vec![0.0, 0.0],
            end: vec![0.0, 0.0],
        };
        let path = crf.viterbi(&emissions).unwrap();
        // Either all 0s or all 1s — depending on tiebreaking. Both are
        // valid, just check the path is monotone.
        for w in path[0].windows(2) {
            assert_eq!(w[0], w[1], "transitions should keep label constant");
        }
    }

    #[test]
    fn viterbi_batches_independently() {
        let emissions = Tensor::new(
            &[
                [[10.0f32, 0.0], [0.0, 10.0]],
                [[0.0, 10.0], [10.0, 0.0]],
            ],
            &Device::Cpu,
        )
        .unwrap();
        let crf = LinearChainCrf {
            transitions: vec![vec![0.0, 0.0], vec![0.0, 0.0]],
            start: vec![0.0, 0.0],
            end: vec![0.0, 0.0],
        };
        let paths = crf.viterbi(&emissions).unwrap();
        assert_eq!(paths, vec![vec![0, 1], vec![1, 0]]);
    }
}
