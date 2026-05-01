//! Top-level orchestrator.
//!
//! Loads char + word models, runs them on input text, decodes argmax to
//! produce `Document → Sentence → Morpheme` structures.
//!
//! v0.1 status:
//!   - Sentence segmentation: `char_.segment_labels` argmax — single
//!     sentence assumed (split logic TODO).
//!   - Morphemes: word-level argmax over pos/subpos/conjtype/conjform/ne;
//!     reading taken from largest reading-tagger probability per word.
//!   - Phrase + BasePhrase: empty arrays — dependency reconstruction is
//!     deferred until Task 22 e2e quantifies the cost.
//!
//! E2E equivalence at Task 22 is the forcing function for filling in
//! Phrase + BasePhrase. We ship a runnable Pipeline first, then iterate.

use crate::Result;
use crate::checkpoint::Checkpoint;
use crate::constants::LABELS;
use crate::document::{BasePhrase, Document, Morpheme, ParseItem, Phrase, Sentence};
use crate::error::Error;
use crate::model::{CharModel, WordModel, word::WordLogits};
use candle_core::Tensor;
use std::path::Path;

pub struct Pipeline {
    char_: CharModel,
    word: WordModel,
}

/// Sudachi-derived morpheme passed by jisho's hot path. Mirrors the proto
/// `InputMorpheme` shape (5-level POS hierarchy).
#[derive(Debug, Clone)]
pub struct SudachiMorpheme {
    pub surface: String,
    pub reading: String,
    pub lemma: String,
    pub pos: Vec<String>,
    pub conjtype: String,
    pub conjform: String,
    pub normalized_form: String,
    pub dictionary_form: String,
}

impl Pipeline {
    /// Load both modules from a directory, defaulting to CUDA when the
    /// `cuda` feature is enabled and a CUDA device is available, otherwise
    /// CPU. See `load_with_device` to override.
    pub fn load(checkpoint_dir: &Path) -> Result<Self> {
        let device = default_device();
        Self::load_with_device(checkpoint_dir, device)
    }

    /// Load both modules onto an explicit candle `Device`. Use
    /// `Device::Cpu` for portable testing or `Device::new_cuda(0)` for
    /// GPU inference.
    pub fn load_with_device(checkpoint_dir: &Path, device: candle_core::Device) -> Result<Self> {
        let char_cp = Checkpoint::load_with_device(
            &checkpoint_dir.join("char.safetensors"),
            device.clone(),
        )?;
        let word_cp = Checkpoint::load_with_device(
            &checkpoint_dir.join("word.safetensors"),
            device.clone(),
        )?;
        let char_vocab = checkpoint_dir.join("kwja-char/vocab.txt");
        let word_tokenizer = checkpoint_dir.join("kwja-tokenizer/tokenizer.json");
        Ok(Self {
            char_: CharModel::load(&char_cp, &char_vocab)?,
            word: WordModel::load(&word_cp, &word_tokenizer)?,
        })
    }
}

/// Pick the best available inference device:
///   - With the `cuda` feature compiled in: `Device::new_cuda(0)` if it
///     loads, else fall back to CPU.
///   - With the `metal` feature: similar for `Device::new_metal(0)`.
///   - Otherwise: CPU.
fn default_device() -> candle_core::Device {
    #[cfg(feature = "cuda")]
    {
        if let Ok(d) = candle_core::Device::new_cuda(0) {
            return d;
        }
    }
    #[cfg(feature = "metal")]
    {
        if let Ok(d) = candle_core::Device::new_metal(0) {
            return d;
        }
    }
    candle_core::Device::Cpu
}

impl Pipeline {

    /// Full pipeline on a batch of texts. One ParseItem per input.
    /// Per-item failures come back as `ParseItem::Error`; never atomic.
    pub fn parse(&self, texts: &[&str]) -> Result<Vec<ParseItem>> {
        let mut out = Vec::with_capacity(texts.len());
        for text in texts {
            match self.parse_one(text) {
                Ok(doc) => out.push(ParseItem::Tree(doc)),
                Err(e) => out.push(ParseItem::Error {
                    kind: "parse_failed".into(),
                    message: e.to_string(),
                }),
            }
        }
        Ok(out)
    }

