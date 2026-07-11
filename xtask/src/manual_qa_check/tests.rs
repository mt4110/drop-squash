use super::check_file;
use super::requirements::{REQUIRED_CHECKS, REQUIRED_FIELDS};

#[test]
fn accepts_complete_manual_qa_tables() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::create_dir(&artifact).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, complete_manual_qa(&artifact)).unwrap();

    assert!(check_file(&path).unwrap().is_empty());
}

#[test]
fn reports_empty_environment_fields() {
    let (_directory, path) = write_manual_qa("| App build |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA field is empty: App build".to_string()));
}

#[test]
fn reports_empty_four_column_results() {
    let (_directory, path) = write_manual_qa("| Convert | sample.mov | Smaller output |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result is empty: Convert".to_string()));
}

#[test]
fn reports_empty_three_column_results() {
    let (_directory, path) = write_manual_qa("| Release gate | Passes |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result is empty: Release gate".to_string()));
}

#[test]
fn reports_missing_required_checks() {
    let (_directory, path) = write_manual_qa("| App build | 0.1.0 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA check is missing: Cancellation".to_string()));
}

#[test]
fn reports_missing_app_artifact_path() {
    let (_directory, path) = write_manual_qa("| App artifact | /missing/DropSquash.app |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("artifact does not exist")));
}

#[test]
fn reports_non_iso_date() {
    let (_directory, path) = write_manual_qa("| Date | 7/11/2026 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("YYYY-MM-DD")));
}

#[test]
fn reports_vague_manual_results() {
    let (_directory, path) =
        write_manual_qa("| Cancellation | large.mov | Returns to ready | Pass |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Cancellation".to_string()));
}

fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}

fn complete_manual_qa(artifact: &std::path::Path) -> String {
    let mut text = String::from("| Field | Value |\n|---|---|\n");
    for field in REQUIRED_FIELDS {
        let value = match field {
            "App artifact" => artifact.display().to_string(),
            "Date" => "2026-07-11".to_string(),
            _ => "Concrete evidence".to_string(),
        };
        text.push_str(&format!("| {field} | {value} |\n"));
    }
    text.push_str("| Check | Expected | Result |\n|---|---|---|\n");
    for check in REQUIRED_CHECKS {
        if check.starts_with('`') {
            text.push_str(&format!("| {check} | Passes | Pass |\n"));
        } else {
            text.push_str(&format!(
                "| {check} | Passes | Observed expected behavior |\n"
            ));
        }
    }
    text
}
