use std::time::Duration;

use super::{csv, BenchmarkRow};

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
