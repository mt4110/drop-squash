use dropsquash_core::{EncodeResult, MaskPlan, Result, SecureShareOptions};
use dropsquash_privacy::{PrivacyReceipt, SecureShareReceipt};

use crate::commands::dto::ReceiptSummary;

pub fn save_receipt(
    result: &EncodeResult,
    secure_share: Option<&SecureShareOptions>,
    mask_plan: Option<&MaskPlan>,
    write_privacy_receipt: bool,
) -> Result<Option<ReceiptSummary>> {
    if let Some(options) = secure_share {
        let saved = match mask_plan {
            Some(plan) => SecureShareReceipt::save_for_result_with_mask_plan(result, options, plan),
            None => SecureShareReceipt::save_for_result(result, options),
        }?;
        return Ok(Some(ReceiptSummary::new(saved, "secure-share")));
    }
    if !write_privacy_receipt {
        return Ok(None);
    }
    PrivacyReceipt::save_for_result(result).map(|path| Some(ReceiptSummary::new(path, "privacy")))
}

#[cfg(test)]
mod tests;