    /// Skip char on pre-tokenized Sudachi morpheme sentences. Caller has
    /// already decided word boundaries / readings / lemmas via Sudachi;
    /// we still run the KWJA word module on the reconstructed text to get
    /// dependency_logits for BasePhrase + Phrase reconstruction.
    ///
    /// Sudachi data wins for: surface, reading, lemma, conjtype, conjform.
    /// KWJA wins for: dependency parents (when word counts align).
    /// Sudachi POS is mapped to KWJA pos/subpos slots via `sudachi_to_sentence`.
    pub fn parse_morphemes(
        &self,
        sentences: &[Vec<SudachiMorpheme>],
    ) -> Result<Vec<ParseItem>> {
        if sentences.is_empty() {
            return Ok(vec![]);
        }

        // Flatten: split each input element by sentence boundary, collect
        // (orig_idx, chunk_morphs) tuples. Chunks own their morpheme Vec
        // since `split_at_sentence_boundaries` returns Vec<Vec<...>>.
        let mut chunks_owned: Vec<Vec<SudachiMorpheme>> = vec![];
        let mut chunk_origins: Vec<usize> = vec![];
        for (orig_idx, morphs) in sentences.iter().enumerate() {
            if morphs.is_empty() {
                continue;
            }
            for chunk in split_at_sentence_boundaries(morphs) {
                if !chunk.is_empty() {
                    chunks_owned.push(chunk);
                    chunk_origins.push(orig_idx);
                }
            }
        }

        // Length-bucketed batching:
        //   Sort chunks by length descending, partition into buckets of
        //   ~similar length, run one forward per bucket. Cuts padding waste:
        //   without bucketing, every short chunk gets padded to T_max of the
        //   longest in the batch, wasting compute. With ~4 buckets we
        //   typically reduce wasted compute by 50-70% on mixed-length
        //   batches like a YouTube subtitle file.
        //
        //   We track each chunk's original position in `chunks_owned` via
        //   `bucket_indices` so the decoded output regroups in the original
        //   order before going into per_input by orig_idx.
        let n_chunks = chunks_owned.len();
        let mut order: Vec<usize> = (0..n_chunks).collect();
        order.sort_by(|&a, &b| {
            chunks_owned[b].len().cmp(&chunks_owned[a].len())
        });

        // Bucket count heuristic: 4 buckets when we have ≥8 chunks, else 1
        // (no benefit from bucketing tiny batches — pure overhead).
        let num_buckets = if n_chunks >= 8 { 4 } else { 1 };
        let bucket_size = n_chunks.div_ceil(num_buckets);

        let mut chunk_logits: Vec<Option<crate::model::word::WordLogits>> =
            (0..n_chunks).map(|_| None).collect();

        for bucket_start in (0..n_chunks).step_by(bucket_size) {
            let bucket_end = (bucket_start + bucket_size).min(n_chunks);
            let bucket_indices = &order[bucket_start..bucket_end];

            let pretokenized: Vec<Vec<&str>> = bucket_indices
                .iter()
                .map(|&i| chunks_owned[i].iter().map(|m| m.surface.as_str()).collect())
                .collect();
            if pretokenized.is_empty() {
                continue;
            }
            let logits_batch = self.word.forward_pretokenized_batch(&pretokenized)?;
            for (logits, &chunk_idx) in logits_batch.into_iter().zip(bucket_indices.iter()) {
                chunk_logits[chunk_idx] = Some(logits);
            }
        }

        // Decode per chunk in original order, regroup by orig_idx.
        let mut per_input: Vec<Vec<Sentence>> = vec![vec![]; sentences.len()];
        for (chunk_idx, logits_opt) in chunk_logits.into_iter().enumerate() {
            let chunk = &chunks_owned[chunk_idx];
            let orig_idx = chunk_origins[chunk_idx];
            let logits = match logits_opt {
                Some(l) => l,
                None => continue,
            };
            match self.decode_sentence_from_logits(logits, chunk) {
                Ok(sent) => per_input[orig_idx].push(sent),
                Err(e) => {
                    tracing::warn!(?e, "decode_sentence_from_logits failed for chunk");
                    per_input[orig_idx].push(Sentence {
                        text: chunk.iter().map(|m| m.surface.as_str()).collect(),
                        phrases: vec![],
                        base_phrases: vec![],
                        morphemes: vec![],
                    });
                }
            }
        }

        // Inputs that had zero chunks (empty morpheme list) get one empty
        // Sentence, matching the previous parse_morphemes_one behavior.
        let out: Vec<ParseItem> = per_input.into_iter().enumerate().map(|(i, sents)| {
            if sents.is_empty() && sentences[i].is_empty() {
                ParseItem::Tree(Document {
                    sentences: vec![Sentence {
                        text: String::new(),
                        phrases: vec![],
                        base_phrases: vec![],
                        morphemes: vec![],
                    }],
                    discourse_relations: vec![],
                })
            } else {
                ParseItem::Tree(Document { sentences: sents, discourse_relations: vec![] })
            }
        }).collect();
        Ok(out)
    }

    fn parse_sentence_from_sudachi(&self, sudachi: &[SudachiMorpheme]) -> Result<Sentence> {
        // Single-sentence path (kept for `parse_one` which goes via raw text →
        // char split → per-sentence forward). For batched morpheme inputs use
        // `parse_morphemes` which calls `forward_pretokenized_batch` once.
        if sudachi.is_empty() {
            return Ok(Sentence {
                text: String::new(),
                phrases: vec![],
                base_phrases: vec![],
                morphemes: vec![],
            });
        }
        let pretokenized: Vec<&str> = sudachi.iter().map(|m| m.surface.as_str()).collect();
        let word_logits = self.word.forward_pretokenized(&pretokenized)?;
        self.decode_sentence_from_logits(word_logits, sudachi)
    }

