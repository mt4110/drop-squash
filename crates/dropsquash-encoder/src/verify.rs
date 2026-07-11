use std::path::Path;

use dropsquash_core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputVerification {
    pub output_exists: bool,
    pub output_bytes: u64,
    pub original_bytes: u64,
    pub is_smaller_than_original: bool,
}

pub fn verify_output(original: &Path, output: &Path) -> Result<OutputVerification> {
    let original_bytes = std::fs::metadata(original)?.len();
    let output_metadata = std::fs::metadata(output).ok();
    let output_bytes = output_metadata.as_ref().map_or(0, std::fs::Metadata::len);

    Ok(OutputVerification {
        output_exists: output_metadata.is_some(),
        output_bytes,
        original_bytes,
        is_smaller_than_original: output_bytes > 0
            && original_bytes > 0
            && output_bytes < original_bytes,
    })
}
