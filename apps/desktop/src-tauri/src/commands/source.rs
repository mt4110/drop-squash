use dropsquash_core::{EncodeResult, SourcePolicy};
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
            conversion_succeeded: true,
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