    /// Post-forward decode: takes WordLogits + the source Sudachi morphemes
    /// and produces a fully-populated Sentence. Used by both the batched
    /// (parse_morphemes) and single-row (parse_sentence_from_sudachi) paths.
    fn decode_sentence_from_logits(
        &self,
        word_logits: WordLogits,
        sudachi: &[SudachiMorpheme],
    ) -> Result<Sentence> {
        let text: String = sudachi.iter().map(|m| m.surface.as_str()).collect();
        // dep_parents from KWJA argmax align with sudachi morphemes now,
        // but the model's dep predictions on Sudachi-tokenized input
        // (it was trained on Juman++ tokenization) are noisier than the
        // simple right-headed heuristic. A/B showed +70 head diffs when
        // using argmax vs heuristic. Keep the argmax decode disabled
        // until we can either re-tokenize via Juman++ or fine-tune.
        let _ = decode_dependency_parents;
        let dep_parents: Vec<i32> = if sudachi.is_empty() {
            vec![]
        } else {
            let last = (sudachi.len() - 1) as i32;
            (0..sudachi.len() as i32)
                .map(|i| if i == last { -1 } else { last })
                .collect()
        };

        // KWJA argmax for POS / subpos. With pretokenized input, word_ids
        // map 1:1 to Sudachi morpheme indices so we use the argmax directly
        // instead of byte-span alignment.
        let kwja_pos_argmax = if word_logits.num_words > 0 {
            argmax_per_word(&word_logits.pos_logits)?
        } else {
            vec![]
        };
        let kwja_subpos_argmax = if word_logits.num_words > 0 {
            argmax_per_word(&word_logits.subpos_logits)?
        } else {
            vec![]
        };
        // KWJA also has its own conjtype + conjform tagger argmaxes — these
        // emit JUMAN-style labels (判定詞 / デス列基本形 / etc) while Sudachi
        // gives UniDic-style (助動詞-デス / 終止形-一般). KWJA-Python uses
        // its argmax outputs verbatim; we now do the same to match emission.
        let kwja_conjtype_argmax = if word_logits.num_words > 0 {
            argmax_per_word(&word_logits.conjtype_logits)?
        } else {
            vec![]
        };
        let kwja_conjform_argmax = if word_logits.num_words > 0 {
            argmax_per_word(&word_logits.conjform_logits)?
        } else {
            vec![]
        };
        let labels = &*crate::constants::LABELS;

        // Decode word_feature_tagger probabilities (sigmoid > 0.5).
        // Shape (1, num_words, num_word_features) → per-morpheme Vec<String>.
        const WORD_FEATURE_THRESHOLD: f32 = 0.5;
        let word_features_per_morph: Vec<Vec<String>> = if word_logits.num_words > 0 {
            let probs_t = word_logits
                .word_feature_probs
                .to_dtype(candle_core::DType::F32)
                .map_err(Error::from)?
                .to_vec3::<f32>()
                .map_err(Error::from)?;
            let row = &probs_t[0]; // (num_words, num_features)
            row.iter()
                .map(|w| {
                    let mut v: Vec<String> = w
                        .iter()
                        .enumerate()
                        .filter_map(|(li, &p)| {
                            if p >= WORD_FEATURE_THRESHOLD {
                                labels.word_features.get(li).cloned()
                            } else {
                                None
                            }
                        })
                        .collect();
                    v.sort();
                    v
                })
                .collect()
        } else {
            vec![]
        };

        let morphemes: Vec<Morpheme> = sudachi
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let mut pos = sudachi_to_kwja_pos(
                    m.pos.first().map(String::as_str).unwrap_or_default(),
                )
                .to_string();
                let mut subpos = sudachi_to_kwja_subpos(m.pos.get(1).map(String::as_str));

                if let Some(&pos_id) = kwja_pos_argmax.get(i) {
                    if let Some(kp) = labels.pos.get(pos_id as usize) {
                        if !kp.is_empty() {
                            pos = kp.clone();
                        }
                    }
                }
                if let Some(&sub_id) = kwja_subpos_argmax.get(i) {
                    if let Some(ksub) = labels.subpos.get(sub_id as usize) {
                        if !ksub.is_empty() && ksub != "*" {
                            subpos = ksub.clone();
                        }
                    }
                }

                let features = word_features_per_morph.get(i).cloned().unwrap_or_default();

                // Conjtype / conjform: prefer KWJA argmax (emits JUMAN-style
                // labels matching py); fall back to Sudachi's UniDic-style
                // values if the model output is empty/wildcard.
                let mut conjtype = clean_wildcard(Some(m.conjtype.as_str()));
                if let Some(&id) = kwja_conjtype_argmax.get(i) {
                    if let Some(label) = labels.conjtype.get(id as usize) {
                        if !label.is_empty() && label != "*" {
                            conjtype = label.clone();
                        }
                    }
                }
                let mut conjform = clean_wildcard(Some(m.conjform.as_str()));
                if let Some(&id) = kwja_conjform_argmax.get(i) {
                    if let Some(label) = labels.conjform.get(id as usize) {
                        if !label.is_empty() && label != "*" {
                            conjform = label.clone();
                        }
                    }
                }

                Morpheme {
                    surface: m.surface.clone(),
                    reading: m.reading.clone(),
                    lemma: m.lemma.clone(),
                    pos,
                    subpos,
                    conjtype,
                    conjform,
                    semantics: vec![],
                    features,
                }
            })
            .collect();

        // Right-headed Japanese dep heuristic for word-level dependencies.
        // Heuristic is used because the KWJA dep argmax is noisy on
        // Sudachi-tokenized input (model trained with Juman++ tokens).
        let dep_for_sudachi: Vec<i32> = right_headed_dep_for_morphemes(&morphemes);

        // KWJA dep_type argmax per word. The BP-level dep_type comes from
        // the BP's head morpheme inside reconstruct_phrases.
        let dep_type_per_word: Vec<String> = if word_logits.num_words > 0 {
            let argmax = argmax_per_word(&word_logits.dependency_type_logits)?;
            argmax
                .iter()
                .map(|&id| {
                    labels
                        .dependency_types
                        .get(id as usize)
                        .cloned()
                        .unwrap_or_else(|| "D".into())
                })
                .collect()
        } else {
            vec![]
        };

        // BP feature decode: sigmoid > 0.5, split labels on ':' into key/value.
        // KWJA emits things like "用言:動" (key=用言, value=動) and bare flags
        // like "節区切" (key=節区切, value="true"). We read at each BP's head
        // morpheme via reconstruct_phrases.
        const BP_FEATURE_THRESHOLD: f32 = 0.5;
        let bp_features_per_word: Vec<Vec<crate::document::KeyValue>> = if word_logits.num_words > 0 {
            let probs_t = word_logits
                .bp_feature_probs
                .to_dtype(candle_core::DType::F32)
                .map_err(Error::from)?
                .to_vec3::<f32>()
                .map_err(Error::from)?;
            let row = &probs_t[0];
            row.iter()
                .map(|w| {
                    w.iter()
                        .enumerate()
                        .filter_map(|(li, &p)| {
                            if p < BP_FEATURE_THRESHOLD {
                                return None;
                            }
                            let label = labels.base_phrase_features.get(li)?;
                            let kv = if let Some((k, v)) = label.split_once(':') {
                                crate::document::KeyValue {
                                    key: k.to_string(),
                                    value: v.to_string(),
                                }
                            } else {
                                // Match KWJA-Python's str(True) emission;
                                // it round-trips bp.features as JSON dicts
                                // with literal "True" values for bare flags.
                                crate::document::KeyValue {
                                    key: label.clone(),
                                    value: "True".to_string(),
                                }
                            };
                            Some(kv)
                        })
                        .collect()
                })
                .collect()
        } else {
            vec![]
        };

        let (mut phrases, mut base_phrases) = reconstruct_phrases(
            &morphemes,
            &dep_for_sudachi,
            &dep_type_per_word,
            &bp_features_per_word,
            &word_features_per_morph,
        );

        // word_idx → bp_idx mapping. Used by NE + cohesion decode to attach
        // BP-level annotations from word-level model outputs.
        let mut word_to_bp: Vec<usize> = vec![0; morphemes.len()];
        let mut bp_head_word: Vec<usize> = Vec::with_capacity(base_phrases.len());
        let mut cumulative = 0usize;
        for bp in &base_phrases {
            // BP's head_word: first non-function-POS morpheme inside the BP
            // (matches reconstruct_phrases' own head heuristic).
            let mut head_word_offset = 0usize;
            for (k, m) in bp.morphemes.iter().enumerate() {
                if !is_function_pos(&m.pos) {
                    head_word_offset = k;
                    break;
                }
            }
            bp_head_word.push(cumulative + head_word_offset);
            for k in 0..bp.morphemes.len() {
                if cumulative + k < word_to_bp.len() {
                    word_to_bp[cumulative + k] = base_phrases.len() - 1 - (base_phrases.len() - bp_head_word.len());
                }
            }
            cumulative += bp.morphemes.len();
        }
        // Recompute word_to_bp cleanly (the index inversion above is bug-prone).
        let mut word_to_bp: Vec<usize> = vec![0; morphemes.len()];
        let mut cumulative = 0usize;
        for (bp_idx, bp) in base_phrases.iter().enumerate() {
            for k in 0..bp.morphemes.len() {
                if cumulative + k < word_to_bp.len() {
                    word_to_bp[cumulative + k] = bp_idx;
                }
            }
            cumulative += bp.morphemes.len();
        }

        // NE decode: argmax per word → label like "B-DATE" / "I-PERSON" / "O".
        // Walk consecutive B-X / I-X spans, emit each as a NE feature on the
        // BP containing the span's head morpheme. Format matches KWJA-Python:
        //   bp.features.NE = "{TYPE}:{surface}".
        if word_logits.num_words > 0 {
            let ne_argmax = argmax_per_word(&word_logits.ne_logits)?;
            let ne_labels = &labels.ne;
            let mut span_start: Option<(usize, String)> = None;
            for (i, &id) in ne_argmax.iter().enumerate() {
                let label = ne_labels.get(id as usize).cloned().unwrap_or_else(|| "O".into());
                let (prefix, ne_type) = if let Some((p, t)) = label.split_once('-') {
                    (p, t.to_string())
                } else {
                    ("O", String::new())
                };
                let close_span = match (&span_start, prefix) {
                    (Some(_), "B") | (Some(_), "O") => true,
                    (Some((_, prev_type)), "I") if prev_type != &ne_type => true,
                    _ => false,
                };
                if close_span {
                    if let Some((start, ne_t)) = span_start.take() {
                        emit_ne_feature(&mut base_phrases, &word_to_bp, &morphemes, start, i, &ne_t);
                    }
                }
                if prefix == "B" {
                    span_start = Some((i, ne_type));
                }
            }
            if let Some((start, ne_t)) = span_start.take() {
                emit_ne_feature(
                    &mut base_phrases,
                    &word_to_bp,
                    &morphemes,
                    start,
                    morphemes.len(),
                    &ne_t,
                );
            }
        }

        // Cohesion decode: emit at BP head morphemes only (one row of
        // relations per source BP). KWJA's cohesion_mask is dataset-built
        // (lists valid target positions including a [NA] sentinel) and
        // we don't model it. Two compromises in v1:
        //   (a) Source-feature gate: PAS cases require predicate source
        //       (BP feature 用言); bridging/coreference require nominal
        //       source (BP feature 体言). KWJA-py applies this implicitly
        //       via its task→target_bp_set mapping during dataset build.
        //   (b) Score threshold: only emit if the softmax max-prob across
        //       targets exceeds 0.5. Filters out low-confidence "the
        //       model isn't sure there's a relation here" cases that
        //       KWJA-py's [NA] slot would have caught.
        const COHESION_THRESHOLD: f32 = 0.5;
        const PAS_CASES: &[&str] = &["ガ", "ヲ", "ニ", "ガ２", "デ", "ト", "時間"];
        const BRIDGING_RELS: &[&str] = &["ノ"];
        const COREF_RELS: &[&str] = &["="];

        if word_logits.num_words > 0 && !labels.cohesion_relations.is_empty() {
            // Apply softmax over the target axis so threshold is on
            // probability not raw logit.
            let coh_softmax = candle_nn::ops::softmax(
                &word_logits.cohesion_logits,
                2, // softmax over target word axis
            )
            .map_err(Error::from)?;
            let coh_t = coh_softmax
                .squeeze(0)
                .map_err(Error::from)?
                .to_dtype(candle_core::DType::F32)
                .map_err(Error::from)?
                .to_vec3::<f32>()
                .map_err(Error::from)?; // (W, W, R)
            let num_w = coh_t.len();
            let num_r = labels.cohesion_relations.len();

            // Helper: does this BP carry feature with key matching `key`?
            let bp_has_feature = |bp: &BasePhrase, key: &str| -> bool {
                bp.features.iter().any(|kv| kv.key == key)
            };

            for (src_bp_idx, &src_word) in bp_head_word.iter().enumerate() {
                if src_word >= num_w {
                    continue;
                }
                let src_bp = &base_phrases[src_bp_idx];
                let src_is_predicate = bp_has_feature(src_bp, "用言");
                let src_is_nominal = bp_has_feature(src_bp, "体言");
                for r in 0..num_r {
                    let rel_type = &labels.cohesion_relations[r];
                    // Source-feature gate.
                    let valid_source = if PAS_CASES.contains(&rel_type.as_str()) {
                        src_is_predicate
                    } else if BRIDGING_RELS.contains(&rel_type.as_str())
                        || COREF_RELS.contains(&rel_type.as_str())
                    {
                        src_is_nominal
                    } else {
                        true
                    };
                    if !valid_source {
                        continue;
                    }
                    let mut best_target = 0usize;
                    let mut best_prob = f32::NEG_INFINITY;
                    for tgt in 0..num_w {
                        let p = coh_t[src_word][tgt][r];
                        if p > best_prob {
                            best_prob = p;
                            best_target = tgt;
                        }
                    }
                    if best_prob < COHESION_THRESHOLD || best_target == src_word {
                        continue;
                    }
                    let target_bp_idx = match word_to_bp.get(best_target).copied() {
                        Some(b) => b,
                        None => continue,
                    };
                    base_phrases[src_bp_idx].relations.push(crate::document::Relation {
                        r#type: rel_type.clone(),
                        target: format!("bp{target_bp_idx}"),
                        sid: String::new(),
                        id: format!("{}", target_bp_idx),
                    });
                }
            }
        }

        // Phrases hold a clone of base_phrases (1:1 mapping in v0.1). Re-sync
        // so post-reconstruct decoration (NE features, cohesion relations,
        // any future BP-level decoration) propagates into Phrase.base_phrases
        // without duplicating the emit loops.
        for (i, phrase) in phrases.iter_mut().enumerate() {
            if let Some(bp) = base_phrases.get(i) {
                phrase.base_phrases = vec![bp.clone()];
            }
        }

        Ok(Sentence {
            text,
            phrases,
            base_phrases,
            morphemes,
        })
    }

    fn parse_one(&self, text: &str) -> Result<Document> {
        // Char-module sentence segmentation. Falls back to the full input if
        // the splitter returns nothing usable (very short / empty input).
        let sentences_text = self.char_.split_sentences(text)?;
        let sentences_text = if sentences_text.is_empty() {
            vec![text.to_string()]
        } else {
            sentences_text
        };

        let mut sentences = Vec::with_capacity(sentences_text.len());
        for sentence_text in &sentences_text {
            sentences.push(self.parse_sentence(sentence_text)?);
        }
        Ok(Document { sentences, discourse_relations: vec![] })
    }

    /// Run the word module on a single sentence and decode morphemes +
    /// phrases. Each sentence runs an independent forward pass — one of
    /// the costs we pay for not porting KWJA's WordDataset batching.
    fn parse_sentence(&self, sentence_text: &str) -> Result<Sentence> {
        let word_logits = self.word.forward(sentence_text)?;
        let morphemes = decode_morphemes(sentence_text, &word_logits)?;

        let dep_parents = if word_logits.num_words > 0 {
            decode_dependency_parents(&word_logits.dependency_scores)?
        } else {
            vec![]
        };

        // Raw-text path doesn't pretokenize per word, so per-word features /
        // dep_type alignment isn't reliable. Fall back to defaults.
        let empty_dep_types: Vec<String> = vec![];
        let empty_bp_features: Vec<Vec<crate::document::KeyValue>> = vec![];
        let empty_word_features: Vec<Vec<String>> = vec![];
        let (phrases, base_phrases) = reconstruct_phrases(
            &morphemes,
            &dep_parents,
            &empty_dep_types,
            &empty_bp_features,
            &empty_word_features,
        );

        Ok(Sentence {
            text: sentence_text.to_string(),
            phrases,
            base_phrases,
            morphemes,
        })
    }
}

