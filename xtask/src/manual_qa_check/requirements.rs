pub(super) const REQUIRED_FIELDS: [&str; 11] = [
    "App build",
    "App artifact",
    "macOS version",
    "Machine",
    "Input sample set",
    "Output folder",
    "Config path",
    "History path",
    "License cache path",
    "Tester",
    "Date",
];

pub(super) const REQUIRED_CHECKS: [&str; 26] = [
    "Choose recording conversion",
    "Drag-and-drop conversion",
    "Duplicate output naming",
    "Cancellation",
    "Multi-file queue",
    "Ask source policy",
    "Trash source policy",
    "Failed conversion",
    "Larger output",
    "Reveal output",
    "Sandbox purchase",
    "Empty key activation",
    "Invalid key activation",
    "Valid sandbox activation",
    "Forget license on this Mac",
    "`cargo run -p xtask -- release-check`",
    "`cargo run -p xtask -- file-size-check`",
    "`cargo run -p xtask -- media-policy-check`",
    "`cargo run -p xtask -- privacy-policy-check`",
    "`cargo run -p xtask -- website-check`",
    "`cargo run -p xtask -- benchmark --input <sample> --output-dir <tmp>`",
    "`cargo run -p xtask -- manual-qa-check`",
    "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`",
    "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`",
    "`cargo run -p xtask -- macos-signing-check`",
    "Gatekeeper open test",
];

pub(super) fn require_labels(
    prefix: &str,
    required: &[&str],
    labels: &[String],
    missing: &mut Vec<String>,
) {
    for label in required {
        if !labels.iter().any(|value| value == label) {
            missing.push(format!("{prefix}: {label}"));
        }
    }
}
