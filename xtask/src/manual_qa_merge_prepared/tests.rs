use super::merge;

#[test]
fn merges_filled_rows_and_preserves_text() {
    let target = "\
# Manual QA

| Field | Value |
|---|---|
| App build |  |
| Check | Input | Expected | Result |
|---|---|---|---|
| `cargo run -p xtask -- release-check` | Passes | Passes |  |
";
    let source = "\
| App build | DropSquash 0.1.0 git abc1234 |
| `cargo run -p xtask -- release-check` | Passes | Passes | release-check passed |
";

    let merged = merge(target, source).unwrap();

    assert!(merged.contains("| App build | DropSquash 0.1.0 git abc1234 |"));
    assert!(merged.contains("| `cargo run -p xtask -- release-check` | Passes | Passes | release-check passed |"));
    assert!(merged.starts_with("# Manual QA"));
}

#[test]
fn skips_blank_manual_result_rows() {
    let target = "\
| Check | Input | Expected | Result |
|---|---|---|---|
| Choose recording conversion | Small `.mov` | Smaller output | observed earlier |
";
    let source = "\
| Choose recording conversion | Small `.mov` | Smaller output |  |
";

    let merged = merge(target, source).unwrap_err();

    assert!(merged.contains("no filled rows"));
}

#[test]
fn errors_when_no_labels_match() {
    let target = "| Field | Value |\n|---|---|\n| App build |  |\n";
    let source = "| Tester | masakitakemura |\n";

    let error = merge(target, source).unwrap_err();

    assert!(error.contains("did not match any target rows"));
}
