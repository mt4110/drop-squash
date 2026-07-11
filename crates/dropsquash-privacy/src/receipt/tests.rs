use std::path::PathBuf;

use dropsquash_core::{EncodeResult, Profile};

use super::{receipt_path_for, PrivacyReceipt};
use crate::MetadataPolicy;

#[test]
fn receipt_records_local_only_without_metadata_overclaim() {
    let result = encode_result("/private/source/input.mov", "/tmp/out/output.mp4");

    let receipt = PrivacyReceipt::from(&result);

    assert_eq!(receipt.input_name, "input.mov");
    assert_eq!(receipt.output_name, "output.mp4");
    assert_eq!(receipt.uploaded_bytes, 0);
    assert_eq!(receipt.metadata_policy, MetadataPolicy::Preserve);
}

#[test]
fn receipt_json_omits_absolute_paths() {
    let result = encode_result("/Users/me/Secret/input.mov", "/tmp/out/output.mp4");
    let receipt = PrivacyReceipt::from(&result);
    let json = serde_json::to_string(&receipt).unwrap();

    assert!(json.contains("\"input_name\":\"input.mov\""));
    assert!(json.contains("\"output_name\":\"output.mp4\""));
    assert!(!json.contains("/Users/me/Secret"));
    assert!(!json.contains("/tmp/out"));
}

#[test]
fn receipt_path_replaces_output_extension() {
    assert_eq!(
        receipt_path_for(&PathBuf::from("recording.squashed.mp4")),
        PathBuf::from("recording.squashed.privacy.json")
    );
}

#[test]
fn save_for_result_writes_pretty_json() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("recording.squashed.mp4");
    let result = encode_result("input.mov", output_path);

    let receipt_path = PrivacyReceipt::save_for_result(&result).unwrap();
    let json = std::fs::read_to_string(receipt_path).unwrap();

    assert!(json.contains("\"uploaded_bytes\": 0"));
    assert!(json.contains("\"metadata_policy\": \"preserve\""));
}

fn encode_result(input: impl Into<PathBuf>, output: impl Into<PathBuf>) -> EncodeResult {
    EncodeResult {
        input_path: input.into(),
        output_path: output.into(),
        profile: Profile::Auto,
        original_bytes: 100,
        output_bytes: 40,
        success: true,
        error_message: None,
    }
}
