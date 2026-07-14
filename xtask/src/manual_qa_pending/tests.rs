use super::{parse_args, pending_rows, section::grouped, USAGE};

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
                "Codesign verification".to_string(),
                "Public DMG/app artifact verifies".to_string(),
            ),
        ],
        None,
    );

    assert_eq!(groups[0].0, "Packaged App");
    assert_eq!(groups[1].0, "License Sandbox");
    assert_eq!(groups[2].0, "Distribution And Signing");
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
