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

    assert!(text.contains("| App build | DropSquash 0.1.0 git abc1234 |"));
    assert!(text.contains("artifact-check passed"));
    assert!(text.contains("SHA-256"));
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
