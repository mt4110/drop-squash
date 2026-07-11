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
    let mut decision = decide_source_action(
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
        decision = verified_trash_decision(result)?;
    }
    if decision.action == SourceAction::MoveOriginalToTrash {
        TrashService
            .move_to_trash(&decision.source_path)
            .map_err(format_error)?;
    }
    Ok(decision)
}

fn verified_trash_decision(result: &EncodeResult) -> Result<SourceActionDecision, String> {
    if !output_belongs_to_source(&result.input_path, &result.output_path) {
        return Ok(decide_source_action(
            result.input_path.clone(),
            SourcePolicy::Keep,
            SourceSafety {
                conversion_succeeded: false,
                output_exists: result.output_path.exists(),
                original_bytes: result.original_bytes,
                output_bytes: result.output_bytes,
            },
        ));
    }
    let verification =
        verify_output(&result.input_path, &result.output_path).map_err(format_error)?;
    Ok(decide_source_action(
        result.input_path.clone(),
        SourcePolicy::Trash,
        SourceSafety {
            conversion_succeeded: verification.is_valid_output,
            output_exists: verification.output_exists,
            original_bytes: verification.original_bytes,
            output_bytes: verification.output_bytes,
        },
    ))
}

pub fn trash_original(
    source_path: String,
    output_path: String,
) -> Result<SourceActionDecision, String> {
    let source_path = PathBuf::from(source_path);
    let output_path = PathBuf::from(output_path);
    if !output_belongs_to_source(&source_path, &output_path) {
        return Ok(decide_source_action(
            source_path,
            SourcePolicy::Keep,
            SourceSafety {
                conversion_succeeded: false,
                output_exists: output_path.exists(),
                original_bytes: 0,
                output_bytes: 0,
            },
        ));
    }
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

fn output_belongs_to_source(source_path: &std::path::Path, output_path: &std::path::Path) -> bool {
    let Some(source_stem) = source_path.file_stem().and_then(|value| value.to_str()) else {
        return false;
    };
    let Some(output_name) = output_path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    output_name == format!("{source_stem}.squashed.mp4")
        || (output_name.starts_with(&format!("{source_stem}.squashed-"))
            && output_name.ends_with(".mp4"))
}

#[cfg(test)]
mod tests;
