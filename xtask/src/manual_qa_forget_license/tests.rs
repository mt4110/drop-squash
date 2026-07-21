use super::run;

#[test]
fn removes_license_cache_and_prints_check() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    std::fs::write(&path, "{}").unwrap();

    run(vec![path.display().to_string()]).unwrap();

    assert!(!path.exists());
}

#[test]
fn accepts_history_path() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    let history = directory.path().join("history.jsonl");
    std::fs::write(&path, "{}").unwrap();
    std::fs::write(&history, "").unwrap();

    run(vec![
        path.display().to_string(),
        history.display().to_string(),
    ])
    .unwrap();

    assert!(!path.exists());
}

#[test]
fn formats_row_candidate() {
    assert_eq!(
        super::row_candidate("trial"),
        "Forgetting state disabled action; confirmed license cache removed and observed app returned to trial state"
    );
}

#[test]
fn formats_markdown_row() {
    let row = super::markdown_row("observed app returned to trial state");

    assert!(row.starts_with("| Forget license on this Mac |"));
    assert!(row.ends_with("|"));
}

#[test]
fn rejects_missing_path() {
    assert!(run(Vec::new()).unwrap_err().contains("usage:"));
}
