use super::{phases, row_notes, run::parse_args, section::grouped, USAGE};

#[test]
fn finds_pending_result_rows() {
    let rows = super::pending_rows::pending_rows(
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
        &[(
            "Benchmark sample set".to_string(),
            "Three samples".to_string(),
        )],
        Some("benchmark"),
    );

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].0, "Benchmark Evidence");
}

#[test]
fn reports_filter_for_known_labels() {
    assert_eq!(
        super::suggested_section("Choose recording conversion"),
        "packaged-app"
    );
    assert_eq!(
        super::suggested_section("Valid sandbox activation"),
        "license"
    );
    assert_eq!(
        super::suggested_section("Benchmark sample set"),
        "benchmark"
    );
    assert_eq!(
        super::suggested_section("Codesign verification"),
        "distribution"
    );
}

#[test]
fn local_proof_filter_includes_packaged_app_and_benchmark() {
    let groups = grouped(
        &[
            (
                "Choose recording conversion".to_string(),
                "Small `.mov` screen recording".to_string(),
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

    assert!(super::pending_rows::includes_packaged_app(&groups));
}

#[test]
fn reports_row_notes_for_custom_inputs() {
    assert_eq!(
        row_notes::for_label("Failed conversion"),
        Some("note: create a throwaway invalid .mp4 with `cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-manual-qa-invalid.mp4`, then use it outside the benchmark sample set")
    );
}

#[test]
fn prints_focused_gate_for_visible_sections() {
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
        ],
        None,
    );
    let lines = super::gates::for_groups(&groups);

    assert!(lines.contains(
        &"focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section packaged-app"
            .to_string()
    ));
    assert!(lines.contains(
        &"focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section license"
            .to_string()
    ));
}

#[test]
fn finds_packaged_app_artifact_row() {
    let text = "| App artifact | /tmp/DropSquash.dmg |\n";

    assert_eq!(
        super::fields::field_value(text, "App artifact"),
        Some("/tmp/DropSquash.dmg")
    );
}

#[test]
fn finds_license_cache_path_row() {
    let text = "| License cache path | /tmp/license.json |\n";

    assert_eq!(
        super::fields::field_value(text, "License cache path"),
        Some("/tmp/license.json")
    );
}

#[test]
fn prints_license_helper_commands_for_pending_rows() {
    let groups = grouped(
        &[
            ("Valid sandbox activation".to_string(), String::new()),
            ("Forget license on this Mac".to_string(), String::new()),
        ],
        Some("license"),
    );
    let lines = super::license::extra_lines(
        "| App artifact | /tmp/DropSquash.app |\n| Config path | /tmp/config.json |\n| History path | /tmp/history.jsonl |\n| License cache path | /tmp/license.json |\n",
        &groups,
    );

    assert!(lines.contains(
        &"license cache helper command: cargo run -p xtask -- manual-qa-license-cache '/tmp/license.json'".to_string()
    ));
    assert!(lines.contains(
        &"license seed valid cache command: cargo run -p xtask -- manual-qa-seed-license-cache '/tmp/license.json'".to_string()
    ));
    assert!(lines.contains(
        &"license seed expired cache command: cargo run -p xtask -- manual-qa-seed-license-cache '/tmp/license.json' --expired".to_string()
    ));
    assert!(lines.contains(
        &"license launch app command: cargo run -p xtask -- manual-qa-launch-app '/tmp/DropSquash.app' '/tmp/config.json'".to_string()
    ));
    assert!(lines.contains(
        &"license fresh build command: CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build".to_string()
    ));
    assert!(lines.contains(
        &"license fresh app artifact: /tmp/dsq-build-target/release/bundle/macos/DropSquash.app"
            .to_string()
    ));
    assert!(lines.contains(
        &"license network failure launch command: cargo run -p xtask -- manual-qa-launch-app --license-api-base-url 'http://127.0.0.1:9/v1/licenses' '/tmp/DropSquash.app' '/tmp/config.json'".to_string()
    ));
    assert!(lines.contains(
        &"license fresh app launch command: cargo run -p xtask -- manual-qa-launch-app '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/config.json'".to_string()
    ));
    assert!(lines.contains(
        &"license fresh app network failure launch command: cargo run -p xtask -- manual-qa-launch-app --license-api-base-url 'http://127.0.0.1:9/v1/licenses' '/tmp/dsq-build-target/release/bundle/macos/DropSquash.app' '/tmp/config.json'".to_string()
    ));
    assert!(lines.contains(
        &"license forget helper command: cargo run -p xtask -- manual-qa-forget-license '/tmp/license.json' '/tmp/history.jsonl'".to_string()
    ));
    assert!(lines.contains(
        &"license diagnostics command: cargo run -p dropsquash -- license status --history '/tmp/history.jsonl' --cache-path '/tmp/license.json'".to_string()
    ));
    assert!(lines.contains(
        &"license forget command: cargo run -p dropsquash -- license forget --cache-path '/tmp/license.json'".to_string()
    ));
    assert!(lines.contains(
        &"license diagnostics after UI action: cargo run -p dropsquash -- license status --history '/tmp/history.jsonl' --cache-path '/tmp/license.json'"
            .to_string()
    ));
    assert!(lines.contains(
        &"license activation before-state command: cargo run -p dropsquash -- license status --history '/tmp/history.jsonl' --cache-path '/tmp/license.json'".to_string()
    ));
    assert!(lines
        .iter()
        .any(|line| line.contains("license invalid activation row candidate")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license invalid activation markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license empty activation row candidate")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license empty activation markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license product setup markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license sandbox purchase markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license valid activation row candidate")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license valid activation markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license network failure markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license expired refresh markdown row")));
    assert!(lines.contains(
        &"license cache removed check: test ! -e \"/tmp/license.json\" && echo removed || echo still-present".to_string()
    ));
    assert!(lines.contains(
        &"license forget before-state command: cargo run -p dropsquash -- license status --history '/tmp/history.jsonl' --cache-path '/tmp/license.json'".to_string()
    ));
    assert!(lines.contains(
        &"license forget after-state command: cargo run -p dropsquash -- license status --history '/tmp/history.jsonl' --cache-path '/tmp/license.json'".to_string()
    ));
    assert!(lines
        .iter()
        .any(|line| line.contains("license forget markdown row helper")));
}

#[test]
fn warns_when_license_launch_uses_dmg_artifact() {
    let groups = grouped(
        &[("Valid sandbox activation".to_string(), String::new())],
        Some("license"),
    );
    let lines = super::license::extra_lines(
        "| App artifact | /tmp/DropSquash.dmg |\n| Config path | /tmp/config.json |\n| History path | /tmp/history.jsonl |\n| License cache path | /tmp/license.json |\n",
        &groups,
    );

    assert!(lines
        .iter()
        .any(|line| line.contains("license launch note: App artifact is a DMG")));
}

#[test]
fn finds_output_and_history_rows() {
    let text =
        "| Config path | /tmp/config.json |\n| Output folder | /tmp/output |\n| History path | /tmp/history.jsonl |\n";

    assert_eq!(
        super::fields::field_value(text, "Config path"),
        Some("/tmp/config.json")
    );
    assert_eq!(
        super::fields::field_value(text, "Output folder"),
        Some("/tmp/output")
    );
    assert_eq!(
        super::fields::field_value(text, "History path"),
        Some("/tmp/history.jsonl")
    );
}

#[test]
fn prints_distribution_helper_commands_for_pending_rows() {
    let groups = grouped(
        &[("Codesign verification".to_string(), String::new())],
        Some("distribution"),
    );
    let lines = super::distribution::extra_lines(
        "| App artifact | /tmp/DropSquash.dmg |\n| Output folder | /tmp/out |\n",
        &groups,
    );

    assert!(lines.contains(
        &"distribution fresh build command: CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build".to_string()
    ));
    assert!(lines.contains(
        &"distribution fresh normalize command: cargo run -p xtask -- normalize-dmg /tmp/dsq-build-target/release/bundle/dmg".to_string()
    ));
    assert!(lines.contains(
        &"distribution fresh dmg artifact: /tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg"
            .to_string()
    ));
    assert!(lines.contains(
        &"distribution fresh artifact check: cargo run -p xtask -- artifact-check '/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg'".to_string()
    ));
    assert!(lines.contains(
        &"distribution fresh signing plan: cargo run -p xtask -- macos-signing-plan '/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg' '/tmp/dropsquash-signed-release'".to_string()
    ));
    assert!(lines.contains(
        &"distribution fresh checksum command: cargo run -p xtask -- checksum '/tmp/dropsquash-signed-release/DropSquash.dmg' --output '/tmp/dropsquash-signed-release/SHA256SUMS'".to_string()
    ));
    assert!(lines.contains(
        &"distribution artifact check: cargo run -p xtask -- artifact-check '/tmp/DropSquash.dmg'"
            .to_string()
    ));
    assert!(lines.contains(
        &"distribution signing plan: cargo run -p xtask -- macos-signing-plan '/tmp/DropSquash.dmg' '/tmp/dropsquash-signed-release'".to_string()
    ));
    assert!(lines.contains(
        &"distribution codesign verify plan: cargo run -p xtask -- macos-codesign-verify-plan '/tmp/DropSquash.dmg'".to_string()
    ));
    assert!(lines.contains(
        &"distribution stapler plan: cargo run -p xtask -- macos-stapler-plan '/tmp/DropSquash.dmg'".to_string()
    ));
    assert!(lines.contains(
        &"distribution spctl plan: cargo run -p xtask -- macos-spctl-plan '/tmp/DropSquash.dmg'"
            .to_string()
    ));
    assert!(lines.contains(
        &"distribution checksum command: cargo run -p xtask -- checksum '/tmp/DropSquash.dmg' --output '/tmp/out/SHA256SUMS'".to_string()
    ));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution homebrew release notes path")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution Gatekeeper note")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution codesign row candidate")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution codesign markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution notarization row candidate")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution notarization markdown row")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution Gatekeeper row candidate")));
    assert!(lines
        .iter()
        .any(|line| line.contains("distribution Gatekeeper markdown row")));
}

