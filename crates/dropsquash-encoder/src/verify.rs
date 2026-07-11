use std::path::Path;

use dropsquash_core::Result;
use serde::{Deserialize, Serialize};

mod mp4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputVerification {
    pub output_exists: bool,
    pub output_bytes: u64,
    pub original_bytes: u64,
    pub output_extension_is_mp4: bool,
    pub has_mp4_file_type: bool,
    pub has_nonzero_duration: bool,
    pub is_smaller_than_original: bool,
    pub is_valid_output: bool,
}

pub fn verify_output(original: &Path, output: &Path) -> Result<OutputVerification> {
    let original_bytes = std::fs::metadata(original)?.len();
    let output_metadata = std::fs::metadata(output).ok();
    let output_bytes = output_metadata.as_ref().map_or(0, std::fs::Metadata::len);
    let output_extension_is_mp4 = output
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mp4"));
    let mp4 = mp4::inspect(output);
    let is_smaller_than_original =
        output_bytes > 0 && original_bytes > 0 && output_bytes < original_bytes;

    Ok(OutputVerification {
        output_exists: output_metadata.is_some(),
        output_bytes,
        original_bytes,
        output_extension_is_mp4,
        has_mp4_file_type: mp4.has_file_type,
        has_nonzero_duration: mp4.has_nonzero_duration,
        is_smaller_than_original,
        is_valid_output: is_smaller_than_original
            && output_extension_is_mp4
            && mp4.has_file_type
            && mp4.has_nonzero_duration,
    })
}

#[cfg(test)]
mod tests;
