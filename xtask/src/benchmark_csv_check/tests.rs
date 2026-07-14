use std::path::Path;

use super::validate_path;

#[test]
fn accepts_release_set_csv() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");
    std::fs::write(&path, csv(3, 50)).unwrap();

    validate_path(&path).unwrap();
}

#[test]
fn rejects_missing_csv() {
    let error = validate_path(Path::new("/tmp/dropsquash-missing-results.csv")).unwrap_err();

    assert!(error.contains("benchmark CSV does not exist"));
}

#[test]
fn rejects_relative_csv_path() {
    let error = validate_path(Path::new("results.csv")).unwrap_err();

    assert!(error.contains("absolute .csv path outside the repository"));
}

#[test]
fn rejects_csv_inside_repository() {
    let path = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("benchmark-results.csv");

    let error = validate_path(&path).unwrap_err();

    assert!(error.contains("outside the repository"));
}

#[test]
fn rejects_non_csv_path() {
    let error = validate_path(Path::new("/tmp/dropsquash-benchmark.txt")).unwrap_err();

    assert!(error.contains("absolute .csv path outside the repository"));
}

#[test]
fn rejects_short_release_set() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");
    std::fs::write(&path, csv(2, 50)).unwrap();

    let error = validate_path(&path).unwrap_err();

    assert!(error.contains("at least three sample rows"));
}

#[test]
fn rejects_not_smaller_outputs() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");
    std::fs::write(&path, csv(3, 100)).unwrap();

    let error = validate_path(&path).unwrap_err();

    assert!(error.contains("output is not smaller"));
}

#[test]
fn rejects_missing_speed_ratio() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("results.csv");
    std::fs::write(&path, csv(3, 50).replace(",4.000\n", ",\n")).unwrap();

    let error = validate_path(&path).unwrap_err();

    assert!(error.contains("missing speed_ratio"));
}

fn csv(rows: usize, output_percent: u64) -> String {
    let mut text = String::from(
        "backend,input,output,original_bytes,output_bytes,duration_s,elapsed_s,compression_ratio,saved_percent,throughput_mib_s,speed_ratio\n",
    );
    for index in 0..rows {
        text.push_str(&format!(
            "apple-native,sample-{index}.mov,out-{index}.mp4,100,{output_percent},8.000,2.000,0.500,50.0,0.500,4.000\n",
        ));
    }
    text
}