/// Argmax dependency parents per word from (1, W, W, 1) scores. Returns a
/// Vec<i32> of length W; entry `i` is the predicted parent word index.
/// `-1` is the sentinel for root in KWJA's emission convention.
fn decode_dependency_parents(scores: &Tensor) -> Result<Vec<i32>> {
    use candle_core::DType;
    // (1, W, W, 1) → squeeze trailing 1 → (1, W, W) → argmax along axis 2 → (1, W)
    let s = scores.squeeze(3).map_err(Error::from)?;
    let argmax = s
        .argmax_keepdim(2)
        .map_err(Error::from)?
        .squeeze(2)
        .map_err(Error::from)?
        .squeeze(0)
        .map_err(Error::from)?
        .to_dtype(DType::I64)
        .map_err(Error::from)?;
    let raw: Vec<i64> = argmax.to_vec1().map_err(Error::from)?;
    Ok(raw.into_iter().map(|p| p as i32).collect())
}

/// Group morphemes into BasePhrases and Phrases.
///
/// Heuristic for v0.1 (no LoRA base_phrase_feature_tagger):
///   - A BasePhrase is a content head + its trailing function-word tail
///     (particles 助詞 / aux verbs 助動詞 / suffixes 接尾辞).
///   - Walk the morpheme list left-to-right. Start a new BasePhrase whenever
///     we hit a content word AND the current BasePhrase already has one.
///   - For v0.1 each BasePhrase becomes its own Phrase. Bunsetsu-vs-kihon-ku
///     coarsening is a v0.2 concern.
///
/// `dep_parents[i]` is the parent word index predicted by the word module.
/// We translate that to "parent BasePhrase id" for both BasePhrase.head and
/// Phrase.head: `head = bp_of(dep_parents[head_morpheme_of(bp)])` if the
/// pointed-at word lives in a different BasePhrase, else `-1` (root).
/// Emit a `NE` feature on the BasePhrase that contains `morphemes[start]`.
/// Surface = concat of `morphemes[start..end]`. Format matches KWJA-Python:
/// `bp.features.NE = "{TYPE}:{surface}"`.
fn emit_ne_feature(
    base_phrases: &mut [BasePhrase],
    word_to_bp: &[usize],
    morphemes: &[Morpheme],
    start: usize,
    end: usize,
    ne_type: &str,
) {
    if start >= morphemes.len() || end <= start || ne_type.is_empty() {
        return;
    }
    let surface: String = morphemes[start..end.min(morphemes.len())]
        .iter()
        .map(|m| m.surface.as_str())
        .collect();
    let bp_idx = match word_to_bp.get(start).copied() {
        Some(b) if b < base_phrases.len() => b,
        _ => return,
    };
    base_phrases[bp_idx].features.push(crate::document::KeyValue {
        key: "NE".to_string(),
        value: format!("{ne_type}:{surface}"),
    });
}

