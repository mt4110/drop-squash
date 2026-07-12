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
        "Empty key activation" => &["Activate is disabled", "local license cache"],
        "Valid sandbox activation" => &[
            "Lemon Squeezy sandbox activation request",
            "disabled while Activating",
            "local license cache",
        ],
        "Invalid license key handling" => &[
            "invalid key",
            "disabled while activating",
            "local license cache",
        ],
        "License network failure" => &[
            "failed activation request",
            "friendly error",
            "preserved local cache",
        ],
        "Local license forget" => &["disabled while forgetting", "returned app state"],
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
