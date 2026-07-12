use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

pub(super) fn require_notes_sha_matches_manual_qa(
    notes: &str,
    manual_qa_path: &Path,
) -> Result<(), String> {
    let expected = field_value(notes, "SHA-256")
        .ok_or_else(|| "release notes SHA-256 must be present before publish".to_string())?;
    let manual_qa = std::fs::read_to_string(manual_qa_path).map_err(|error| {
        format!(
            "failed to read manual QA {}: {error}",
            manual_qa_path.display()
        )
    })?;
    let artifact = manual_qa_artifact(&manual_qa)?;
    let actual = artifact_sha(&artifact)?;
    if expected == actual {
        return Ok(());
    }
    Err("release notes SHA-256 must match manual QA App artifact".to_string())
}

fn manual_qa_artifact(text: &str) -> Result<PathBuf, String> {
    field_value(text, "App artifact")
        .map(PathBuf::from)
        .ok_or_else(|| "manual QA App artifact must be present before publish".to_string())
}

fn artifact_sha(path: &Path) -> Result<String, String> {
    let bytes = crate::dmg::read(path, "publish manual QA artifact")?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    let table_label = format!("| {label} |");
    let note_label = format!("- {label}:");
    text.lines().find_map(|line| {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix(&note_label) {
            return Some(value.trim());
        }
        if !trimmed.starts_with(&table_label) {
            return None;
        }
        trimmed.trim_matches('|').split('|').map(str::trim).nth(1)
    })
}

#[cfg(test)]
mod tests;
