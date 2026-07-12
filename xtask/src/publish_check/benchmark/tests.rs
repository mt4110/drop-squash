use super::require_notes_csv_matches_manual_qa;

#[test]
fn accepts_matching_benchmark_csv_paths() {
    let notes = notes_with_csv("/tmp/dropsquash-bench/results.csv");
    let (_directory, manual) = manual_qa_with_csv("/tmp/dropsquash-bench/results.csv");

    assert!(require_notes_csv_matches_manual_qa(&notes, &manual).is_ok());
}

#[test]
fn rejects_mismatched_benchmark_csv_paths() {
    let notes = notes_with_csv("/tmp/dropsquash-bench/results.csv");
    let (_directory, manual) = manual_qa_with_csv("/tmp/other-bench/results.csv");

    let error = require_notes_csv_matches_manual_qa(&notes, &manual).unwrap_err();

    assert!(error.contains("must match manual QA"));
}

#[test]
fn rejects_missing_release_notes_csv_path() {
    let notes = "- Benchmark sample set: short medium large smaller outputs on MacBook macOS\n";
    let (_directory, manual) = manual_qa_with_csv("/tmp/dropsquash-bench/results.csv");

    let error = require_notes_csv_matches_manual_qa(notes, &manual).unwrap_err();

    assert!(error.contains("release notes Benchmark sample set"));
}

#[test]
fn rejects_missing_manual_qa_csv_path() {
    let notes = notes_with_csv("/tmp/dropsquash-bench/results.csv");
    let (_directory, manual) = write_manual_qa(
        "| Benchmark sample set | Passes | short medium large smaller on MacBook macOS |\n",
    );

    let error = require_notes_csv_matches_manual_qa(&notes, &manual).unwrap_err();

    assert!(error.contains("manual QA Benchmark sample set"));
}

#[test]
fn rejects_repo_local_matching_csv_paths() {
    let csv = std::env::current_dir()
        .unwrap()
        .join("target/dropsquash-bench/results.csv");
    let csv = csv.to_str().unwrap();
    let notes = notes_with_csv(csv);
    let (_directory, manual) = manual_qa_with_csv(csv);

    let error = require_notes_csv_matches_manual_qa(&notes, &manual).unwrap_err();

    assert!(error.contains("CSV path must stay outside the repository"));
}

fn notes_with_csv(csv: &str) -> String {
    format!(
        "- Benchmark sample set: short medium large smaller outputs on MacBook macOS with CSV saved outside repo at {csv}\n"
    )
}

fn manual_qa_with_csv(csv: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    write_manual_qa(&format!(
        "| Benchmark sample set | Passes | short medium large smaller outputs on MacBook macOS with CSV saved outside repo at {csv} |\n"
    ))
}

fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}
