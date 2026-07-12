use std::path::{Path, PathBuf};

mod artifact;
mod benchmark;
mod blockers;
mod evidence;
mod manual_qa;
mod references;
mod source_commit;
mod urls;

#[cfg(test)]
use blockers::{unverified_blockers, unverified_blockers_error};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let notes = args
        .first()
        .ok_or_else(|| "publish-check requires <release-notes.md>".to_string())?;
    crate::release_check::run()?;
    ensure_website_complete(Path::new("website"))?;
    ensure_manual_qa_complete(Path::new("docs/manual-qa.md"))?;
    let notes_path = PathBuf::from(notes);
    crate::release_notes_check::check_file(&notes_path)?;
    let notes_text = read_release_notes(&notes_path)?;
    source_commit::require_current(&notes_text)?;
    artifact::require_notes_sha_matches_manual_qa(&notes_text, Path::new("docs/manual-qa.md"))?;
    benchmark::require_notes_csv_matches_manual_qa(&notes_text, Path::new("docs/manual-qa.md"))?;
    let blockers = read_release_blockers(Path::new("docs/release-blockers.md"))?;
    let unverified = blockers::unverified_blockers(&blockers);
    let mismatched = references::mismatched(&blockers, &notes_text);
    if unverified.is_empty() && mismatched.is_empty() {
        println!("publish checks passed");
        return Ok(());
    }
    Err(publish_blockers_error(&unverified, &mismatched))
}

fn ensure_website_complete(path: &Path) -> Result<(), String> {
    crate::website_check::check_path(path)
        .map_err(|error| format!("website must pass before publish:\n{error}"))
}

fn ensure_manual_qa_complete(path: &Path) -> Result<(), String> {
    let missing = crate::manual_qa_check::check_file(path)?;
    if !missing.is_empty() {
        return Err(format!(
            "manual QA must pass before publish:\n{}",
            missing.join("\n")
        ));
    }
    manual_qa::require_public_dmg(path)?;
    manual_qa::require_current_head(path)
}

fn read_release_blockers(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read release blockers {}: {error}",
            path.display()
        )
    })
}

fn read_release_notes(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read release notes {}: {error}", path.display()))
}

fn publish_blockers_error(unverified: &[&str], mismatched: &[&str]) -> String {
    let mut errors = Vec::new();
    if !unverified.is_empty() {
        errors.push(blockers::unverified_blockers_error(unverified));
    }
    if !mismatched.is_empty() {
        errors.push(format!(
            "release blocker references must match release notes URLs before publish: {}",
            mismatched.join(", ")
        ));
    }
    errors.join("\n")
}

#[cfg(test)]
mod tests;
