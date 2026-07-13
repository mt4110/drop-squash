use std::time::Duration;

use super::require_timing_evidence;
use crate::benchmark::report::BenchmarkRow;

#[test]
fn accepts_rows_with_duration_and_speed_ratio() {
    require_timing_evidence(&[row(Some(Duration::from_secs(8)), Duration::from_secs(2))]).unwrap();
}

#[test]
fn rejects_missing_duration_evidence() {
    let error = require_timing_evidence(&[row(None, Duration::from_secs(2))]).unwrap_err();

    assert!(error.contains("duration and speed ratio"));
}

#[test]
fn rejects_missing_speed_ratio_evidence() {
    let error =
        require_timing_evidence(&[row(Some(Duration::from_secs(8)), Duration::ZERO)]).unwrap_err();

    assert!(error.contains("duration and speed ratio"));
}

fn row(duration: Option<Duration>, elapsed: Duration) -> BenchmarkRow {
    BenchmarkRow {
        backend: "apple-native".to_string(),
        input: "short.mov".to_string(),
        output: "short.squashed.mp4".to_string(),
        original_bytes: 1_048_576,
        output_bytes: 524_288,
        elapsed,
        duration,
    }
}
