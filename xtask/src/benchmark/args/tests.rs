use dropsquash_core::{OutputSize, Profile};

use super::BenchmarkArgs;

#[test]
fn parses_required_inputs_and_defaults() {
    let args = BenchmarkArgs::parse(vec![
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mp4".to_string(),
        "--output-dir".to_string(),
        "bench-out".to_string(),
    ])
    .unwrap();

    assert_eq!(args.inputs.len(), 2);
    assert_eq!(args.output_dir, std::path::PathBuf::from("bench-out"));
    assert_eq!(args.csv_output, None);
    assert_eq!(args.profile, Profile::Auto);
    assert!(!args.release_set);
    assert_eq!(args.output_size, OutputSize::Auto);
}

#[test]
fn parses_profile_and_size() {
    let args = BenchmarkArgs::parse(vec![
        "--input".to_string(),
        "a.mov".to_string(),
        "--output-dir".to_string(),
        "bench-out".to_string(),
        "--profile".to_string(),
        "slack".to_string(),
        "--size".to_string(),
        "720p".to_string(),
    ])
    .unwrap();

    assert_eq!(args.profile, Profile::Slack);
    assert_eq!(args.output_size, OutputSize::P720);
}

#[test]
fn parses_csv_output() {
    let args = BenchmarkArgs::parse(vec![
        "--input".to_string(),
        "a.mov".to_string(),
        "--output-dir".to_string(),
        "bench-out".to_string(),
        "--csv-output".to_string(),
        "bench-out/results.csv".to_string(),
    ])
    .unwrap();

    assert_eq!(
        args.csv_output,
        Some(std::path::PathBuf::from("bench-out/results.csv"))
    );
}

#[test]
fn release_set_requires_three_inputs() {
    let csv = tempfile::tempdir().unwrap().path().join("results.csv");
    let error = BenchmarkArgs::parse(vec![
        "--release-set".to_string(),
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mov".to_string(),
        "--output-dir".to_string(),
        "bench-out".to_string(),
        "--csv-output".to_string(),
        csv.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("at least three"));

    let output_dir = tempfile::tempdir().unwrap();
    let csv_dir = tempfile::tempdir().unwrap();
    let args = BenchmarkArgs::parse(vec![
        "--release-set".to_string(),
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mov".to_string(),
        "--input".to_string(),
        "c.mov".to_string(),
        "--output-dir".to_string(),
        output_dir.path().display().to_string(),
        "--csv-output".to_string(),
        csv_dir.path().join("results.csv").display().to_string(),
    ])
    .unwrap();

    assert!(args.release_set);
}

#[test]
fn release_set_requires_output_outside_repository() {
    let csv = tempfile::tempdir().unwrap().path().join("results.csv");
    let error = BenchmarkArgs::parse(vec![
        "--release-set".to_string(),
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mov".to_string(),
        "--input".to_string(),
        "c.mov".to_string(),
        "--output-dir".to_string(),
        "bench-out".to_string(),
        "--csv-output".to_string(),
        csv.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("outside the repository"));
}

#[test]
fn release_set_requires_csv_output() {
    let output_dir = tempfile::tempdir().unwrap();
    let error = BenchmarkArgs::parse(vec![
        "--release-set".to_string(),
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mov".to_string(),
        "--input".to_string(),
        "c.mov".to_string(),
        "--output-dir".to_string(),
        output_dir.path().display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--csv-output"));
    assert!(error.contains("outside the repository"));
}

#[test]
fn release_set_requires_csv_output_outside_repository() {
    let output_dir = tempfile::tempdir().unwrap();
    let error = BenchmarkArgs::parse(vec![
        "--release-set".to_string(),
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mov".to_string(),
        "--input".to_string(),
        "c.mov".to_string(),
        "--output-dir".to_string(),
        output_dir.path().display().to_string(),
        "--csv-output".to_string(),
        "results.csv".to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("--csv-output"));
    assert!(error.contains("outside the repository"));
}

#[test]
fn release_set_requires_csv_output_extension() {
    let output_dir = tempfile::tempdir().unwrap();
    let csv_dir = tempfile::tempdir().unwrap();
    let error = BenchmarkArgs::parse(vec![
        "--release-set".to_string(),
        "--input".to_string(),
        "a.mov".to_string(),
        "--input".to_string(),
        "b.mov".to_string(),
        "--input".to_string(),
        "c.mov".to_string(),
        "--output-dir".to_string(),
        output_dir.path().display().to_string(),
        "--csv-output".to_string(),
        csv_dir.path().join("results.txt").display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains(".csv"));
    assert!(error.contains("--csv-output"));
}

#[test]
fn rejects_missing_required_values() {
    assert!(BenchmarkArgs::parse(vec![]).is_err());
    assert!(BenchmarkArgs::parse(vec!["--input".to_string(), "a.mov".to_string(),]).is_err());
}
