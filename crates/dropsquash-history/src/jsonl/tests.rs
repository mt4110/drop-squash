use std::path::PathBuf;

use dropsquash_core::{EncodeResult, Profile};
use tempfile::tempdir;

use super::*;

fn result(success: bool) -> EncodeResult {
    EncodeResult {
        input_path: PathBuf::from("input.mov"),
        output_path: PathBuf::from("output.mp4"),
        profile: Profile::Auto,
        original_bytes: 100,
        output_bytes: 20,
        success,
        error_message: None,
    }
}

#[tokio::test]
async fn writes_and_reads_jsonl_records() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("history.jsonl");

    append_record(&path, &ConversionRecord::new(result(true)))
        .await
        .unwrap();
    append_record(&path, &ConversionRecord::new(result(false)))
        .await
        .unwrap();

    let records = read_records(&path).await.unwrap();
    assert_eq!(records.len(), 2);
    assert!(records[0].result.success);
    assert!(!records[1].result.success);
}

#[tokio::test]
async fn missing_history_file_reads_as_empty() {
    let dir = tempdir().expect("temp dir");
    let records = read_records(&dir.path().join("missing.jsonl"))
        .await
        .unwrap();
    assert!(records.is_empty());
}

#[tokio::test]
async fn successful_record_append_rejects_failed_or_larger_outputs() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("history.jsonl");

    assert!(append_successful_record(&path, result(false))
        .await
        .is_err());
    let mut larger = result(true);
    larger.output_bytes = 120;
    assert!(append_successful_record(&path, larger).await.is_err());
    assert!(read_records(&path).await.unwrap().is_empty());
}

#[tokio::test]
async fn successful_record_append_persists_successful_smaller_output() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("history.jsonl");

    append_successful_record(&path, result(true)).await.unwrap();

    let records = read_records(&path).await.unwrap();
    assert_eq!(records.len(), 1);
    assert!(records[0].result.is_successful_conversion());
}
