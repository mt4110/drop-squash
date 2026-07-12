pub(super) const PACKAGED_MACOS_EVIDENCE: &[&str] = &[
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
];

const METADATA_FIELDS: &[&str] = &[
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

pub(super) fn is_metadata_field(check: &str) -> bool {
    METADATA_FIELDS.contains(&check)
}

pub(super) const MANUAL_BLOCKERS: [(&str, &[&str]); 10] = [
    ("Packaged macOS manual QA", PACKAGED_MACOS_EVIDENCE),
    ("Lemon Squeezy product setup", &["Sandbox product setup"]),
    ("Lemon Squeezy sandbox purchase", &["Sandbox purchase"]),
    ("Empty key activation", &["Empty key activation"]),
    ("Valid sandbox activation", &["Valid sandbox activation"]),
    ("Invalid license key handling", &["Invalid key activation"]),
    ("License network failure", &["License network failure"]),
    ("Local license forget", &["Forget license on this Mac"]),
    ("Gatekeeper clean-machine open", &["Gatekeeper open test"]),
    (
        "Benchmark release set",
        &[
            "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>`",
            "Benchmark sample set",
            "Benchmark regression threshold",
        ],
    ),
];
