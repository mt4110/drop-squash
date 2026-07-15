use super::{
    includes_packaged_app, parse_args, pending_rows, phases, row_notes, section::grouped, USAGE,
};

#[test]
fn finds_pending_result_rows() {
    let rows = pending_rows(
        "| App build | DropSquash 0.1.0 git abc1234 |\n\
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |\n\
| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |\n\
| Benchmark sample set | Three samples | recorded |\n",
    );

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].0, "Choose recording conversion");
    assert_eq!(rows[1].0, "`cargo run -p xtask -- manual-qa-check`");
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn groups_pending_rows_by_section() {
    let groups = grouped(
        &[
            (
                "Choose recording conversion".to_string(),
                "Small `.mov` screen recording".to_string(),
            ),
            (
                "Valid sandbox activation".to_string(),
                "Sandbox activation".to_string(),
            ),
            (
                "Benchmark sample set".to_string(),
                "Three samples".to_string(),
            ),
            (
                "Codesign verification".to_string(),
                "Public DMG/app artifact verifies".to_string(),
            ),
        ],
        None,
    );

    assert_eq!(groups[0].0, "Packaged App");
    assert_eq!(groups[1].0, "License Sandbox");
    assert_eq!(groups[2].0, "Benchmark Evidence");
    assert_eq!(groups[3].0, "Distribution And Signing");
}

#[test]
fn parses_optional_section_filter() {
    let (path, section) = parse_args(vec![
        "/tmp/manual.md".into(),
        "--section".into(),
        "packaged-app".into(),
    ])
    .unwrap();

    assert_eq!(path, std::path::PathBuf::from("/tmp/manual.md"));
    assert_eq!(section.as_deref(), Some("packaged-app"));
}

#[test]
fn filters_grouped_rows_to_requested_section() {
    let groups = grouped(
        &[(
            "Choose recording conversion".to_string(),
            "Small `.mov` screen recording".to_string(),
        )],
        Some("packaged-app"),
    );

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].0, "Packaged App");
}

#[test]
fn filters_benchmark_rows_to_requested_section() {
    let groups = grouped(
        &[("Benchmark sample set".to_string(), "Three samples".to_string())],
        Some("benchmark"),
    );

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].0, "Benchmark Evidence");
}

#[test]
fn reports_filter_for_known_labels() {
    assert_eq!(super::suggested_section("Choose recording conversion"), "packaged-app");
    assert_eq!(super::suggested_section("Valid sandbox activation"), "license");
    assert_eq!(super::suggested_section("Benchmark sample set"), "benchmark");
    assert_eq!(super::suggested_section("Codesign verification"), "distribution");
}

#[test]
fn local_proof_filter_includes_packaged_app_and_benchmark() {
    let groups = grouped(
        &[
            (
                "Choose recording conversion".to_string(),
                "Small `.mov` screen recording".to_string(),
            ),
            ("Benchmark sample set".to_string(), "Three samples".to_string()),
            (
                "Codesign verification".to_string(),
                "Public DMG/app artifact verifies".to_string(),
            ),
        ],
        Some("local-proof"),
    );

    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].0, "Packaged App");
    assert_eq!(groups[1].0, "Benchmark Evidence");
}

#[test]
fn detects_packaged_app_group() {
    let groups = grouped(
        &[(
            "Choose recording conversion".to_string(),
            "Small `.mov` screen recording".to_string(),
        )],
        Some("packaged-app"),
    );

    assert!(includes_packaged_app(&groups));
}

#[test]
fn reports_row_notes_for_custom_inputs() {
    assert_eq!(
        row_notes::for_label("Failed conversion"),
        Some("note: create a throwaway invalid .mp4 with `cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-manual-qa-invalid.mp4`, then use it outside the benchmark sample set")
    );
}

#[test]
fn finds_packaged_app_artifact_row() {
    let text = "| App artifact | /tmp/DropSquash.dmg |\n";

    assert_eq!(super::field_value(text, "App artifact"), Some("/tmp/DropSquash.dmg"));
}

#[test]
fn finds_license_cache_path_row() {
    let text = "| License cache path | /tmp/license.json |\n";

    assert_eq!(super::field_value(text, "License cache path"), Some("/tmp/license.json"));
}

#[test]
fn finds_output_and_history_rows() {
    let text =
        "| Config path | /tmp/config.json |\n| Output folder | /tmp/output |\n| History path | /tmp/history.jsonl |\n";

    assert_eq!(super::field_value(text, "Config path"), Some("/tmp/config.json"));
    assert_eq!(super::field_value(text, "Output folder"), Some("/tmp/output"));
    assert_eq!(super::field_value(text, "History path"), Some("/tmp/history.jsonl"));
}

#[test]
fn quotes_packaged_app_open_command_path() {
    assert_eq!(
        super::packaged_app::extra_lines(
            "| App artifact | /tmp/QA Path's/DropSquash.dmg |\n"
        )[1],
        "packaged-app open command: open -- '/tmp/QA Path'\\''s/DropSquash.dmg'"
    );
}

#[test]
fn reports_phase_for_packaged_app_rows() {
    assert_eq!(phases::for_label("Choose recording conversion"), Some("Small Sample"));
}

#[test]
fn reports_phase_counts_for_packaged_rows() {
    let counts = phases::counts(&[
        ("Choose recording conversion".to_string(), "small".to_string()),
        ("Cancellation".to_string(), "large".to_string()),
        ("Reveal output".to_string(), "small".to_string()),
    ]);

    assert_eq!(counts, vec![("Small Sample", 2), ("Large Sample", 1)]);
}

#[test]
fn reports_phase_counts_for_license_rows() {
    let counts = phases::counts(&[
        ("Sandbox product setup".to_string(), String::new()),
        ("Empty key activation".to_string(), String::new()),
        ("Invalid key activation".to_string(), String::new()),
    ]);

    assert_eq!(counts, vec![("Setup", 1), ("Activation Safety", 2)]);
}

#[test]
fn reports_phase_counts_for_distribution_rows() {
    let counts = phases::counts(&[
        ("Codesign verification".to_string(), String::new()),
        ("Notarization staple verification".to_string(), String::new()),
        ("Gatekeeper open test".to_string(), String::new()),
    ]);

    assert_eq!(
        counts,
        vec![("Signature Verification", 2), ("Gatekeeper", 1)]
    );
}
