use super::require_notes_sha_matches_manual_qa;

#[test]
fn accepts_matching_release_notes_sha() {
    let (_directory, notes, manual_qa) = fixture();

    require_notes_sha_matches_manual_qa(&notes, &manual_qa).unwrap();
}

#[test]
fn rejects_mismatched_release_notes_sha() {
    let (_directory, _notes, manual_qa) = fixture();
    let notes = "- SHA-256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n";

    let error = require_notes_sha_matches_manual_qa(notes, &manual_qa).unwrap_err();

    assert!(error.contains("manual QA App artifact"));
}

#[test]
fn rejects_missing_release_notes_sha() {
    let (_directory, _notes, manual_qa) = fixture();

    let error = require_notes_sha_matches_manual_qa("", &manual_qa).unwrap_err();

    assert!(error.contains("SHA-256 must be present"));
}

#[test]
fn rejects_missing_manual_qa_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let manual_qa = directory.path().join("manual-qa.md");
    std::fs::write(&manual_qa, "| App build | DropSquash 0.1.0 git abc1234 |\n").unwrap();

    let error = require_notes_sha_matches_manual_qa("- SHA-256: abc\n", &manual_qa).unwrap_err();

    assert!(error.contains("manual QA App artifact must be present"));
}

#[test]
fn rejects_non_udif_manual_qa_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, b"dmg").unwrap();
    let manual_qa = directory.path().join("manual-qa.md");
    std::fs::write(
        &manual_qa,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let error = require_notes_sha_matches_manual_qa("- SHA-256: abc\n", &manual_qa).unwrap_err();

    assert!(error.contains("publish manual QA artifact"));
}

fn fixture() -> (tempfile::TempDir, String, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let bytes = dmg_bytes(b"dropsquash");
    std::fs::write(&artifact, &bytes).unwrap();
    let manual_qa = directory.path().join("manual-qa.md");
    std::fs::write(
        &manual_qa,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();
    let notes = format!("- SHA-256: {}\n", sha256_hex(&bytes));
    (directory, notes, manual_qa)
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
