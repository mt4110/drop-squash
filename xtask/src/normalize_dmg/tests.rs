use super::normalize;

#[test]
fn renames_single_tauri_dmg_to_canonical_name() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("DropSquash_0.1.0_aarch64.dmg");
    std::fs::write(&source, b"dmg").unwrap();

    let target = normalize(directory.path()).unwrap();

    assert_eq!(target, directory.path().join("DropSquash.dmg"));
    assert!(target.exists());
    assert!(!source.exists());
}

#[test]
fn accepts_already_canonical_dmg() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("DropSquash.dmg");
    std::fs::write(&target, b"dmg").unwrap();

    assert_eq!(normalize(directory.path()).unwrap(), target);
}

#[test]
fn rejects_empty_directory() {
    let directory = tempfile::tempdir().unwrap();

    let error = normalize(directory.path()).unwrap_err();

    assert!(error.contains("no DMG artifacts"));
}

#[test]
fn rejects_multiple_dmgs() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("DropSquash.dmg"), b"dmg").unwrap();
    std::fs::write(
        directory.path().join("DropSquash_0.1.0_aarch64.dmg"),
        b"dmg",
    )
    .unwrap();

    let error = normalize(directory.path()).unwrap_err();

    assert!(error.contains("expected exactly one DMG artifact"));
}

#[test]
fn rejects_unexpected_dmg_name() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Other.dmg"), b"dmg").unwrap();

    let error = normalize(directory.path()).unwrap_err();

    assert!(error.contains("DropSquash build artifact"));
}
