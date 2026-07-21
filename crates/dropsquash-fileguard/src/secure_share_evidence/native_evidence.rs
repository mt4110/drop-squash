use dropsquash_core::{AppError, MaskPlan};

const NATIVE_POLICY: &str = "phase5-native-bridge-v1";
const NATIVE_BACKEND: &str = "apple_native_capture_v1";

pub(super) fn require_native_destruction(plan: &MaskPlan) -> Result<(), AppError> {
    if plan.verification_expectations.verification_policy_version != NATIVE_POLICY {
        return Ok(());
    }
    if plan.audit.capture_backend.as_deref() != Some(NATIVE_BACKEND) {
        return Err(invalid("is missing Apple-native capture boundary evidence"));
    }
    let frames = plan.frames.len();
    if plan.audit.native_destroyed_frame_count != frames
        || plan.audit.native_destroyed_region_count != frames
        || plan.audit.native_verified_frame_count != frames
    {
        return Err(invalid("is missing per-frame native destruction evidence"));
    }
    Ok(())
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share evidence {reason}"))
}
