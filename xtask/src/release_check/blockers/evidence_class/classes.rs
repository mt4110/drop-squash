pub(super) fn matches(blocker: &str, class: &str) -> bool {
    expected(blocker) == Some(class)
}

pub(super) fn expected(blocker: &str) -> Option<&'static str> {
    match blocker {
        "Packaged macOS manual QA" | "Gatekeeper clean-machine open" => Some("Manual packaged-app"),
        "Lemon Squeezy product setup"
        | "Lemon Squeezy sandbox purchase"
        | "Empty key activation"
        | "Valid sandbox activation"
        | "Invalid license key handling"
        | "License network failure"
        | "Expired license refresh"
        | "Local license forget" => Some("License sandbox"),
        "Public website deployment" | "Refund policy finalized" | "Live checkout link" => {
            Some("Public web")
        }
        "Signed DMG" | "Notarized and stapled DMG" => Some("Signing/notarization"),
        "Benchmark release set" => Some("Benchmark"),
        "Published checksum" | "Homebrew cask install" => Some("Distribution"),
        _ => None,
    }
}
