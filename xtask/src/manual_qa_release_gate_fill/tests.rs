use super::{fill_release_gate_rows, parse_args, uses_main_manual_qa, USAGE};

#[test]
fn fills_release_gate_rows() {
    let text = "\
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- media-policy-check` | Passes |  |
| `cargo run -p xtask -- privacy-policy-check` | Passes |  |
| `cargo run -p xtask -- website-check` | Passes |  |
";
    let rows = fixture_rows();
    let filled = fill_release_gate_rows(text, &rows).unwrap();

    assert!(filled.contains("release-check passed"));
    assert!(filled.contains("file-size-check passed"));
    assert!(filled.contains("media-policy-check passed"));
    assert!(filled.contains("privacy-policy-check passed"));
    assert!(filled.contains("website-check passed"));
}

#[test]
fn rejects_missing_release_gate_rows() {
    let rows = fixture_rows();
    let error = fill_release_gate_rows("", &rows).unwrap_err();

    assert!(error.contains("manual QA file is missing release gate rows"));
    assert!(error.contains("release-check"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
    assert_eq!(parse_args(vec!["--help".into()]).unwrap_err(), USAGE);
}

#[test]
fn prepared_draft_skips_release_check_row() {
    let path = std::path::PathBuf::from("/tmp/dropsquash-manual-qa-prepared.md");

    assert!(!uses_main_manual_qa(&path));
    let rows = fixture_rows()
        .into_iter()
        .filter(|(label, _)| *label != "`cargo run -p xtask -- release-check`")
        .collect::<Vec<_>>();
    assert_eq!(rows[0].0, "`cargo run -p xtask -- file-size-check`");
}

#[test]
fn main_manual_qa_keeps_release_check_row() {
    let path = std::path::PathBuf::from("docs/manual-qa.md");

    assert!(uses_main_manual_qa(&path));
}

fn fixture_rows() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "`cargo run -p xtask -- release-check`",
            "release-check passed",
        ),
        (
            "`cargo run -p xtask -- file-size-check`",
            "file-size-check passed",
        ),
        (
            "`cargo run -p xtask -- media-policy-check`",
            "media-policy-check passed",
        ),
        (
            "`cargo run -p xtask -- privacy-policy-check`",
            "privacy-policy-check passed",
        ),
        (
            "`cargo run -p xtask -- website-check`",
            "website-check passed",
        ),
    ]
}
