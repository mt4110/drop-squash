use super::write;
use crate::manual_qa_prepare::markdown;

#[test]
fn writes_fields_and_release_candidate_rows() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("prepared.md");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let fields: Vec<markdown::Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    write(&output, &fields, Some(&artifact)).unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.starts_with("Prepared manual QA draft only."));
    assert!(text.contains("| App build | DropSquash 0.1.0 git abc1234 |"));
    assert!(text.contains("| Disk image launch notice |"));
    assert!(text.contains("artifact-check passed"));
    assert!(text.contains("SHA-256"));
}

#[test]
fn written_draft_is_rejected_by_manual_qa_check() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");
    let fields: Vec<markdown::Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    write(&output, &fields, None).unwrap();
    let missing = crate::manual_qa_check::check_file(&output).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("prepared draft markers")));
}

#[test]
fn release_candidate_draft_is_rejected_by_manual_qa_check() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("prepared.md");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let fields: Vec<markdown::Field> = vec![("App artifact", artifact.display().to_string())];

    write(&output, &fields, Some(&artifact)).unwrap();
    let missing = crate::manual_qa_check::check_file(&output).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("prepared draft markers")));
}

#[test]
fn rejects_existing_output_file() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");
    std::fs::write(&output, "keep this evidence").unwrap();
    let fields: Vec<markdown::Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    let error = write(&output, &fields, None).unwrap_err();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(error.contains("failed to create manual QA Markdown output"));
    assert_eq!(text, "keep this evidence");
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
