use super::{rows, Field};

#[test]
fn field_type_accepts_manual_qa_rows() {
    let fields: Vec<Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

    assert_eq!(fields[0].0, "App build");
}

#[test]
fn generated_rows_satisfy_manual_qa_field_checks() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("output");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    std::fs::create_dir(&output).unwrap();
    let fields = vec![
        (
            "App build",
            format!("DropSquash 0.1.0 git {}", current_head()),
        ),
        ("App artifact", artifact.display().to_string()),
        (
            "Input sample set",
            "short, medium, and large local recordings".into(),
        ),
        ("macOS version", "macOS 26.5.2".into()),
        ("Machine", "MacBookPro18,4 arm64".into()),
        ("Output folder", output.display().to_string()),
        (
            "Config path",
            "/Users/me/Library/Application Support/DropSquash/config.json".into(),
        ),
        (
            "History path",
            "/Users/me/Library/Application Support/DropSquash/history.jsonl".into(),
        ),
        (
            "License cache path",
            "/Users/me/Library/Application Support/DropSquash/license.json".into(),
        ),
        ("Tester", "masaki".into()),
        ("Date", "2026-07-12".into()),
    ];
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, rows(&fields).join("\n")).unwrap();

    let missing = crate::manual_qa_check::check_file(&path).unwrap();

    assert!(!missing.iter().any(|error| error.contains("manual QA App")));
    assert!(!missing
        .iter()
        .any(|error| error.contains("manual QA field")));
    assert!(!missing
        .iter()
        .any(|error| error.contains("manual QA state path")));
}

#[test]
fn generated_field_labels_match_required_manual_qa_fields() {
    let fields = vec![
        ("App build", String::new()),
        ("App artifact", String::new()),
        ("Input sample set", String::new()),
        ("macOS version", String::new()),
        ("Machine", String::new()),
        ("Output folder", String::new()),
        ("Config path", String::new()),
        ("History path", String::new()),
        ("License cache path", String::new()),
        ("Tester", String::new()),
        ("Date", String::new()),
    ];
    let generated = fields
        .iter()
        .map(|(label, _)| *label)
        .collect::<std::collections::BTreeSet<_>>();
    let required = crate::manual_qa_check::requirements::REQUIRED_FIELDS
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(generated, required);
}

#[test]
fn required_field_labels_exist_in_manual_qa_template() {
    let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();
    let missing = crate::manual_qa_check::requirements::REQUIRED_FIELDS
        .into_iter()
        .filter(|label| !template.contains(&format!("| {label} |")))
        .collect::<Vec<_>>();

    assert!(missing.is_empty(), "{missing:?}");
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}

fn current_head() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
