use std::fs;

use super::lines;

#[test]
fn reports_missing_cache_and_non_isolated_history() {
    let directory = tempfile::tempdir().unwrap();
    let manual = directory.path().join("manual.md");
    fs::write(
        &manual,
        "| Config path | /tmp/config.json |\n| History path | /Users/example/history.jsonl |\n| License cache path | /tmp/license.json |\n",
    )
    .unwrap();
    let lines = lines(&manual).unwrap();
    assert!(lines.iter().any(|line| line
        .contains("license diagnostics command: cargo run -p dropsquash -- license status")));
    assert!(lines.iter().any(|line| line.contains(
        "fresh packaged-app launch command: cargo run -p xtask -- manual-qa-launch-app"
    )));
    assert!(lines
        .iter()
        .any(|line| line.contains("license cache status: missing /tmp/license.json")));
    assert!(lines
        .iter()
        .any(|line| line
            .contains("license cache inspect command: sed -n '1,160p' '/tmp/license.json'")));
    assert!(lines.iter().any(|line| line
        .contains("license cache helper command: cargo run -p xtask -- manual-qa-license-cache")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license cache summary: raw license key persisted: no")));
    assert!(lines.iter().any(|line| line
        .contains("license history status: non-isolated path /Users/example/history.jsonl")));
}

#[test]
fn reports_existing_cache() {
    let directory = tempfile::tempdir().unwrap();
    let cache = directory.path().join("license.json");
    let manual = directory.path().join("manual.md");
    fs::write(&cache, "{}").unwrap();
    fs::write(
        &manual,
        format!(
            "| History path | /tmp/history.jsonl |\n| License cache path | {} |\n",
            cache.display()
        ),
    )
    .unwrap();
    let lines = lines(&manual).unwrap();
    assert!(lines
        .iter()
        .any(|line| line.contains("license cache status: found")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license cache inspect command: sed -n '1,160p'")));
    assert!(lines.iter().any(|line| line
        .contains("license cache helper command: cargo run -p xtask -- manual-qa-license-cache")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license cache summary: license cache fingerprint: missing")));
    assert!(lines
        .iter()
        .any(|line| line.contains("license history status: isolated path /tmp/history.jsonl")));
}
