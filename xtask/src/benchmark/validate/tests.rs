use dropsquash_core::{EncodeResult, Profile};

use super::result;

#[test]
fn accepts_existing_smaller_output() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("out.mp4");
    std::fs::write(&output, [1, 2, 3]).unwrap();

    result(&encode_result(output, 10, 3, true)).unwrap();
}

#[test]
fn rejects_failed_or_larger_output() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("out.mp4");
    std::fs::write(&output, [1, 2, 3]).unwrap();

    assert!(result(&encode_result(output.clone(), 10, 3, false)).is_err());
    assert!(result(&encode_result(output, 10, 12, true)).is_err());
}

#[test]
fn rejects_missing_output() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("missing.mp4");

    assert!(result(&encode_result(output, 10, 3, true)).is_err());
}

#[test]
fn rejects_output_size_that_no_longer_matches_file() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("out.mp4");
    std::fs::write(&output, [1, 2, 3]).unwrap();

    let error = result(&encode_result(output, 10, 2, true)).unwrap_err();

    assert!(error.contains("output size changed"));
}

fn encode_result(
    output_path: std::path::PathBuf,
    original: u64,
    output: u64,
    success: bool,
) -> EncodeResult {
    EncodeResult {
        input_path: std::path::PathBuf::from("input.mov"),
        output_path,
        profile: Profile::Auto,
        original_bytes: original,
        output_bytes: output,
        success,
        error_message: None,
    }
}
