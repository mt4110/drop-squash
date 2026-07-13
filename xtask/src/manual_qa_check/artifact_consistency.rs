use crate::dmg;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const ARTIFACT_CHECK: &str = "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`";
const CHECKSUM: &str =
    "`cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS`";
const CODESIGN: &str = "Codesign verification";
const NOTARIZATION: &str = "Notarization staple verification";
const GATEKEEPER: &str = "Gatekeeper open test";

pub(super) fn validate(rows: &[(String, String)], missing: &mut Vec<String>) {
    let Some(name) = app_artifact_name(rows) else {
        return;
    };
    if !name.to_ascii_lowercase().ends_with(".dmg") {
        return;
    }
    for label in [ARTIFACT_CHECK, CHECKSUM, CODESIGN, NOTARIZATION, GATEKEEPER] {
        if let Some(result) = value_for(rows, label) {
            require_same_artifact(label, result, &name, missing);
        }
    }
    for label in [ARTIFACT_CHECK, CHECKSUM] {
        if let Some(result) = value_for(rows, label) {
            require_same_artifact_path(label, result, rows, missing);
        }
    }
    if let Some(result) = value_for(rows, ARTIFACT_CHECK) {
        require_artifact_check_context(result, missing);
    }
    require_checksum_digest(rows, missing);
}

fn app_artifact_name(rows: &[(String, String)]) -> Option<String> {
    app_artifact_path(rows)?
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

fn app_artifact_path(rows: &[(String, String)]) -> Option<PathBuf> {
    value_for(rows, "App artifact").map(PathBuf::from)
}

fn require_same_artifact(label: &str, result: &str, name: &str, missing: &mut Vec<String>) {
    if result
        .to_ascii_lowercase()
        .contains(&name.to_ascii_lowercase())
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} must reference App artifact {name}"
    ));
}

fn require_same_artifact_path(
    label: &str,
    result: &str,
    rows: &[(String, String)],
    missing: &mut Vec<String>,
) {
    let Some(path) = app_artifact_path(rows) else {
        return;
    };
    let expected = path.display().to_string();
    if result.contains(&expected) {
        return;
    }
    missing.push(format!(
        "manual QA {label} must reference App artifact path {expected}"
    ));
}

fn require_artifact_check_context(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if lower.contains("public") && lower.contains("udif") {
        return;
    }
    missing.push("manual QA artifact-check must mention public UDIF DropSquash.dmg".to_string());
}

fn require_checksum_digest(rows: &[(String, String)], missing: &mut Vec<String>) {
    let Some(path) = app_artifact_path(rows) else {
        return;
    };
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return;
    }
    let Some(result) = value_for(rows, CHECKSUM) else {
        return;
    };
    let Ok(expected) = artifact_digest(&path) else {
        return;
    };
    if result.to_ascii_lowercase().contains(&expected) {
        return;
    }
    missing.push("manual QA checksum must match App artifact digest".to_string());
}

fn artifact_digest(path: &Path) -> Result<String, String> {
    let bytes = dmg::read(path, "manual QA checksum artifact")?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn value_for<'a>(rows: &'a [(String, String)], label: &str) -> Option<&'a str> {
    rows.iter()
        .find(|(row_label, _)| row_label == label)
        .map(|(_, value)| value.as_str())
}
