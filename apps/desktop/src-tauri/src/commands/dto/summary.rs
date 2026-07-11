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
}

impl ConversionSummary {
    pub fn new(result: EncodeResult, decision: SourceActionDecision) -> Self {
        Self {
            source_action: decision.action,
            source_path: decision.source_path.display().to_string(),
            output_path: result.output_path.display().to_string(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            saved_bytes: result.saved_bytes(),
            reduction_percent: result.reduction_percent(),
        }
    }
}
