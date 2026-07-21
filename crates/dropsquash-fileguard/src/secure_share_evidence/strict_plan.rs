use dropsquash_core::{
    required_capture_continuity_watches, strict_shield_exposure_coverage, AppError, Confidence,
    FrameStatus, MaskPlan, MaskPolicy, MaskReason, ObservationSource, RegionPolicy,
};

const STRICT_CAPTURE_ID: &str = "secure-share-recording-local";

pub(super) fn require_redacted_strict_plan(plan: &MaskPlan) -> Result<(), AppError> {
    if plan.policy != MaskPolicy::StrictReveal {
        return Err(invalid("does not use the Strict Shield policy"));
    }
    if plan.capture_id != STRICT_CAPTURE_ID {
        return Err(invalid(
            "uses a non-redacted Strict Shield capture identifier",
        ));
    }
    if plan.schema_version != 2 {
        return Err(invalid(
            "uses a legacy or unsupported Strict Shield plan schema",
        ));
    }
    require_capture_continuity(plan)?;
    require_native_capture_backend(plan)?;
    require_exposure_coverage(plan)?;
    if plan.frames.is_empty() {
        return Err(invalid("contains no Strict Shield capture frames"));
    }
    if plan.frame_size.width == 0 || plan.frame_size.height == 0 {
        return Err(invalid("contains an empty Strict Shield frame size"));
    }
    let expectations = &plan.verification_expectations;
    if !expectations.no_audio
        || !expectations.strip_metadata
        || expectations.verification_policy_version.is_empty()
    {
        return Err(invalid("weakens Strict Shield output verification"));
    }
    let mut previous_time = None;
    for (expected_index, frame) in plan.frames.iter().enumerate() {
        if frame.frame_index != expected_index as u64 {
            return Err(invalid(
                "contains non-contiguous Strict Shield frame indexes",
            ));
        }
        if previous_time.is_some_and(|time| frame.presentation_time_ns <= time) {
            return Err(invalid("contains non-monotonic Strict Shield frame times"));
        }
        previous_time = Some(frame.presentation_time_ns);
        if frame.frame_status != FrameStatus::Complete {
            return Err(invalid("contains a non-complete capture frame"));
        }
        let [region] = frame.regions.as_slice() else {
            return Err(invalid("contains non-redacted Strict Shield geometry"));
        };
        if region.policy != RegionPolicy::Unknown
            || region.reason != MaskReason::UnknownRegion
            || region.sources.as_slice() != [ObservationSource::ScreenCaptureKitFrame]
            || region.confidence != Confidence::CERTAIN
            || region.expansion_px != 0
            || region.rect.x != 0
            || region.rect.y != 0
            || region.rect.width != plan.frame_size.width
            || region.rect.height != plan.frame_size.height
        {
            return Err(invalid("contains non-redacted Strict Shield geometry"));
        }
    }
    Ok(())
}

fn require_exposure_coverage(plan: &MaskPlan) -> Result<(), AppError> {
    let coverage = &plan.audit.exposure_coverage;
    let expected = strict_shield_exposure_coverage();
    if coverage.len() != expected.len() {
        return Err(invalid("has ambiguous exposure coverage evidence"));
    }
    for expected in expected {
        let mut items = coverage.iter().filter(|item| item.path == expected.path);
        let Some(item) = items.next() else {
            return Err(invalid("is missing exposure coverage evidence"));
        };
        if items.next().is_some()
            || item.status != expected.status
            || item.mitigation != expected.mitigation
        {
            return Err(invalid("weakens exposure coverage evidence"));
        }
    }
    Ok(())
}

fn require_capture_continuity(plan: &MaskPlan) -> Result<(), AppError> {
    if plan.audit.capture_continuity_attested != Some(true) {
        return Err(invalid("is missing capture continuity attestation"));
    }
    let expected = required_capture_continuity_watches();
    let watches = &plan.audit.capture_continuity_watches;
    if watches.len() != expected.len()
        || expected
            .iter()
            .any(|watch| !watches.iter().any(|recorded| recorded == watch))
    {
        return Err(invalid("is missing capture continuity watch evidence"));
    }
    Ok(())
}

fn require_native_capture_backend(plan: &MaskPlan) -> Result<(), AppError> {
    if plan.verification_expectations.verification_policy_version != "phase5-native-bridge-v1" {
        return Ok(());
    }
    if plan.audit.capture_backend.as_deref() == Some("apple_native_capture_v1") {
        return Ok(());
    }
    Err(invalid("is missing Apple-native capture boundary evidence"))
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share evidence {reason}"))
}
