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

pub(super) const REQUIRED_CHECKS: [&str; 36] = [
    "Choose recording conversion",
    "Drag-and-drop conversion",
    "Privacy receipt sidecar",
    "Reveal privacy receipt",
    "Duplicate output naming",
    "Cancellation",
    "Multi-file queue",
    "Queued job cancellation",
    "Batch summary",
    "Ask source policy",
    "Trash source policy",
    "Failed conversion",
    "Larger output",
    "Reveal output",
    "Sandbox product setup",
    "Sandbox purchase",
    "Empty key activation",
    "Invalid key activation",
    "Valid sandbox activation",
    "License network failure",
    "Forget license on this Mac",
    "`cargo run -p xtask -- release-check`",
    "`cargo run -p xtask -- file-size-check`",
    "`cargo run -p xtask -- media-policy-check`",
    "`cargo run -p xtask -- privacy-policy-check`",
    "`cargo run -p xtask -- website-check`",
    "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>`",
    "Benchmark sample set",
    "Benchmark regression threshold",
    "`cargo run -p xtask -- manual-qa-check`",
    "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`",
    "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`",
    "`cargo run -p xtask -- macos-signing-check`",
    "Codesign verification",
    "Notarization staple verification",
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