fn reconstruct_phrases(
    morphemes: &[Morpheme],
    dep_parents: &[i32],
    dep_type_per_word: &[String],
    bp_features_per_word: &[Vec<crate::document::KeyValue>],
    word_features_per_morph: &[Vec<String>],
) -> (Vec<Phrase>, Vec<BasePhrase>) {
    if morphemes.is_empty() {
        return (vec![], vec![]);
    }

    // First pass: group word indices into BasePhrases.
    //
    // KWJA's word_feature_tagger predicts a "基本句-区切" feature on the
    // morpheme that ends each base_phrase. When those predictions are
    // available, we group by feature boundaries (matches KWJA-Python's
    // emission). Falling back to the content-word heuristic only when
    // features aren't available (raw-text path or empty predictions).
    let use_feature_boundaries = !word_features_per_morph.is_empty()
        && word_features_per_morph.len() == morphemes.len();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for (i, m) in morphemes.iter().enumerate() {
        if use_feature_boundaries {
            current.push(i);
            // KWJA convention: "基本句-区切" marks the LAST morpheme of the
            // base_phrase, so we close the group AFTER pushing this one.
            if word_features_per_morph[i].iter().any(|f| f == "基本句-区切") {
                groups.push(std::mem::take(&mut current));
            }
        } else {
            // Heuristic fallback: start a new group whenever a content word
            // shows up after the current group already has one.
            let is_content = !is_function_pos(&m.pos);
            if is_content && !current.is_empty() {
                groups.push(std::mem::take(&mut current));
            }
            current.push(i);
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }

    // word_idx → bp_id lookup for translating dep_parents to head pointers.
    let mut word_to_bp: Vec<usize> = vec![0; morphemes.len()];
    for (bp_id, group) in groups.iter().enumerate() {
        for &widx in group {
            word_to_bp[widx] = bp_id;
        }
    }

    // Build BasePhrases. Compute a refined head heuristic per bp:
    //   - last bp = root (-1)
    //   - bp whose content head is 形容詞/連体詞 (modifier-only) → next bp
    //   - everything else → last bp
    let last_bp = (groups.len().saturating_sub(1)) as i32;
    let mut base_phrases = Vec::with_capacity(groups.len());
    for (bp_id, group) in groups.iter().enumerate() {
        let group_morphs: Vec<Morpheme> = group.iter().map(|&i| morphemes[i].clone()).collect();
        let surface: String = group_morphs.iter().map(|m| m.surface.as_str()).collect();
        let head_word_idx = *group.iter().find(|&&i| !is_function_pos(&morphemes[i].pos))
            .unwrap_or(&group[0]);
        let head_pos = morphemes.get(head_word_idx).map(|m| m.pos.as_str()).unwrap_or("");

        let head: i32 = if bp_id as i32 == last_bp {
            -1
        } else if matches!(head_pos, "形容詞" | "連体詞")
            && (bp_id as i32 + 1) <= last_bp
        {
            // Modifier-only bp attaches to immediate next bp.
            (bp_id + 1) as i32
        } else {
            // Subject/object/etc → predicate (last bp).
            last_bp
        };

        // BP dep_type: read KWJA's per-word dep_type prediction at the
        // BP's head morpheme. Fall back to "D" (the most common) if the
        // logits aren't available (e.g. heuristic-only callers).
        let dep_type = dep_type_per_word
            .get(head_word_idx)
            .cloned()
            .unwrap_or_else(|| "D".into());

        // BP features: same — read at the BP's head morpheme. Empty if
        // logits unavailable.
        let features = bp_features_per_word.get(head_word_idx).cloned().unwrap_or_default();

        base_phrases.push(BasePhrase {
            id: bp_id as u32,
            surface,
            head,
            dep_type,
            morphemes: group_morphs,
            features,
            relations: vec![],
        });
    }
    // Suppress unused-binding warning when KWJA dep_parents is heuristic-only.
    let _ = (dep_parents, &word_to_bp);

    // Phrase = BasePhrase 1:1 for v0.1. Real KWJA grouping uses base-phrase
    // features (the LoRA head we don't implement); coarsening is v0.2.
    let phrases: Vec<Phrase> = base_phrases
        .iter()
        .map(|bp| Phrase {
            id: bp.id,
            surface: bp.surface.clone(),
            head: bp.head,
            dep_type: bp.dep_type.clone(),
            base_phrases: vec![bp.clone()],
            morphemes: bp.morphemes.clone(),
        })
        .collect();

    (phrases, base_phrases)
}

/// Function-word POS tags. Both KWJA and Sudachi conventions are accepted
/// here so the same heuristic works whether morphemes come from Sudachi
/// (`補助記号` for punctuation) or from KWJA argmax (`特殊`).
/// Content words start a new BasePhrase; function words attach to the
/// preceding head.
fn is_function_pos(pos: &str) -> bool {
    matches!(
        pos,
        "助詞" | "助動詞" | "接尾辞" | "特殊" | "補助記号" | "記号"
    )
}

/// Right-headed Japanese dep heuristic at the morpheme level. The last
/// morpheme is root; every other morpheme depends on it. Used as a
/// stand-in for proper KWJA dep argmax on Sudachi-tokenized input where
/// the argmax is noisy. BasePhrase-level dep heads are computed
/// separately in `reconstruct_phrases` from POS info.
fn right_headed_dep_for_morphemes(morphemes: &[Morpheme]) -> Vec<i32> {
    if morphemes.is_empty() {
        return vec![];
    }
    let last = (morphemes.len() - 1) as i32;
    (0..morphemes.len() as i32)
        .map(|i| if i == last { -1 } else { last })
        .collect()
}

/// Argmax along the last dim of (1, T, L) logits → Vec<u32> length T.
fn argmax_per_word(t: &Tensor) -> Result<Vec<u32>> {
    use candle_core::DType;
    let last_dim = t.rank() - 1;
    let argmax = t
        .argmax_keepdim(last_dim)
        .map_err(Error::from)?
        .squeeze(last_dim)
        .map_err(Error::from)?
        .squeeze(0)
        .map_err(Error::from)?
        .to_dtype(DType::U32)
        .map_err(Error::from)?;
    argmax.to_vec1::<u32>().map_err(Error::from)
}

/// For each KWJA word_id, find its byte-range from the subwords that map
/// to it. `offsets[i]` is `(start, end)` bytes of subword `i`. Returns
/// Vec of length `num_words`; entries with no subwords get `(0, 0)`.
fn compute_word_spans(
    offsets: &[(usize, usize)],
    word_ids: &[Option<u32>],
    num_words: usize,
) -> Vec<(usize, usize)> {
    let mut spans = vec![(usize::MAX, 0usize); num_words];
    for (i, &wid) in word_ids.iter().enumerate() {
        let Some(wid) = wid else { continue };
        let wid = wid as usize;
        if wid >= num_words {
            continue;
        }
        let (s, e) = offsets[i];
        if spans[wid].0 == usize::MAX {
            spans[wid] = (s, e);
        } else {
            spans[wid].0 = spans[wid].0.min(s);
            spans[wid].1 = spans[wid].1.max(e);
        }
    }
    for span in spans.iter_mut() {
        if span.0 == usize::MAX {
            *span = (0, 0);
        }
    }
    spans
}

/// Find the KWJA word whose span has the largest byte-overlap with the
/// given Sudachi morpheme span `(m_start, m_end)`. Returns `None` if no
/// KWJA word overlaps the morpheme at all (rare — would mean a tokenizer
/// disagreement at the character level).
fn best_kwja_word_for_span(
    m_start: usize,
    m_end: usize,
    kwja_spans: &[(usize, usize)],
) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None; // (kwja_word, overlap)
    for (i, &(ws, we)) in kwja_spans.iter().enumerate() {
        let overlap_start = m_start.max(ws);
        let overlap_end = m_end.min(we);
        if overlap_start >= overlap_end {
            continue;
        }
        let overlap = overlap_end - overlap_start;
        if best.map_or(true, |(_, b)| overlap > b) {
            best = Some((i, overlap));
        }
    }
    best.map(|(i, _)| i)
}

