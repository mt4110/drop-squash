use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExposureCoverage {
    pub path: String,
    pub status: ExposureCoverageStatus,
    pub mitigation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExposureCoverageStatus {
    Covered,
    FailClosed,
    DetectedNotCovered,
    OutOfScope,
}

pub fn strict_shield_exposure_coverage() -> Vec<ExposureCoverage> {
    required_exposure_paths()
        .iter()
        .map(|path| {
            let (status, mitigation) = strict_shield_exposure_status(path);
            ExposureCoverage {
                path: (*path).to_string(),
                status,
                mitigation: mitigation.to_string(),
            }
        })
        .collect()
}

pub fn required_exposure_paths() -> [&'static str; 14] {
    [
        "typed_text",
        "ime_composition_candidate",
        "transient_dialog_popover",
        "notification_overlay",
        "browser_autofill_password_manager",
        "focus_or_foreground_change",
        "display_geometry_or_scale_change",
        "frame_drop_or_discontinuity",
        "accessibility_vision_disagreement",
        "audio_track",
        "media_metadata",
        "target_selection_mistake",
        "multi_display_scaling_transform",
        "external_capture_or_endpoint_exfiltration",
    ]
}

fn strict_shield_exposure_status(path: &str) -> (ExposureCoverageStatus, &'static str) {
    use ExposureCoverageStatus::{Covered, DetectedNotCovered, FailClosed, OutOfScope};
    match path {
        "typed_text"
        | "ime_composition_candidate"
        | "transient_dialog_popover"
        | "notification_overlay"
        | "browser_autofill_password_manager" => (Covered, "strict_shield_full_frame_destruction"),
        "focus_or_foreground_change" | "target_selection_mistake" => {
            (FailClosed, "native_continuity_watch")
        }
        "display_geometry_or_scale_change" | "frame_drop_or_discontinuity" => {
            (FailClosed, "native_frame_continuity_watch")
        }
        "multi_display_scaling_transform" | "accessibility_vision_disagreement" => (
            DetectedNotCovered,
            "strict_shield_does_not_preserve_from_observations",
        ),
        "audio_track" | "media_metadata" => (Covered, "independent_output_verification"),
        "external_capture_or_endpoint_exfiltration" => (OutOfScope, "requires_endpoint_controls"),
        _ => unreachable!("required exposure path is missing a classification"),
    }
}