#[test]
fn warns_when_distribution_artifact_is_local_unsigned_dmg() {
    let groups = grouped(
        &[("Codesign verification".to_string(), String::new())],
        Some("distribution"),
    );
    let lines = super::distribution::extra_lines(
        "| App artifact | /tmp/work/target/release/bundle/dmg/DropSquash.dmg |\n| Output folder | /tmp/out |\n",
        &groups,
    );

    assert!(lines.iter().any(|line| line
        .contains("distribution artifact note: App artifact is the local unsigned QA DMG")));
}

#[test]
fn quotes_packaged_app_open_command_path() {
    let lines =
        super::packaged_app::extra_lines("| App artifact | /tmp/QA Path's/DropSquash.dmg |\n");

    assert!(lines.iter().any(
        |line| line == "packaged-app open command: open -- '/tmp/QA Path'\\''s/DropSquash.dmg'"
    ));
}

#[test]
fn prints_packaged_app_quickstart_with_samples() {
    let dir = tempfile::tempdir().unwrap();
    let csv = dir.path().join("results.csv");
    std::fs::write(
        &csv,
        "backend,input,output,original_bytes,output_bytes,saved_percent,duration_seconds,speed_ratio\n\
apple-native,/tmp/short.mov,/tmp/short.squashed.mp4,10,5,50,1.0,1.0\n\
apple-native,/tmp/medium.mov,/tmp/medium.squashed.mp4,20,10,50,2.0,1.0\n\
apple-native,/tmp/large.mp4,/tmp/large.squashed.mp4,30,15,50,3.0,1.0\n",
    )
    .unwrap();
    let text = format!(
        "| App artifact | /tmp/DropSquash.dmg |\n\
| Config path | /tmp/state/Library/Application Support/DropSquash/config.json |\n\
| Benchmark sample set | Passes | three short, medium, and large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {} |\n",
        csv.display()
    );
    let lines = super::packaged_app::extra_lines(&text);

    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app quickstart 1")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app quickstart 2")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app quickstart 3")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app quickstart 4")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-installed-app stash")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-launch-app --mount-dmg --open-panel")));
}

