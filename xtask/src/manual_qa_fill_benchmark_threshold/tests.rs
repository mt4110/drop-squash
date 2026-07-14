use super::{fill_rows, parse_args, run, USAGE};

#[test]
fn fills_threshold_row() {
    let dir = tempfile::tempdir().unwrap();
    let manual = dir.path().join("manual.md");
    let current = dir.path().join("current.csv");
    let baseline = dir.path().join("baseline.csv");
    std::fs::write(&manual, manual_text()).unwrap();
    std::fs::write(&current, csv_text(&[10.0, 9.0, 7.5])).unwrap();
    std::fs::write(&baseline, csv_text(&[10.0, 10.0, 10.0])).unwrap();

    run(paths(&manual, &current, &baseline)).unwrap();
    let filled = std::fs::read_to_string(&manual).unwrap();

    assert!(filled.contains("1 sample exceeded 20% regression"));
    assert!(filled.contains("same-machine release candidate baseline"));
    assert!(filled.contains("large.mov 25.0%"));
}

#[test]
fn rejects_two_large_regressions() {
    let dir = tempfile::tempdir().unwrap();
    let current = dir.path().join("current.csv");
    let baseline = dir.path().join("baseline.csv");
    std::fs::write(&current, csv_text(&[7.0, 7.5, 10.0])).unwrap();
    std::fs::write(&baseline, csv_text(&[10.0, 10.0, 10.0])).unwrap();

    let error = super::compare::result(&current, &baseline).unwrap_err();

    assert!(error.contains("2 samples exceeded 20% regression"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn rejects_missing_threshold_row() {
    let error = fill_rows("| Benchmark sample set | Passes | ok |\n", "result").unwrap_err();

    assert!(error.contains("benchmark threshold row"));
}

fn paths(
    manual: &std::path::Path,
    current: &std::path::Path,
    baseline: &std::path::Path,
) -> Vec<String> {
    vec![
        manual.display().to_string(),
        current.display().to_string(),
        baseline.display().to_string(),
    ]
}

fn manual_text() -> &'static str {
    "| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples against the same-machine release candidate baseline without a documented reason |  |\n"
}

fn csv_text(values: &[f64; 3]) -> String {
    format!(
        "backend,input,output,original_bytes,output_bytes,duration_s,elapsed_s,compression_ratio,saved_percent,throughput_mib_s,speed_ratio\napple-native,/tmp/short.mov,/tmp/short.mp4,100,50,8.000,2.000,0.500,50.0,{:.3},4.000\napple-native,/tmp/medium.mov,/tmp/medium.mp4,200,100,12.000,3.000,0.500,50.0,{:.3},4.000\napple-native,/tmp/large.mov,/tmp/large.mp4,300,150,16.000,4.000,0.500,50.0,{:.3},4.000\n",
        values[0], values[1], values[2]
    )
}
