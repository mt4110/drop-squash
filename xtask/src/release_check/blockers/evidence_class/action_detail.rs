pub(super) fn has_required_detail(blocker: &str, action: &str) -> bool {
    required_phrases(blocker)
        .iter()
        .all(|phrase| action.contains(phrase))
}

fn required_phrases(blocker: &str) -> &'static [&'static str] {
    match blocker {
        "Packaged macOS manual QA" => &["public", "DropSquash.dmg"],
        "Lemon Squeezy product setup" => &["intended product", "DropSquash", "license keys"],
        "Lemon Squeezy sandbox purchase" => &[
            "sandbox checkout",
            "intended product",
            "test buyer",
            "order",
        ],
        "Empty key activation" => &[
            "Activate is disabled",
            "raw-key, fingerprint, and instance absence",
        ],
        "Valid sandbox activation" => &[
            "Lemon Squeezy sandbox activation request",
            "disabled while Activating",
            "64-character hex fingerprint",
            "`instance_id`",
            "raw-key absence",
        ],
        "Invalid license key handling" => &[
            "invalid key",
            "disabled while activating",
            "friendly error",
            "raw-key, fingerprint, and instance absence",
        ],
        "License network failure" => &[
            "failed activation request",
            "friendly error",
            "preserved local cache",
            "64-character hex fingerprint",
            "`instance_id`",
            "raw-key absence",
        ],
        "Expired license refresh" => &[
            "expired offline grace cache",
            "reconnect prompt",
            "blocked before starting",
            "raw-key absence",
        ],
        "Local license forget" => &["disabled while forgetting", "returned app state"],
        "Public website deployment" => &[
            "production site",
            "release-status",
            "privacy",
            "pricing",
            "support",
            "download",
        ],
        "Refund policy finalized" => &["final refund policy URL", "checkout goes live"],
        "Live checkout link" => &[
            "public pricing page",
            "tested Lemon Squeezy checkout",
            "intended product",
        ],
        "Signed DMG" => &["public", "DropSquash.dmg", "`codesign`", "Developer ID"],
        "Notarized and stapled DMG" => {
            &["public", "DropSquash.dmg", "`spctl`", "notary", "stapler"]
        }
        "Benchmark release set" => &["absolute CSV path outside repo"],
        "Published checksum" => &["public", "DropSquash.dmg", "SHA-256", "GitHub Release"],
        "Homebrew cask install" => &[
            "brew install",
            "versioned",
            "DropSquash.dmg",
            "matching SHA-256",
            "auto_updates false",
            "zap",
        ],
        _ => &[],
    }
}
