mod io;

use std::path::Path;

use dropsquash_core::EncodeResult;
use serde::{Deserialize, Serialize};

use crate::MetadataPolicy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyReceipt {
    pub input_name: String,
    pub output_name: String,
    pub uploaded_bytes: u64,
    pub metadata_policy: MetadataPolicy,
}

impl From<&EncodeResult> for PrivacyReceipt {
    fn from(result: &EncodeResult) -> Self {
        Self {
            input_name: file_name(&result.input_path),
            output_name: file_name(&result.output_path),
            uploaded_bytes: 0,
            metadata_policy: MetadataPolicy::Preserve,
        }
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests;
