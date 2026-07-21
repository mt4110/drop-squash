use serde::Serialize;

use super::ConversionSummary;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConversionOutcome {
    Converted(ConversionSummary),
    KeptOriginal { message: String },
}
