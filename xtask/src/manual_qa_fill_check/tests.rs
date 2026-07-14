use super::{fill_row, parse_args, USAGE};

#[test]
fn fills_manual_qa_check_row() {
    let filled =
        fill_row("| `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded |  |\n")
            .unwrap();

    assert!(filled.contains("manual-qa-check passed"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}
