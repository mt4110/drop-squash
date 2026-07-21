use super::{fill_rows, parse_args, summary, USAGE};

#[test]
fn fills_benchmark_rows() {
    let dir = tempfile::tempdir().unwrap();
    let csv = dir.path().join("results.csv");
    std::fs::write(&csv, csv_text()).unwrap();
    let text = "\
| Machine | MacBookPro18,4 arm64 |
| macOS version | macOS 26.5.2 |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | Passes |  |
| Benchmark sample set | Passes |  |
";
    let filled = fill_rows(text, &summary(&csv, text).unwrap()).unwrap();

    assert!(filled.contains("benchmark CSV recorded for three samples"));
    assert!(filled.contains("original local recordings"));
    assert!(filled.contains("backend apple-native"));
    assert!(filled.contains("short.mov 8.000s 50.0% saved"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn ignores_non_table_lines_when_reading_fields() {
    let dir = tempfile::tempdir().unwrap();
    let csv = dir.path().join("results.csv");
    std::fs::write(&csv, csv_text()).unwrap();
    let text = "\
Prepared manual QA draft only.

- CSV: /tmp/results.csv

| Machine | MacBookPro18,4 arm64 |
| macOS version | macOS 26.5.2 |
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | Passes |  |
| Benchmark sample set | Passes |  |
";

    let filled = fill_rows(text, &summary(&csv, text).unwrap()).unwrap();

    assert!(filled.contains("benchmark CSV recorded for three samples"));
}

fn csv_text() -> &'static str {
    "backend,input,output,original_bytes,output_bytes,duration_s,elapsed_s,compression_ratio,saved_percent,throughput_mib_s,speed_ratio\napple-native,/tmp/short.mov,/tmp/short.mp4,100,50,8.000,2.000,0.500,50.0,0.500,4.000\napple-native,/tmp/medium.mov,/tmp/medium.mp4,200,100,12.000,3.000,0.500,50.0,0.750,4.000\napple-native,/tmp/large.mov,/tmp/large.mp4,300,150,16.000,4.000,0.500,50.0,1.000,4.000\n"
}
