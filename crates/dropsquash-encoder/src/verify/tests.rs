use std::io::Write;

use super::verify_output;

#[test]
fn accepts_smaller_mp4_with_file_type_box() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("input.mov");
    let output = directory.path().join("output.mp4");
    std::fs::write(&original, vec![0u8; 128]).unwrap();
    std::fs::write(&output, fixture_mp4(12)).unwrap();

    let verification = verify_output(&original, &output).unwrap();

    assert!(verification.output_exists);
    assert!(verification.output_extension_is_mp4);
    assert!(verification.has_mp4_file_type);
    assert!(verification.has_nonzero_duration);
    assert!(verification.duration_matches_source);
    assert!(verification.is_smaller_than_original);
    assert!(verification.is_valid_output);
    assert!(verification.failure_summary().is_empty());
}

#[test]
fn rejects_smaller_non_mp4_payload() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("input.mov");
    let output = directory.path().join("output.mp4");
    std::fs::write(&original, vec![0u8; 128]).unwrap();
    std::fs::write(&output, b"not really media").unwrap();

    let verification = verify_output(&original, &output).unwrap();

    assert!(verification.is_smaller_than_original);
    assert!(!verification.has_mp4_file_type);
    assert!(!verification.is_valid_output);
    assert!(verification
        .failure_summary()
        .contains("mp4 file-type box missing"));
}

#[test]
fn rejects_non_mp4_extension() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("input.mov");
    let output = directory.path().join("output.mov");
    std::fs::write(&original, vec![0u8; 128]).unwrap();
    std::fs::File::create(&output)
        .unwrap()
        .write_all(&fixture_mp4(12))
        .unwrap();

    let verification = verify_output(&original, &output).unwrap();

    assert!(!verification.output_extension_is_mp4);
    assert!(verification.has_mp4_file_type);
    assert!(verification.has_nonzero_duration);
    assert!(!verification.is_valid_output);
    assert!(verification
        .failure_summary()
        .contains("output extension is not mp4"));
}

#[test]
fn rejects_zero_duration_mp4() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("input.mov");
    let output = directory.path().join("output.mp4");
    std::fs::write(&original, vec![0u8; 128]).unwrap();
    std::fs::write(&output, fixture_mp4(0)).unwrap();

    let verification = verify_output(&original, &output).unwrap();

    assert!(verification.has_mp4_file_type);
    assert!(!verification.has_nonzero_duration);
    assert!(!verification.is_valid_output);
    assert!(verification
        .failure_summary()
        .contains("duration is zero or unreadable"));
}

#[test]
fn rejects_output_with_mismatched_duration() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("input.mov");
    let output = directory.path().join("output.mp4");
    let mut original_bytes = fixture_mp4(6_000);
    original_bytes.extend_from_slice(&[0; 128]);
    std::fs::write(&original, original_bytes).unwrap();
    std::fs::write(&output, fixture_mp4(12)).unwrap();

    let verification = verify_output(&original, &output).unwrap();

    assert!(verification.has_nonzero_duration);
    assert!(!verification.duration_matches_source);
    assert!(!verification.is_valid_output);
    assert!(verification
        .failure_summary()
        .contains("duration differs from source"));
}

fn fixture_mp4(duration: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\0\0\0\x14ftypmp42\0\0\0\0mp42");
    bytes.extend_from_slice(b"\0\0\0\x24moov");
    bytes.extend_from_slice(b"\0\0\0\x1cmvhd");
    bytes.extend_from_slice(&[0, 0, 0, 0]);
    bytes.extend_from_slice(&[0; 8]);
    bytes.extend_from_slice(&600u32.to_be_bytes());
    bytes.extend_from_slice(&duration.to_be_bytes());
    bytes
}
