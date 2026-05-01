//! kwja-rs: pure-Rust port of KWJA inference (typo + char + word modules).
//!
//! Provides argmax-identical equivalence with KWJA-Python on the modules used
//! by jisho. See README for what's in/out of scope.

pub mod checkpoint;
pub mod constants;
pub mod crf;
pub mod document;
pub mod error;
pub mod model;
pub mod pipeline;
pub mod tokenizer;

pub use checkpoint::Checkpoint;
pub use crf::LinearChainCrf;
pub use document::{
    BasePhrase, Document, KeyValue, Morpheme, ParseItem, Phrase, Relation, Sentence,
};
pub use error::Error;
pub use model::{
    BiaffineDependencyHead, CharModel, DebertaBackbone, SequentialMlpHead, WordLogits, WordModel,
    pool_subwords,
};
pub use pipeline::{BucketingConfig, Pipeline, SudachiMorpheme};
pub use tokenizer::{CharTokenizer, DebertaTokenizer, Encoded};

pub type Result<T> = std::result::Result<T, Error>;
