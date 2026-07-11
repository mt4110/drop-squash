use std::io::Write;

use super::checksum_line;

#[test]
fn checksum_line_uses_sha256sum_format() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("artifact.txt");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"dropsquash")
        .unwrap();

    let line = checksum_line(&path).unwrap();

    assert!(!line.ends_with("  "));
    assert!(line.contains("  "));
    assert!(line.ends_with("artifact.txt"));
    assert!(line.starts_with("bd6403ba9c2b"));
}

#[test]
fn directories_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let error = checksum_line(directory.path()).unwrap_err();

    assert!(error.contains("not a file"));
}

#[test]
fn empty_files_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("empty.dmg");
    std::fs::File::create(&path).unwrap();

    let error = checksum_line(&path).unwrap_err();

    assert!(error.contains("empty"));
}
