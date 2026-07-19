use objc2_application_services::AXIsProcessTrusted;

pub fn preflight_accessibility_access() -> bool {
    unsafe { AXIsProcessTrusted() }
}
