use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("candle: {0}")]
    Candle(#[from] candle_core::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("safetensors: {0}")]
    SafeTensors(String),
    #[error("tokenizer: {0}")]
    Tokenizer(String),
    #[error("checkpoint: {0}")]
    Checkpoint(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
