use std::path::{Path, PathBuf};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let notes = args
        .first()
        .ok_or_else(|| "publish-check requires <release-notes.md>".to_string())?;
    crate::release_check::run()?;
    ensure_website_complete(Path::new("website"))?;
    ensure_manual_qa_complete(Path::new("docs/manual-qa.md"))?;
    crate::release_notes_check::check_file(&PathBuf::from(notes))?;
    let blockers =
        std::fs::read_to_string("docs/release-blockers.md").map_err(|error| error.to_string())?;
    let unverified = unverified_blockers(&blockers);
    if unverified.is_empty() {
        println!("publish checks passed");
        return Ok(());
    }
    Err(unverified_blockers_error(&unverified))
}

fn ensure_website_complete(path: &Path) -> Result<(), String> {
    crate::website_check::check_path(path)
        .map_err(|error| format!("website must pass before publish:\n{error}"))
}

fn ensure_manual_qa_complete(path: &Path) -> Result<(), String> {
    let missing = crate::manual_qa_check::check_file(path)?;
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "manual QA must pass before publish:\n{}",
        missing.join("\n")
    ))
}

fn unverified_blockers(text: &str) -> Vec<&'static str> {
    crate::release_check::required_blockers()
        .iter()
        .copied()
        .filter(|blocker| !is_verified(text, blocker))
        .collect()
}

fn is_verified(text: &str, blocker: &str) -> bool {
    text.lines().any(|line| {
        let Some(cells) = cells(line) else {
            return false;
        };
        cells.first() == Some(&blocker)
            && cells.get(1) == Some(&"Verified")
            && cells
                .get(3)
                .is_some_and(|reference| has_publish_evidence(reference))
    })
}

fn has_publish_evidence(reference: &str) -> bool {
    let reference = reference.trim().trim_matches('`');
    if matches!(reference, "" | "TBD") {
        return false;
    }
    reference.starts_with("docs/")
        || reference.starts_with("https://")
        || reference == "Release notes"
        || reference.starts_with("GitHub Release https://")
        || reference.starts_with("Homebrew tap PR https://")
}

fn unverified_blockers_error(blockers: &[&str]) -> String {
    format!(
        "release blockers must be Verified with traceable Evidence reference before publish: {}",
        blockers.join(", ")
    )
}

fn cells(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') {
        return None;
    }
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    (cells.len() == 5).then_some(cells)
}

#[cfg(test)]
mod tests;
