use std::time::Duration;

use super::{csv, write, BenchmarkRow};

#[test]
fn escapes_csv_path_cells() {
    let text = csv(&[BenchmarkRow {
        input: "My, Recording.mov".to_string(),
        output: "out \"quoted\".mp4".to_string(),
        original_bytes: 1_048_576,
        output_bytes: 524_288,
        elapsed: Duration::from_secs(2),
    }]);

    assert!(text.contains("\"My, Recording.mov\""));
    assert!(text.contains("\"out \"\"quoted\"\".mp4\""));
    assert!(text.contains(",0.500,0.500"));
}

#[test]
fn writes_csv_to_path() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");

    write(&path, &[row()]).unwrap();

    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.starts_with("input,output,original_bytes"));
    assert!(text.contains("a.mov,out.mp4,1048576,524288"));
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
        input: "a.mov".to_string(),
        output: "out.mp4".to_string(),
        original_bytes: 1_048_576,
        output_bytes: 524_288,
        elapsed: Duration::from_secs(2),
    }
}
