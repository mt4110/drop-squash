use super::write;
use crate::manual_qa_prepare::{markdown, options::Options};

#[test]
fn writes_fields_and_release_candidate_rows() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("prepared.md");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let fields: Vec<markdown::Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    write(
        &output,
        &fields,
        Some(&artifact),
        &options(directory.path()),
    )
    .unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.starts_with("Prepared manual QA draft only."));
    assert!(text.contains("Benchmark commands:"));
    assert!(text.contains("Benchmark context to record:"));
    assert!(text.contains("cargo run -p xtask -- benchmark --release-set"));
    assert!(text.contains("cargo run -p xtask -- benchmark-csv-check"));
    assert!(text.contains("Packaged app command:"));
    assert!(text.contains("open -- '"));
    assert!(text.contains("DropSquash.dmg'"));
    assert!(text.contains("Checksum command:"));
    assert!(text.contains("cargo run -p xtask -- checksum"));
    assert!(text.contains("SHA256SUMS"));
    assert!(text.contains("| App build | DropSquash 0.1.0 git abc1234 |"));
    assert!(text.contains("| Disk image launch notice |"));
    assert!(text.contains("| Choose recording conversion |"));
    assert!(text.contains("| Batch summary |"));
    assert!(text.contains("| Trash source policy |"));
    assert!(text.contains("| Valid sandbox activation |"));
    assert!(text.contains("| `cargo run -p xtask -- release-check` |"));
    assert!(text.contains("| Codesign verification |"));
    assert!(text.contains("`cargo run -p dropsquash -- license status`"));
    assert!(text.contains("artifact-check passed"));
    assert!(text.contains("SHA-256"));
}

#[test]
fn writes_benchmark_environment_context_when_available() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");
    let fields: Vec<markdown::Field> = vec![
        ("macOS version", "macOS 15.5".into()),
        ("Machine", "MacBook Pro".into()),
    ];

    write(&output, &fields, None, &options(directory.path())).unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.contains("- CSV: "));
    assert!(text.contains("benchmark-results.csv"));
    assert!(text.contains("- macOS: macOS 15.5"));
    assert!(text.contains("- Machine: MacBook Pro"));
    assert!(text.contains("Result skeleton: Backend: apple-native"));
    assert!(text.contains("short.mov duration <seconds>s"));
    assert!(text.contains("CSV saved outside repo:"));
}

#[test]
fn benchmark_context_keeps_placeholders_without_environment_fields() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");

    write(&output, &[], None, &options(directory.path())).unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.contains("- macOS: <record macOS version>"));
    assert!(text.contains("- Machine: <record machine>"));
}

#[test]
fn quotes_packaged_app_command_path() {
    let directory = tempfile::tempdir().unwrap();
    let nested = directory.path().join("QA Path's");
    std::fs::create_dir(&nested).unwrap();
    let artifact = nested.join("DropSquash.dmg");
    let output = directory.path().join("prepared.md");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();

    write(&output, &[], Some(&artifact), &options(directory.path())).unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.contains("open -- '"));
    assert!(text.contains("QA Path'\\''s/DropSquash.dmg'"));
}

#[test]
fn writes_restore_command_for_reset_trial() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");
    let mut options = options(directory.path());
    options.reset_trial = true;
    options.app_state_dir = directory
        .path()
        .join("Library")
        .join("Application Support")
        .join("DropSquash");

    write(&output, &[], None, &options).unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.contains("Trial state restore command:"));
    assert!(text.contains("manual-qa-prepare --restore-state"));
    assert!(text.contains("Application Support/DropSquash'"));
    assert!(text.contains("--state-dir '"));
}

#[test]
fn release_candidate_draft_contains_every_required_check() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("prepared.md");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let fields: Vec<markdown::Field> = vec![("App artifact", artifact.display().to_string())];

    write(
        &output,
        &fields,
        Some(&artifact),
        &options(directory.path()),
    )
    .unwrap();
    let text = std::fs::read_to_string(output).unwrap();
    let labels = manual_qa_labels(&text);

    let missing = crate::manual_qa_check::requirements::REQUIRED_CHECKS
        .into_iter()
        .filter(|check| !labels.iter().any(|label| label == check))
        .collect::<Vec<_>>();
    assert!(missing.is_empty(), "{missing:?}");
}

#[test]
fn written_draft_is_rejected_by_manual_qa_check() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");
    let fields: Vec<markdown::Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    write(&output, &fields, None, &options(directory.path())).unwrap();
    let missing = crate::manual_qa_check::check_file(&output).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("prepared draft markers")));
}

#[test]
fn release_candidate_draft_is_rejected_by_manual_qa_check() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("prepared.md");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let fields: Vec<markdown::Field> = vec![("App artifact", artifact.display().to_string())];

    write(
        &output,
        &fields,
        Some(&artifact),
        &options(directory.path()),
    )
    .unwrap();
    let missing = crate::manual_qa_check::check_file(&output).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("prepared draft markers")));
}

#[test]
fn rejects_existing_output_file() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("prepared.md");
    std::fs::write(&output, "keep this evidence").unwrap();
    let fields: Vec<markdown::Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    let error = write(&output, &fields, None, &options(directory.path())).unwrap_err();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(error.contains("failed to create manual QA Markdown output"));
    assert_eq!(text, "keep this evidence");
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}

fn options(path: &std::path::Path) -> Options {
    Options {
        app_artifact: None,
        app_state_dir: path.join("app-state"),
        input_sample_set: None,
        markdown_output: None,
        output_dir: path.join("qa-output"),
        reset_trial: false,
        restore_state: false,
        state_dir: path.join("state"),
    }
}

fn manual_qa_labels(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('|') && !line.contains("---"))
        .filter_map(|line| line.split('|').nth(1))
        .map(str::trim)
        .filter(|label| !matches!(*label, "Field" | "Check"))
        .map(str::to_string)
        .collect()
}