/// Sudachi UniDic uses "*" for empty conjtype/conjform/subpos; production
/// emits empty strings. Normalize.
fn clean_wildcard(s: Option<&str>) -> String {
    match s {
        None | Some("*") | Some("") => String::new(),
        Some(v) => v.to_string(),
    }
}

/// Sudachi UniDic subpos → KWJA subpos. Most pass through; Sudachi-specific
/// values without a KWJA equivalent map to "*" so JumanDIC enrichment in
/// jisho-parse-rs can fill in a more-specific subpos when available.
fn sudachi_to_kwja_subpos(s: Option<&str>) -> String {
    match s {
        // Sudachi placeholders / generic values KWJA emits as wildcard.
        None | Some("") | Some("*") | Some("一般") | Some("非自立可能") => "*".to_string(),
        Some(v) => v.to_string(),
    }
}

/// Sudachi UniDic top-level POS → KWJA POS. Production stores KWJA POS in
/// passage_parse_tree.tree.morphemes[*].pos; we use Sudachi-tokenized input
/// so a deterministic mapping keeps byte equivalence on the common cases.
pub fn sudachi_to_kwja_pos(sudachi_pos: &str) -> &'static str {
    match sudachi_pos {
        "名詞" => "名詞",
        "動詞" => "動詞",
        "形容詞" => "形容詞",
        "形状詞" => "形容詞",  // Sudachi 形状詞 (na-adjectives) → KWJA 形容詞
        "副詞" => "副詞",
        "連体詞" => "連体詞",
        "接続詞" => "接続詞",
        "感動詞" => "感動詞",
        "助詞" => "助詞",
        "助動詞" => "助動詞",
        "接頭辞" => "接頭辞",
        "接尾辞" => "接尾辞",
        "代名詞" => "指示詞",  // Sudachi 代名詞 (pronouns) → KWJA 指示詞
        "補助記号" => "特殊",  // punctuation
        "記号" => "特殊",
        "空白" => "特殊",
        _ => "特殊",
    }
}

