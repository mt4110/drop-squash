use super::{fill_release_gate_rows, parse_args, USAGE};

#[test]
fn fills_release_gate_rows() {
    let text = "\
| `cargo run -p xtask -- release-check` | Passes |  |
| `cargo run -p xtask -- file-size-check` | Passes |  |
| `cargo run -p xtask -- media-policy-check` | Passes |  |
| `cargo run -p xtask -- privacy-policy-check` | Passes |  |
| `cargo run -p xtask -- website-check` | Passes |  |
";
    let filled = fill_release_gate_rows(text, &results()).unwrap();

    assert!(filled.contains("release-check passed"));
    assert!(filled.contains("file-size-check passed"));
    assert!(filled.contains("media-policy-check passed"));
    assert!(filled.contains("privacy-policy-check passed"));
    assert!(filled.contains("website-check passed"));
}

#[test]
fn rejects_missing_release_gate_rows() {
    let error = fill_release_gate_rows("", &results()).unwrap_err();

    assert!(error.contains("manual QA file is missing release gate rows"));
    assert!(error.contains("release-check"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
    assert_eq!(parse_args(vec!["--help".into()]).unwrap_err(), USAGE);
}

fn results() -> [(&'static str, &'static str); 5] {
    [
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
