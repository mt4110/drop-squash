use std::io::Write;

use super::{command, Request};

#[test]
fn prints_gh_release_create_command() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");

    let text = command(&request).unwrap();

    assert!(text.starts_with("gh release create v1.2.3 "));
    assert!(text.contains("DropSquash.dmg"));
    assert!(text.contains("SHA256SUMS"));
    assert!(text.contains("--title v1.2.3"));
    assert!(text.contains("--notes-file"));
}

#[test]
fn rejects_non_release_tag() {
    let directory = tempfile::tempdir().unwrap();
    let error = command(&request(directory.path(), "1.2.3")).unwrap_err();

    assert!(error.contains("v1.2.3"));
}

#[test]
fn rejects_malformed_release_tag() {
    let directory = tempfile::tempdir().unwrap();
    let error = command(&request(directory.path(), "v1.two.3")).unwrap_err();

    assert!(error.contains("v1.2.3"));
}

#[test]
fn rejects_wrong_checksum_name() {
    let directory = tempfile::tempdir().unwrap();
    let mut request = request(directory.path(), "v1.2.3");
    request.checksum = directory.path().join("checksums.txt");
    write_file(&request.checksum, "abc  DropSquash.dmg");

    let error = command(&request).unwrap_err();

    assert!(error.contains("SHA256SUMS"));
}

#[test]
fn rejects_checksum_without_dmg_line() {
    let directory = tempfile::tempdir().unwrap();
    let request = request(directory.path(), "v1.2.3");
    write_file(&request.checksum, "abc  Other.dmg");

    let error = command(&request).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_missing_notes_file() {
    let directory = tempfile::tempdir().unwrap();
    let mut request = request(directory.path(), "v1.2.3");
    request.notes = directory.path().join("missing.md");

    let error = command(&request).unwrap_err();

    assert!(error.contains(".md"));
}

fn request(directory: &std::path::Path, tag: &str) -> Request {
    let dmg = directory.join("DropSquash.dmg");
    let checksum = directory.join("SHA256SUMS");
    let notes = directory.join("release-notes.md");
    write_dmg(&dmg);
    write_file(&checksum, "0123456789abcdef  DropSquash.dmg");
    write_file(&notes, "# Release\n");
    Request {
        tag: tag.to_string(),
        dmg,
        checksum,
        notes,
    }
}

fn write_dmg(path: &std::path::Path) {
    let mut bytes = b"signed".to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    write_file(path, &String::from_utf8_lossy(&bytes));
}

fn write_file(path: &std::path::Path, text: &str) {
    std::fs::File::create(path)
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
}
