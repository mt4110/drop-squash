pub(super) fn for_blocker(blocker: &str) -> Option<&'static [&'static str]> {
    match blocker {
        "Packaged macOS manual QA" => Some(&["public", "DropSquash.dmg"]),
        "Empty key activation" => Some(&[
            "Activate is disabled",
            "raw-key, fingerprint, and instance absence",
        ]),
        "Valid sandbox activation" => Some(&[
            "Lemon Squeezy sandbox activation request",
            "disabled while Activating",
            "check local license cache",
            "64-character lowercase hex fingerprint",
            "`instance_id`",
            "raw-key absence",
        ]),
        "Invalid license key handling" => Some(&[
            "invalid key",
            "disabled while activating",
            "friendly error",
            "raw-key, fingerprint, and instance absence",
        ]),
        "License network failure" => Some(&[
            "failed activation request",
            "check the friendly error",
            "friendly error",
            "preserved local cache",
            "64-character lowercase hex fingerprint",
            "`instance_id`",
            "raw-key absence",
        ]),
        "Expired license refresh" => Some(&[
            "Attempt conversion",
            "expired offline grace cache",
            "reconnect prompt",
            "blocked before starting",
            "raw-key absence",
        ]),
        "Local license forget" => Some(&[
            "disabled while forgetting",
            "confirm cache removal",
            "observe the trial or locked state",
        ]),
        "Benchmark release set" => Some(&[
            "short, medium, and large",
            "backend",
            "saved percent",
            "duration",
            "speed ratio",
            "absolute CSV path outside repo",
            "20% threshold",
            "same-machine",
            "release candidate baseline",
        ]),
        _ => None,
    }
}