/// Split a morpheme list into per-sentence chunks at sentence-final
/// punctuation. The terminator stays with the preceding chunk (KWJA-Python
/// emits sentences inclusive of `。`).
fn split_at_sentence_boundaries(morphs: &[SudachiMorpheme]) -> Vec<Vec<SudachiMorpheme>> {
    let mut chunks = Vec::new();
    let mut current: Vec<SudachiMorpheme> = Vec::new();
    for m in morphs {
        let is_terminator = matches!(m.surface.as_str(), "。" | "！" | "？" | "!" | "?");
        current.push(m.clone());
        if is_terminator {
            chunks.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

fn argmax_last_dim(t: &Tensor) -> Result<Vec<u32>> {
    use candle_core::DType;
    let argmax = t
        .argmax_keepdim(t.rank() - 1)
        .map_err(Error::from)?
        .squeeze(t.rank() - 1)
        .map_err(Error::from)?
        .squeeze(0)
        .map_err(Error::from)?
        .to_dtype(DType::U32)
        .map_err(Error::from)?;
    argmax.to_vec1::<u32>().map_err(Error::from)
}

fn decode_morphemes(text: &str, w: &WordLogits) -> Result<Vec<Morpheme>> {
    if w.num_words == 0 {
        return Ok(vec![]);
    }
    let pos_ids = argmax_last_dim(&w.pos_logits)?;
    let subpos_ids = argmax_last_dim(&w.subpos_logits)?;
    let conjtype_ids = argmax_last_dim(&w.conjtype_logits)?;
    let conjform_ids = argmax_last_dim(&w.conjform_logits)?;

    let labels = &*LABELS;
    let mut morphemes = Vec::with_capacity(w.num_words);

    for word_idx in 0..w.num_words {
        // Find subwords belonging to this word, gather surface + reading.
        let surface = collect_word_surface(text, &w.encoded.offsets, &w.encoded.word_ids, word_idx);
        let pos = labels
            .pos
            .get(pos_ids[word_idx] as usize)
            .cloned()
            .unwrap_or_default();
        let subpos = labels
            .subpos
            .get(subpos_ids[word_idx] as usize)
            .cloned()
            .unwrap_or_default();
        let conjtype = labels
            .conjtype
            .get(conjtype_ids[word_idx] as usize)
            .cloned()
            .unwrap_or_default();
        let conjform = labels
            .conjform
            .get(conjform_ids[word_idx] as usize)
            .cloned()
            .unwrap_or_default();
        morphemes.push(Morpheme {
            surface: surface.clone(),
            // Reading deferred to follow-up — KWJA's reading tagger emits
            // per-subword classes that map to surface chars via a non-trivial
            // index; placeholder = surface.
            reading: surface.clone(),
            // Lemma deferred — same reason as reading.
            lemma: surface,
            pos,
            subpos,
            conjtype,
            conjform,
            semantics: vec![],
            // Raw-text path doesn't populate word_features; ParseMorphemes does.
            features: vec![],
        });
    }
    Ok(morphemes)
}

fn collect_word_surface(
    text: &str,
    offsets: &[(usize, usize)],
    word_ids: &[Option<u32>],
    target_word: usize,
) -> String {
    let target = target_word as u32;
    let mut start = usize::MAX;
    let mut end = 0usize;
    for (i, &wid) in word_ids.iter().enumerate() {
        if wid == Some(target) {
            let (s, e) = offsets[i];
            start = start.min(s);
            end = end.max(e);
        }
    }
    if start == usize::MAX {
        return String::new();
    }
    text.get(start..end).unwrap_or("").to_string()
}

fn sudachi_to_sentence(sudachi: &[SudachiMorpheme]) -> Sentence {
    let text: String = sudachi.iter().map(|m| m.surface.as_str()).collect();
    let morphemes = sudachi
        .iter()
        .map(|m| Morpheme {
            surface: m.surface.clone(),
            reading: m.reading.clone(),
            lemma: m.lemma.clone(),
            // Sudachi POS hierarchy: top-level → KWJA's `pos`,
            // second level → `subpos`. KWJA convention varies — for v0.1
            // ship a straightforward mapping and let Task 22 e2e tell us
            // if it diverges.
            pos: m.pos.first().cloned().unwrap_or_default(),
            subpos: m.pos.get(1).cloned().unwrap_or_default(),
            conjtype: m.conjtype.clone(),
            conjform: m.conjform.clone(),
            semantics: vec![],
            features: vec![],
        })
        .collect();
    Sentence {
        text,
        phrases: vec![],
        base_phrases: vec![],
        morphemes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn checkpoint_dir() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap()).join(".local/share/jisho/checkpoints")
    }

    #[test]
    fn pipeline_loads_both_modules() {
        let dir = checkpoint_dir();
        if !dir.join("char.safetensors").exists() {
            eprintln!("skipping: checkpoints missing");
            return;
        }
        let _ = Pipeline::load(&dir).unwrap();
    }

    #[test]
    fn pipeline_parse_returns_one_item_per_input() {
        let dir = checkpoint_dir();
        if !dir.join("char.safetensors").exists() {
            return;
        }
        let p = Pipeline::load(&dir).unwrap();
        let items = p.parse(&["今日は晴れです", "ありがとう"]).unwrap();
        assert_eq!(items.len(), 2);
        for item in &items {
            match item {
                ParseItem::Tree(doc) => {
                    assert_eq!(doc.sentences.len(), 1);
                    let s = &doc.sentences[0];
                    assert!(!s.morphemes.is_empty());
                    // BasePhrase + Phrase reconstruction populates these now.
                    assert!(!s.base_phrases.is_empty(),
                        "base_phrases should be populated by reconstruct_phrases");
                    assert_eq!(s.base_phrases.len(), s.phrases.len(),
                        "v0.1: each BasePhrase becomes its own Phrase");
                    // BasePhrase morphemes should sum to the sentence's flat list.
                    let bp_morph_count: usize = s.base_phrases.iter().map(|bp| bp.morphemes.len()).sum();
                    assert_eq!(bp_morph_count, s.morphemes.len(),
                        "every morpheme must belong to exactly one base_phrase");
                }
                ParseItem::Error { kind, message } => {
                    panic!("unexpected parse error: {kind}: {message}");
                }
            }
        }
    }

    #[test]
    fn function_pos_attaches_to_preceding_content() {
        // Pure unit test of the grouping heuristic without checkpoints.
        let morphemes = vec![
            mk_morph("今日", "名詞"),
            mk_morph("は", "助詞"),
            mk_morph("晴れ", "名詞"),
            mk_morph("です", "助動詞"),
        ];
        let dep_parents = vec![-1, 0, -1, 2];  // word 1→0, word 3→2, others root
        let dep_types: Vec<String> = vec![];
        let bp_feats: Vec<Vec<crate::document::KeyValue>> = vec![];
        let word_feats: Vec<Vec<String>> = vec![];
        let (phrases, base_phrases) = reconstruct_phrases(
            &morphemes, &dep_parents, &dep_types, &bp_feats, &word_feats,
        );
        // Two content heads (今日, 晴れ) → two BasePhrases.
        assert_eq!(base_phrases.len(), 2);
        assert_eq!(base_phrases[0].surface, "今日は");
        assert_eq!(base_phrases[0].morphemes.len(), 2);
        assert_eq!(base_phrases[1].surface, "晴れです");
        assert_eq!(base_phrases[1].morphemes.len(), 2);
        // Phrase mirrors BasePhrase 1:1 in v0.1.
        assert_eq!(phrases.len(), 2);
        assert_eq!(phrases[0].base_phrases.len(), 1);
    }

    fn mk_morph(surface: &str, pos: &str) -> Morpheme {
        Morpheme {
            surface: surface.into(),
            reading: surface.into(),
            lemma: surface.into(),
            pos: pos.into(),
            subpos: String::new(),
            conjtype: String::new(),
            conjform: String::new(),
            semantics: vec![],
            features: vec![],
        }
    }

    #[test]
    fn parse_morphemes_passthrough() {
        let dir = checkpoint_dir();
        if !dir.join("char.safetensors").exists() {
            return;
        }
        let p = Pipeline::load(&dir).unwrap();
        let sudachi = vec![vec![SudachiMorpheme {
            surface: "今日".into(),
            reading: "きょう".into(),
            lemma: "今日".into(),
            pos: vec!["名詞".into(), "普通名詞".into()],
            conjtype: String::new(),
            conjform: String::new(),
            normalized_form: "今日".into(),
            dictionary_form: "今日".into(),
        }]];
        let items = p.parse_morphemes(&sudachi).unwrap();
        assert_eq!(items.len(), 1);
        match &items[0] {
            ParseItem::Tree(doc) => {
                assert_eq!(doc.sentences[0].morphemes[0].surface, "今日");
                assert_eq!(doc.sentences[0].morphemes[0].pos, "名詞");
            }
            _ => panic!("expected tree"),
        }
    }
}
