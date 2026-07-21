use serde_json::Value;

mod coverage;
mod fields;

use fields::{equal_u64, positive_u64, required_array_item, required_bool, required_string};

const DEFAULT_EVIDENCE: &str = "docs/build-week-phase5-evidence.json";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = match args.as_slice() {
        [] => DEFAULT_EVIDENCE,
        [path] => path,
        _ => {
            return Err(
                "usage: cargo run -p xtask -- build-week-phase5-check [evidence.json]".into(),
            )
        }
    };
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let evidence: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    check(&evidence)?;
    crate::phase5_doc_honesty_check::check_default()?;
    println!("Build Week Phase 5 evidence is internally consistent");
    Ok(())
}

fn check(evidence: &Value) -> Result<(), String> {
    required_bool(evidence, "/localOnly", true)?;
    required_bool(evidence, "/recognizedPrivateTextStored", false)?;
    required_string(evidence, "/capture/policy", "strict_reveal")?;
    equal_u64(
        evidence,
        "/capture/frameCount",
        "/capture/fullyMaskedFrameCount",
    )?;
    continuity_attestation(evidence)?;
    required_string(
        evidence,
        "/output/exposureMitigation",
        "strict_shield_full_frame_destruction",
    )?;
    required_string(evidence, "/output/audioTrack", "absent")?;
    required_string(evidence, "/output/assetAndTrackMetadata", "absent")?;
    coverage::check(evidence)?;
    for pointer in [
        "/invalidTargetProbe/mp4Published",
        "/adversarialFailure/mp4Published",
        "/adversarialFailure/maskPlanPublished",
        "/adversarialFailure/partialArtifactPublished",
    ] {
        required_bool(evidence, pointer, false)?;
    }
    required_bool(evidence, "/smartMaskResearchOnly/shareable", false)?;
    positive_u64(
        evidence,
        "/smartMaskResearchOnly/rawPlanMissingTruthCoveragePpm",
    )?;
    for claim in [
        "binding thread-safety attestation",
        "completed selective masking",
        "adversarial frame continuity coverage",
        "enterprise audit readiness",
        "leak-zero guarantee",
    ] {
        required_array_item(evidence, "/notYetClaimed", claim)?;
    }
    Ok(())
}
fn continuity_attestation(evidence: &Value) -> Result<(), String> {
    let attested = evidence
        .pointer("/output/captureContinuityAttested")
        .and_then(Value::as_bool)
        .ok_or_else(|| "/output/captureContinuityAttested must be boolean".to_string())?;
    if !attested {
        return Ok(());
    }
    let detail = evidence
        .pointer("/output/temporalContinuityEvidence")
        .and_then(Value::as_str)
        .filter(|value| value.contains("normalized-time") && !value.contains("pending"));
    detail
        .is_some()
        .then_some(())
        .ok_or_else(|| "attested continuity requires normalized-time evidence".to_string())
}

#[cfg(test)]
mod tests;
