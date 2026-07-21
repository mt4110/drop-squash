use std::path::PathBuf;

use dropsquash_core::EncodeResult;
use dropsquash_postprocess::{SourceAction, SourceActionDecision};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ReceiptSummary {
    pub path: PathBuf,
    pub kind: &'static str,
}

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
    pub receipt_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_receipt_path: Option<String>,
}

impl ConversionSummary {
    pub fn new(
        result: EncodeResult,
        decision: SourceActionDecision,
        receipt: Option<ReceiptSummary>,
    ) -> Self {
        let receipt_path = receipt
            .as_ref()
            .map(|receipt| receipt.path.display().to_string());
        let receipt_kind = receipt.as_ref().map(|receipt| receipt.kind);
        Self {
            source_action: decision.action,
            source_path: decision.source_path.display().to_string(),
            output_path: result.output_path.display().to_string(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            saved_bytes: result.saved_bytes(),
            reduction_percent: result.reduction_percent(),
            receipt_path: receipt_path.clone(),
            receipt_kind,
            privacy_receipt_path: if receipt_kind == Some("privacy") {
                receipt_path
            } else {
                None
            },
        }
    }
}

impl ReceiptSummary {
    pub fn new(path: PathBuf, kind: &'static str) -> Self {
        Self { path, kind }
    }
}
