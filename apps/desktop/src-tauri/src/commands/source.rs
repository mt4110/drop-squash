use std::path::PathBuf;

use dropsquash_core::{EncodeResult, SourcePolicy};
use dropsquash_encoder::verify_output;
use dropsquash_platform::TrashService;
use dropsquash_postprocess::{
    decide_source_action, SourceAction, SourceActionDecision, SourceSafety,
};

use super::format_error;

pub fn handle_source_action(
    result: &EncodeResult,
    source_policy: SourcePolicy,
) -> Result<SourceActionDecision, String> {
    let decision = decide_source_action(
        result.input_path.clone(),
        source_policy,
        SourceSafety {
            conversion_succeeded: result.success,
            output_exists: result.output_path.exists(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
        },
    );
    if decision.action == SourceAction::MoveOriginalToTrash {
        TrashService
            .move_to_trash(&decision.source_path)
            .map_err(format_error)?;
    }
    Ok(decision)
}

pub fn trash_original(
    source_path: String,
    output_path: String,
) -> Result<SourceActionDecision, String> {
    let source_path = PathBuf::from(source_path);
    let output_path = PathBuf::from(output_path);
    let verification = verify_output(&source_path, &output_path).map_err(format_error)?;
    let decision = decide_source_action(
        source_path,
        SourcePolicy::Trash,
        SourceSafety {
            conversion_succeeded: verification.is_valid_output,
            output_exists: verification.output_exists,
            original_bytes: verification.original_bytes,
            output_bytes: verification.output_bytes,
        },
    );
    if decision.action != SourceAction::MoveOriginalToTrash {
        return Ok(decision);
    }
    TrashService
        .move_to_trash(&decision.source_path)
        .map_err(format_error)?;
    Ok(decision)
}

#[cfg(test)]
mod tests;
