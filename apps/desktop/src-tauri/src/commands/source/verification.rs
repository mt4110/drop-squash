use std::path::Path;

use dropsquash_core::SourcePolicy;
use dropsquash_encoder::verify_output;
use dropsquash_postprocess::{
    decide_source_action, SourceAction, SourceActionDecision, SourceSafety,
};

use super::output_name;
use crate::commands::format_error;

pub(super) fn verified_trash_decision(
    source_path: &Path,
    output_path: &Path,
) -> Result<SourceActionDecision, String> {
    if !output_name::belongs_to_source(source_path, output_path) {
        return Ok(keep_decision(
            source_path,
            output_path.exists(),
            "output does not belong to source",
        ));
    }
    let verification = verify_output(source_path, output_path).map_err(format_error)?;
    if !verification.is_valid_output {
        return Ok(keep_decision(
            source_path,
            verification.output_exists,
            &format!(
                "source kept because output verification failed: {}",
                verification.failure_summary()
            ),
        ));
    }
    Ok(decide_source_action(
        source_path.to_path_buf(),
        SourcePolicy::Trash,
        SourceSafety {
            conversion_succeeded: verification.is_valid_output,
            output_exists: verification.output_exists,
            original_bytes: verification.original_bytes,
            output_bytes: verification.output_bytes,
        },
    ))
}

fn keep_decision(source_path: &Path, output_exists: bool, reason: &str) -> SourceActionDecision {
    let mut decision = decide_source_action(
        source_path.to_path_buf(),
        SourcePolicy::Keep,
        SourceSafety {
            conversion_succeeded: false,
            output_exists,
            original_bytes: 0,
            output_bytes: 0,
        },
    );
    decision.action = SourceAction::KeepOriginal;
    decision.reason = reason.to_string();
    decision
}
