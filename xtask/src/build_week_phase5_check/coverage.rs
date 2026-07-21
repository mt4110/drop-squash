use serde_json::Value;

mod claim_text;
mod exact;
mod fixture_contract;

const REQUIRED: [&str; 14] = [
    "typed_text",
    "ime_composition_candidate",
    "transient_dialog_popover",
    "notification_overlay",
    "browser_autofill_password_manager",
    "focus_or_foreground_change",
    "display_geometry_or_scale_change",
    "frame_drop_or_discontinuity",
    "target_selection_mistake",
    "multi_display_scaling_transform",
    "accessibility_vision_disagreement",
    "audio_track",
    "media_metadata",
    "external_capture_or_endpoint_exfiltration",
];

pub(super) fn check(evidence: &Value) -> Result<(), String> {
    let required_paths = evidence
        .pointer("/output/requiredExposurePaths")
        .and_then(Value::as_array)
        .ok_or("/output/requiredExposurePaths must be an array")?;
    let rows = evidence
        .pointer("/output/exposureClassification")
        .and_then(Value::as_array)
        .ok_or("/output/exposureClassification must be an array")?;
    exact::path_set(required_paths, &REQUIRED, "requiredExposurePaths")?;
    exact::classification_set(rows, &REQUIRED)?;
    for path in REQUIRED {
        if !required_paths
            .iter()
            .any(|value| value.as_str() == Some(path))
        {
            return Err(format!("required exposure path is missing: {path}"));
        }
        let row = rows
            .iter()
            .find(|row| row.get("path").and_then(Value::as_str) == Some(path))
            .ok_or_else(|| format!("missing exposure classification for {path}"))?;
        status(row, "outputPixelStatus", path, output_status(path))?;
        let source = row.get("sourceSurfaceStatus").and_then(Value::as_str);
        if !matches!(
            source,
            Some("covered" | "fail-closed" | "detected-but-not-covered" | "out-of-scope")
        ) {
            return Err(format!(
                "invalid source surface status for {path}: {source:?}"
            ));
        }
        let boundary = row
            .get("boundary")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if boundary.is_empty() {
            return Err(format!("missing exposure boundary for {path}"));
        }
        let evidence = row
            .get("evidence")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if evidence.is_empty() {
            return Err(format!("missing exposure evidence for {path}"));
        }
        claim_text::check(path, source, evidence)?;
    }
    fixture_contract::check()?;
    Ok(())
}

fn output_status(path: &str) -> &str {
    if path == "external_capture_or_endpoint_exfiltration" {
        "not_applicable"
    } else {
        "covered"
    }
}

fn status(row: &Value, field: &str, path: &str, expected: &str) -> Result<(), String> {
    match row.get(field).and_then(Value::as_str) {
        Some(actual) if actual == expected => Ok(()),
        actual => Err(format!(
            "{field} for {path} must be {expected}, found {actual:?}"
        )),
    }
}
