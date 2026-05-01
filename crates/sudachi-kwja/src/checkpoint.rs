//! Safetensors checkpoint loader.
//!
//! Owns the file bytes and a candle `Device`. Tensor lookups go through
//! `safetensors::SafeTensors::deserialize` which is zero-copy over the
//! held byte buffer; `get(name)` materializes a `candle_core::Tensor` on
//! the requested device.

use crate::Result;
use crate::error::Error;
use candle_core::{DType, Device, Tensor};
use safetensors::SafeTensors;
use std::path::Path;

pub struct Checkpoint {
    bytes: Vec<u8>,
    device: Device,
}

impl Checkpoint {
    /// Load the checkpoint into memory; tensors stay materialized lazily on
    /// `get()`. Picks the best available device (CUDA if `cuda` feature is
    /// compiled in and a CUDA device is available, else Metal under
    /// `metal`, else CPU). Use `load_with_device` for explicit control.
    pub fn load(path: &Path) -> Result<Self> {
        Self::load_with_device(path, default_device())
    }
}

/// Pick the best inference device. Mirrors `crate::pipeline::default_device`
/// — duplicated here so `Checkpoint::load` can use it without a circular dep.
fn default_device() -> Device {
    #[cfg(feature = "cuda")]
    {
        if let Ok(d) = Device::new_cuda(0) {
            return d;
        }
    }
    #[cfg(feature = "metal")]
    {
        if let Ok(d) = Device::new_metal(0) {
            return d;
        }
    }
    Device::Cpu
}

impl Checkpoint {

    pub fn load_with_device(path: &Path, device: Device) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        // Validate the file parses cleanly so later `get()` calls don't
        // fail with cryptic deserialization errors.
        let _ = SafeTensors::deserialize(&bytes)
            .map_err(|e| Error::SafeTensors(format!("parse {}: {e}", path.display())))?;
        Ok(Self { bytes, device })
    }

    pub fn tensor_names(&self) -> Vec<String> {
        let st = SafeTensors::deserialize(&self.bytes).expect("validated in load");
        st.names().into_iter().map(|s| s.to_string()).collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        let st = SafeTensors::deserialize(&self.bytes).expect("validated in load");
        st.tensor(name).is_ok()
    }

    pub fn get(&self, name: &str) -> Result<Tensor> {
        let st = SafeTensors::deserialize(&self.bytes)
            .map_err(|e| Error::SafeTensors(e.to_string()))?;
        let view = st
            .tensor(name)
            .map_err(|e| Error::Checkpoint(format!("missing tensor {name}: {e}")))?;
        let dtype = candle_dtype(view.dtype());
        Tensor::from_raw_buffer(view.data(), dtype, view.shape(), &self.device)
            .map_err(Error::from)
    }

    pub fn device(&self) -> &Device {
        &self.device
    }
}

fn candle_dtype(dt: safetensors::Dtype) -> DType {
    match dt {
        safetensors::Dtype::F32 => DType::F32,
        safetensors::Dtype::F16 => DType::F16,
        safetensors::Dtype::BF16 => DType::BF16,
        safetensors::Dtype::I64 => DType::I64,
        safetensors::Dtype::U32 => DType::U32,
        safetensors::Dtype::U8 => DType::U8,
        other => panic!("unsupported safetensors dtype: {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn char_path() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local/share/jisho/checkpoints/char.safetensors")
    }

    #[test]
    fn loads_char_checkpoint() {
        let path = char_path();
        if !path.exists() {
            eprintln!("skipping: {path:?} not present (run scripts/convert_checkpoints.py first)");
            return;
        }
        let cp = Checkpoint::load(&path).unwrap();
        let names = cp.tensor_names();
        assert!(!names.is_empty());
        // KWJA's encoder is named "encoder" — check at least one weight under it loads.
        let encoder_weight = names
            .iter()
            .find(|n| n.contains("encoder") && n.ends_with("weight"))
            .expect("expected an encoder.*.weight tensor");
        let _t = cp.get(encoder_weight).unwrap();
    }

    #[test]
    fn missing_tensor_returns_error() {
        let path = char_path();
        if !path.exists() {
            return;
        }
        let cp = Checkpoint::load(&path).unwrap();
        let err = cp.get("nonexistent.tensor.name").unwrap_err();
        assert!(matches!(err, Error::Checkpoint(_)));
    }
}
