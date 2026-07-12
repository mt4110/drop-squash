use super::{require_current_head, require_public_dmg};

#[test]
fn accepts_public_dmg_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes()).unwrap();
    let (_directory, path) =
        write_manual_qa(&format!("| App artifact | {} |\n", artifact.display()));

    assert!(require_public_dmg(&path).is_ok());
}

#[test]
fn rejects_non_udif_dmg_before_publish() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, b"dmg").unwrap();
    let (_directory, path) =
        write_manual_qa(&format!("| App artifact | {} |\n", artifact.display()));

    assert!(require_public_dmg(&path).is_err());
}

#[test]
fn rejects_relative_app_artifact_before_publish() {
    let (_directory, path) = write_manual_qa("| App artifact | DropSquash.dmg |\n");

    assert!(require_public_dmg(&path)
        .unwrap_err()
        .contains("absolute path"));
}

#[test]
fn rejects_missing_app_artifact_before_publish() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let (_directory, path) =
        write_manual_qa(&format!("| App artifact | {} |\n", artifact.display()));

    assert!(require_public_dmg(&path)
        .unwrap_err()
        .contains("must exist"));
}

#[test]
fn rejects_app_artifact_before_publish() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::write(&artifact, b"app").unwrap();
    let (_directory, path) =
        write_manual_qa(&format!("| App artifact | {} |\n", artifact.display()));

    assert!(require_public_dmg(&path)
        .unwrap_err()
        .contains("public DropSquash.dmg"));
}

#[test]
fn accepts_manual_qa_for_current_head() {
    let head = super::git_head().unwrap();
    let (_directory, path) =
        write_manual_qa(&format!("| App build | DropSquash 0.1.0 git {head} |\n"));

    assert!(require_current_head(&path).is_ok());
}

#[test]
fn rejects_manual_qa_for_old_head() {
    let (_directory, path) = write_manual_qa("| App build | DropSquash 0.1.0 git 0000000 |\n");

    assert!(require_current_head(&path)
        .unwrap_err()
        .contains("current HEAD"));
}

#[test]
fn rejects_manual_qa_for_commit_with_matching_prefix() {
    let head = super::git_head().unwrap();
    let (_directory, path) = write_manual_qa(&format!(
        "| App build | DropSquash 0.1.0 git {head}ffff |\n"
    ));

    assert!(require_current_head(&path)
        .unwrap_err()
        .contains("current HEAD"));
}

fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}

fn dmg_bytes() -> Vec<u8> {
    let mut bytes = b"dropsquash".to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
