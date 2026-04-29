//! [`Tokenizer`] — high-level Sudachi + optimizer wrapper.
//!
//! The single tokenisation entry point that downstream consumers
//! should reach for. Wraps a [`JapaneseDictionary`], runs Sudachi in
//! the requested mode, then applies the configured optimizer
//! [`RuleSet`] to the resulting token stream.
//!
//! Consumers that need the raw Sudachi morpheme list (e.g.,
//! sudachi-search's B+C colocation logic) can use [`Tokenizer::raw`]
//! to skip the optimizer pipeline.

use std::sync::Arc;

use crate::lookup::{NoLookup, OptimizerLookup};
use crate::pipeline::{RuleSet, optimize_tokens};
use crate::sudachi::{
    JapaneseDictionary, Mode, StatelessTokenizer, SudachiError, Tokenize,
};
use crate::token::OptimizerToken;

/// Errors surfaced by [`Tokenizer::tokenize`].
#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("sudachi error: {0}")]
    Sudachi(#[from] SudachiError),
}

/// Wraps a Sudachi dictionary + the optimizer rule set chosen at
/// construction. Cheap to clone (Arc'd dictionary, Arc'd rules).
pub struct Tokenizer {
    inner: StatelessTokenizer<Arc<JapaneseDictionary>>,
    rules: Arc<RuleSet>,
    default_mode: Mode,
}

impl Tokenizer {
    /// Build a tokenizer over `dictionary` with [`RuleSet::analysis`]
    /// (every rule). For search consumers, chain
    /// [`Tokenizer::with_rules`].
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self {
        Self {
            inner: StatelessTokenizer::new(dictionary),
            rules: Arc::new(RuleSet::analysis()),
            default_mode: Mode::B,
        }
    }

    /// Override the default rule set. Useful for search consumers
    /// (`RuleSet::search()`) and tests (`RuleSet::empty()`).
    pub fn with_rules(mut self, rules: RuleSet) -> Self {
        self.rules = Arc::new(rules);
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
    /// optimizer pipeline. Uses [`NoLookup`] — to pass vocab data
    /// to the rules, use [`Tokenizer::tokenize_with`].
    pub fn tokenize(&self, text: &str) -> Result<Vec<OptimizerToken>, TokenizeError> {
        self.tokenize_with(text, &NoLookup)
    }

    /// Tokenise `text` in the default mode + apply the optimizer
    /// pipeline with `lookup` providing vocab knowledge.
    pub fn tokenize_with<L: OptimizerLookup>(
        &self,
        text: &str,
        lookup: &L,
    ) -> Result<Vec<OptimizerToken>, TokenizeError> {
        let raw = self.raw_in_mode(text, self.default_mode)?;
        Ok(optimize_tokens(raw, &self.rules, lookup))
    }

    /// Tokenise + optimise in an explicitly chosen mode.
    pub fn tokenize_in_mode<L: OptimizerLookup>(
        &self,
        text: &str,
        mode: Mode,
        lookup: &L,
    ) -> Result<Vec<OptimizerToken>, TokenizeError> {
        let raw = self.raw_in_mode(text, mode)?;
        Ok(optimize_tokens(raw, &self.rules, lookup))
    }

    /// Raw Sudachi output (no optimizer rules). Used by search
    /// consumers that do their own post-processing.
    pub fn raw(&self, text: &str) -> Result<Vec<OptimizerToken>, TokenizeError> {
        self.raw_in_mode(text, self.default_mode)
    }

    /// Raw Sudachi output in a specific mode.
    pub fn raw_in_mode(
        &self,
        text: &str,
        mode: Mode,
    ) -> Result<Vec<OptimizerToken>, TokenizeError> {
        let morphemes = self.inner.tokenize(text, mode, false)?;
        Ok(morphemes
            .iter()
            .map(|m| OptimizerToken::from_morpheme(&m))
            .collect())
    }
}
