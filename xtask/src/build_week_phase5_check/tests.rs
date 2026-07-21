use super::check;
use serde_json::json;

#[test]
fn rejects_shareable_smart_mask_research() {
    let evidence = json!({
        "localOnly": true, "recognizedPrivateTextStored": false,
        "capture": { "policy": "strict_reveal", "frameCount": 1, "fullyMaskedFrameCount": 1 },
        "output": { "captureContinuityAttested": false, "exposureMitigation": "strict_shield_full_frame_destruction", "audioTrack": "absent", "assetAndTrackMetadata": "absent", "requiredExposurePaths": paths(), "exposureClassification": coverage() },
        "invalidTargetProbe": { "mp4Published": false },
        "smartMaskResearchOnly": { "shareable": true, "rawPlanMissingTruthCoveragePpm": 1 },
        "notYetClaimed": ["binding thread-safety attestation", "completed selective masking", "adversarial frame continuity coverage", "enterprise audit readiness", "leak-zero guarantee"]
    });
    assert!(check(&evidence).is_err());
}

#[test]
fn rejects_missing_exposure_boundary() {
    let mut rows = coverage();
    rows[0]["boundary"] = json!("");
    let evidence =
        json!({ "output": { "requiredExposurePaths": paths(), "exposureClassification": rows } });
    assert!(super::coverage::check(&evidence).is_err());
}

#[test]
fn rejects_unknown_required_exposure_path() {
    let mut paths = paths();
    paths.push("invented_claim".to_string());
    let evidence = json!({ "output": { "requiredExposurePaths": paths, "exposureClassification": coverage() } });
    assert!(super::coverage::check(&evidence).is_err());
}

#[test]
fn rejects_unknown_exposure_classification_path() {
    let mut rows = coverage();
    rows.push(json!({
        "path": "invented_claim",
        "outputPixelStatus": "covered",
        "sourceSurfaceStatus": "covered",
        "boundary": "test",
        "evidence": "test"
    }));
    let evidence =
        json!({ "output": { "requiredExposurePaths": paths(), "exposureClassification": rows } });
    assert!(super::coverage::check(&evidence).is_err());
}

#[test]
fn rejects_covered_source_with_pending_evidence() {
    let mut rows = coverage();
    rows[0]["sourceSurfaceStatus"] = json!("covered");
    rows[0]["evidence"] = json!("fixture rerun pending");
    let evidence =
        json!({ "output": { "requiredExposurePaths": paths(), "exposureClassification": rows } });
    assert!(super::coverage::check(&evidence).is_err());
}

#[test]
fn accepts_normalized_time_continuity_attestation() {
    let evidence = json!({
        "output": {
            "captureContinuityAttested": true,
            "temporalContinuityEvidence": "normalized-time fixture verified"
        }
    });
    assert!(super::continuity_attestation(&evidence).is_ok());
}

fn paths() -> Vec<String> {
    coverage()
        .into_iter()
        .map(|row| row["path"].as_str().unwrap().to_string())
        .collect()
}

fn coverage() -> Vec<serde_json::Value> {
    [
        "typed_text", "ime_composition_candidate", "transient_dialog_popover", "notification_overlay",
        "browser_autofill_password_manager", "focus_or_foreground_change", "display_geometry_or_scale_change", "frame_drop_or_discontinuity",
        "target_selection_mistake", "multi_display_scaling_transform", "accessibility_vision_disagreement", "audio_track", "media_metadata", "external_capture_or_endpoint_exfiltration",
    ].into_iter().map(|path| json!({
        "path": path, "outputPixelStatus": if path == "external_capture_or_endpoint_exfiltration" { "not_applicable" } else { "covered" }, "sourceSurfaceStatus": "detected-but-not-covered", "boundary": "test", "evidence": "test"
    })).collect()
}
