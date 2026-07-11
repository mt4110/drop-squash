use super::Environment;

#[test]
fn formats_manual_qa_environment_lines() {
    let environment = Environment::from_parts(
        "15.5".to_string(),
        "MacBookPro18,3 arm64".to_string(),
        "/tmp/output".to_string(),
        "/Users/me/Library/Application Support/DropSquash".to_string(),
        "masaki".to_string(),
        "2026-07-11".to_string(),
    )
    .unwrap();

    assert_eq!(
        environment.manual_qa_lines(),
        vec![
            "manual QA macOS version: macOS 15.5",
            "manual QA Machine: MacBookPro18,3 arm64",
            "manual QA Output folder: /tmp/output",
            "manual QA Config path: /Users/me/Library/Application Support/DropSquash/config.json",
            "manual QA History path: /Users/me/Library/Application Support/DropSquash/history.jsonl",
            "manual QA License cache path: /Users/me/Library/Application Support/DropSquash/license.json",
            "manual QA Tester: masaki",
            "manual QA Date: 2026-07-11",
        ]
    );
}

#[test]
fn rejects_empty_environment_fields() {
    let error = Environment::from_parts(
        "15.5".to_string(),
        String::new(),
        "/tmp/output".to_string(),
        "/tmp/state".to_string(),
        "masaki".to_string(),
        "2026-07-11".to_string(),
    )
    .unwrap_err();

    assert!(error.contains("non-empty"));
}
