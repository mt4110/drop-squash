use std::path::Path;
use std::time::Duration;

use dropsquash_core::Result;
use dropsquash_media::inspect_mp4;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputVerification {
    pub output_exists: bool,
    pub output_bytes: u64,
    pub original_bytes: u64,
    pub output_extension_is_mp4: bool,
    pub has_mp4_file_type: bool,
    pub has_nonzero_duration: bool,
    pub duration_matches_source: bool,
    pub is_smaller_than_original: bool,
    pub is_valid_output: bool,
}

impl OutputVerification {
    pub fn failure_summary(&self) -> String {
        let mut reasons = Vec::new();
        if !self.output_exists {
            reasons.push("output missing");
        }
        if !self.is_smaller_than_original {
            reasons.push("output is not smaller");
        }
        if !self.output_extension_is_mp4 {
            reasons.push("output extension is not mp4");
        }
        if !self.has_mp4_file_type {
            reasons.push("mp4 file-type box missing");
        }
        if !self.has_nonzero_duration {
            reasons.push("duration is zero or unreadable");
        }
        if !self.duration_matches_source {
            reasons.push("duration differs from source");
        }
        reasons.join(", ")
    }
}

pub fn verify_output(original: &Path, output: &Path) -> Result<OutputVerification> {
    let original_bytes = std::fs::metadata(original)?.len();
    let output_metadata = std::fs::metadata(output).ok();
    let output_bytes = output_metadata.as_ref().map_or(0, std::fs::Metadata::len);
    let output_extension_is_mp4 = output
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mp4"));
    let original_mp4 = inspect_mp4(original);
    let mp4 = inspect_mp4(output);
    let duration_matches_source = duration_close(original_mp4.duration, mp4.duration);
    let is_smaller_than_original =
        output_bytes > 0 && original_bytes > 0 && output_bytes < original_bytes;

    Ok(OutputVerification {
        output_exists: output_metadata.is_some(),
        output_bytes,
        original_bytes,
        output_extension_is_mp4,
        has_mp4_file_type: mp4.has_file_type,
        has_nonzero_duration: mp4.has_nonzero_duration(),
        duration_matches_source,
        is_smaller_than_original,
        is_valid_output: is_smaller_than_original
            && output_extension_is_mp4
            && mp4.has_file_type
            && mp4.has_nonzero_duration()
            && duration_matches_source,
    })
}

fn duration_close(original: Option<Duration>, output: Option<Duration>) -> bool {
    let (Some(original), Some(output)) = (original, output) else {
        return true;
    };
    let original = original.as_secs_f64();
    let output = output.as_secs_f64();
    let tolerance = (original * 0.05).max(1.0);
    (original - output).abs() <= tolerance
}

#[cfg(test)]
mod tests;
