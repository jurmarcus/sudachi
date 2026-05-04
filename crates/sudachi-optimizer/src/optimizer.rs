//! [`Optimizer`] — high-level Sudachi-plus-pipeline wrapper.
//!
//! The single tokenisation entry point that downstream consumers
//! should reach for. Wraps a [`JapaneseDictionary`], runs Sudachi in
//! the requested mode, then applies the configured optimizer
//! [`Pipeline`] to the resulting morpheme stream.
//!
//! Consumers that need raw Sudachi output (e.g., sudachi-search's
//! B+C colocation logic) can use [`Optimizer::tokenize_raw`] to skip
//! the pipeline.

use std::sync::Arc;

use crate::lookup::{EmptyLexicon, Lexicon};
use crate::pipeline::{Pipeline, optimize};
use crate::sudachi::{
    JapaneseDictionary, Mode, MorphemeList, StatelessTokenizer, SudachiError, Tokenize,
};
use crate::token::Morpheme;

/// Errors surfaced by [`Optimizer::tokenize`].
#[derive(Debug, thiserror::Error)]
pub enum OptimizeError {
    #[error("sudachi error: {0}")]
    Sudachi(#[from] SudachiError),
}

/// Wraps a Sudachi dictionary + the optimizer [`Pipeline`] chosen at
/// construction. Cheap to clone (Arc'd dictionary, Arc'd pipeline).
pub struct Optimizer {
    inner: StatelessTokenizer<Arc<JapaneseDictionary>>,
    pipeline: Arc<Pipeline>,
    default_mode: Mode,
}

impl Optimizer {
    /// Build an optimizer over `dictionary` with [`Pipeline::analysis`]
    /// (every stage). For search consumers chain
    /// [`Optimizer::with_pipeline`].
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self {
        Self {
            inner: StatelessTokenizer::new(dictionary),
            pipeline: Arc::new(Pipeline::analysis()),
            default_mode: Mode::B,
        }
    }

    /// Override the default pipeline. Useful for search consumers
    /// (`Pipeline::search()`) and tests (`Pipeline::empty()`).
    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = Arc::new(pipeline);
        self
    }

    /// Override the default Sudachi splitting mode. Default is
    /// [`Mode::B`] (medium granularity), which most analysis tasks
    /// want. Search consumers usually run B and C separately and
    /// emit both granularities.
    pub fn with_default_mode(mut self, mode: Mode) -> Self {
        self.default_mode = mode;
        self
    }

    /// Tokenise `text` in the default mode + apply the configured
    /// pipeline. Uses [`EmptyLexicon`] — to pass vocab data to
    /// vocab-aware stages, use [`Optimizer::tokenize_with`].
    pub fn tokenize(&self, text: &str) -> Result<Vec<Morpheme>, OptimizeError> {
        self.tokenize_with(text, &EmptyLexicon)
    }

    /// Tokenise `text` in the default mode + apply the pipeline with
    /// `lexicon` providing vocab knowledge.
    pub fn tokenize_with<L: Lexicon>(
        &self,
        text: &str,
        lexicon: &L,
    ) -> Result<Vec<Morpheme>, OptimizeError> {
        let raw = self.tokenize_raw_in(text, self.default_mode)?;
        Ok(optimize(raw, &self.pipeline, lexicon))
    }

    /// Tokenise + optimise in an explicitly chosen mode.
    pub fn tokenize_in<L: Lexicon>(
        &self,
        text: &str,
        mode: Mode,
        lexicon: &L,
    ) -> Result<Vec<Morpheme>, OptimizeError> {
        let raw = self.tokenize_raw_in(text, mode)?;
        Ok(optimize(raw, &self.pipeline, lexicon))
    }

    /// Raw Sudachi output (no pipeline). For search consumers that
    /// do their own post-processing.
    pub fn tokenize_raw(&self, text: &str) -> Result<Vec<Morpheme>, OptimizeError> {
        self.tokenize_raw_in(text, self.default_mode)
    }

    /// Raw Sudachi output in a specific mode (no pipeline).
    pub fn tokenize_raw_in(
        &self,
        text: &str,
        mode: Mode,
    ) -> Result<Vec<Morpheme>, OptimizeError> {
        let morphemes = self.inner.tokenize(text, mode, false)?;
        let lexicon = morphemes.dict().lexicon();
        Ok(morphemes
            .iter()
            .map(|m| Morpheme::from_sudachi(&m, &lexicon))
            .collect())
    }

    /// Tokenise `text` at multiple modes from a **single shared lattice
    /// build**. Returns the raw [`MorphemeList`]s in the same order as
    /// `modes`. No optimizer pipeline is applied — for consumers
    /// (like sudachi-search) that do their own post-processing across
    /// the returned lists.
    ///
    /// Backed by `sudachi::StatelessTokenizer::tokenize_multi_mode`,
    /// which builds the lattice once and applies the mode-specific
    /// `split_path` step per mode. Roughly **1.7× faster** than calling
    /// [`Optimizer::tokenize_raw_in`] once per mode (the dominant
    /// cost — input rewrite, lattice build, best-path resolve,
    /// path-rewrite plugins — runs once, not N times).
    ///
    /// All returned `MorphemeList`s share the same underlying input
    /// buffer via `Rc<RefCell<>>` (one `InputBuffer` clone per call,
    /// not one per mode), keeping multi-mode allocator pressure flat
    /// regardless of how many modes are requested.
    pub fn tokenize_raw_multi_mode(
        &self,
        text: &str,
        modes: &[Mode],
    ) -> Result<Vec<MorphemeList<Arc<JapaneseDictionary>>>, OptimizeError> {
        Ok(self.inner.tokenize_multi_mode(text, modes, false)?)
    }
}
