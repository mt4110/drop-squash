use std::path::PathBuf;

use dropsquash_core::EncodeResult;
use dropsquash_postprocess::{SourceAction, SourceActionDecision};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionSummary {
    pub output_path: String,
    pub original_bytes: u64,
    pub output_bytes: u64,
    pub saved_bytes: u64,
    pub reduction_percent: f64,
    pub source_action: SourceAction,
    pub source_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_receipt_path: Option<String>,
}

impl ConversionSummary {
    pub fn new(
        result: EncodeResult,
        decision: SourceActionDecision,
        privacy_receipt_path: Option<PathBuf>,
    ) -> Self {
        Self {
            source_action: decision.action,
            source_path: decision.source_path.display().to_string(),
            output_path: result.output_path.display().to_string(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            saved_bytes: result.saved_bytes(),
            reduction_percent: result.reduction_percent(),
            privacy_receipt_path: privacy_receipt_path.map(|path| path.display().to_string()),
        }
    }
}
