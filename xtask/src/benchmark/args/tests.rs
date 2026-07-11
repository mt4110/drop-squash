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
    assert_eq!(args.profile, Profile::Auto);
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
fn rejects_missing_required_values() {
    assert!(BenchmarkArgs::parse(vec![]).is_err());
    assert!(BenchmarkArgs::parse(vec!["--input".to_string(), "a.mov".to_string(),]).is_err());
}
