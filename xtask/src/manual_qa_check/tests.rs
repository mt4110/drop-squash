use super::check_file;

#[test]
fn accepts_complete_manual_qa_tables() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        r#"
| Field | Value |
|---|---|
| App build | 0.1.0 |
| Check | Input | Expected | Result |
|---|---|---|---|
| Convert | sample.mov | Smaller output | Pass |
| Check | Expected | Result |
|---|---|---|
| Release gate | Passes | Pass |
"#,
    )
    .unwrap();

    assert!(check_file(&path).unwrap().is_empty());
}

#[test]
fn reports_empty_environment_fields() {
    let (_directory, path) = write_manual_qa("| App build |  |\n");
    let missing = check_file(&path).unwrap();

    assert_eq!(missing, vec!["manual QA field is empty: App build"]);
}

#[test]
fn reports_empty_four_column_results() {
    let (_directory, path) = write_manual_qa("| Convert | sample.mov | Smaller output |  |\n");
    let missing = check_file(&path).unwrap();

    assert_eq!(missing, vec!["manual QA result is empty: Convert"]);
}

#[test]
fn reports_empty_three_column_results() {
    let (_directory, path) = write_manual_qa("| Release gate | Passes |  |\n");
    let missing = check_file(&path).unwrap();

    assert_eq!(missing, vec!["manual QA result is empty: Release gate"]);
}

fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}
