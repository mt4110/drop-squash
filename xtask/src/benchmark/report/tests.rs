use std::time::Duration;

use super::{csv, write, BenchmarkRow};

#[test]
fn escapes_csv_path_cells() {
    let text = csv::from_rows(&[BenchmarkRow {
        backend: "apple-native".to_string(),
        input: "My, Recording.mov".to_string(),
        output: "out \"quoted\".mp4".to_string(),
        original_bytes: 1_048_576,
        output_bytes: 524_288,
        elapsed: Duration::from_secs(2),
        duration: Some(Duration::from_secs(8)),
    }]);

    assert!(text.contains("\"My, Recording.mov\""));
    assert!(text.contains("\"out \"\"quoted\"\".mp4\""));
    assert!(text.contains(",8.000,2.000000,0.500,50.000000,0.500,4.000"));
}

#[test]
fn writes_csv_to_path() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");

    write(&path, &[row()]).unwrap();

    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.starts_with("backend,input,output,original_bytes"));
    assert!(text.contains("apple-native,a.mov,out.mp4,1048576,524288"));
}

#[test]
fn leaves_duration_and_speed_blank_when_probe_has_no_duration() {
    let text = csv::from_rows(&[BenchmarkRow {
        duration: None,
        ..row()
    }]);

    assert!(text.contains(",524288,,2.000000,0.500,50.000000,0.500,\n"));
}

#[test]
fn keeps_near_equal_smaller_outputs_below_one_ratio_and_above_zero_saved_percent() {
    let text = csv::from_rows(&[BenchmarkRow {
        backend: "apple-native".to_string(),
        input: "tiny.mov".to_string(),
        output: "tiny.mp4".to_string(),
        original_bytes: 1_177_311,
        output_bytes: 1_177_000,
        elapsed: Duration::from_secs_f64(0.055),
        duration: Some(Duration::from_secs_f64(8.012)),
    }]);

    assert!(!text.contains(",1.000000,"));
    assert!(!text.contains(",0.000000,0.500,"));
}

#[test]
fn refuses_to_overwrite_csv_path() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");
    std::fs::write(&path, "existing").unwrap();

    let error = write(&path, &[row()]).unwrap_err();

    assert!(error.contains("failed to write benchmark CSV"));
}

fn row() -> BenchmarkRow {
    BenchmarkRow {
        backend: "apple-native".to_string(),
        input: "a.mov".to_string(),
        output: "out.mp4".to_string(),
        original_bytes: 1_048_576,
        output_bytes: 524_288,
        elapsed: Duration::from_secs(2),
        duration: Some(Duration::from_secs(8)),
    }
}
