use std::path::PathBuf;

use dropsquash_core::EncodeResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyReceipt {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub uploaded_bytes: u64,
}

impl From<&EncodeResult> for PrivacyReceipt {
    fn from(result: &EncodeResult) -> Self {
        Self {
            input_path: result.input_path.clone(),
            output_path: result.output_path.clone(),
            uploaded_bytes: 0,
        }
    }
}