#[test]
fn prints_mounted_dmg_launch_helpers() {
    let lines = super::packaged_app::extra_lines(
        "| App artifact | /tmp/DropSquash.dmg |\n\
| Config path | /tmp/state/Library/Application Support/DropSquash/config.json |\n",
    );

    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app mounted dmg helper command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app mounted dmg + panel helper command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app mounted dmg drag event helper command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app mounted dmg screenshot command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("packaged-app mounted dmg screenshot note")));
}

#[test]
fn reports_phase_for_packaged_app_rows() {
    assert_eq!(
        phases::for_label("Choose recording conversion"),
        Some("Small Sample")
    );
}

#[test]
fn reports_phase_counts_for_packaged_rows() {
    let counts = phases::counts(&[
        (
            "Choose recording conversion".to_string(),
            "small".to_string(),
        ),
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
        (
            "Notarization staple verification".to_string(),
            String::new(),
        ),
        ("Gatekeeper open test".to_string(), String::new()),
    ]);

    assert_eq!(
        counts,
        vec![("Signature Verification", 2), ("Gatekeeper", 1)]
    );
}

#[test]
fn prints_packaged_visibility_note_for_drag_section() {
    let lines = super::packaged_app::extra_lines(
        "| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |\n",
    );

    assert!(lines.iter().any(|line| {
        line.contains("packaged-app visibility note:")
            && line.contains("full license field")
            && line.contains("Choose recording action")
    }));
}
