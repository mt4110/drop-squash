use super::write;

#[test]
fn writes_markdown_once() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("release-notes.md");

    write(&path, &["## Artifact".into(), "- Version: v0.1.0".into()]).unwrap();
    let text = std::fs::read_to_string(path).unwrap();

    assert!(text.contains("## Artifact"));
    assert!(text.ends_with('\n'));
}

#[test]
fn rejects_existing_markdown_output() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("release-notes.md");
    std::fs::write(&path, "existing").unwrap();

    let error = write(&path, &["replacement".into()]).unwrap_err();

    assert!(error.contains("failed to create"));
}
