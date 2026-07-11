use std::path::{Path, PathBuf};

mod evidence;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let notes = args
        .first()
        .ok_or_else(|| "publish-check requires <release-notes.md>".to_string())?;
    crate::release_check::run()?;
    ensure_website_complete(Path::new("website"))?;
    ensure_manual_qa_complete(Path::new("docs/manual-qa.md"))?;
    crate::release_notes_check::check_file(&PathBuf::from(notes))?;
    let blockers = read_release_blockers(Path::new("docs/release-blockers.md"))?;
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

fn read_release_blockers(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read release blockers {}: {error}",
            path.display()
        )
    })
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
                .is_some_and(|reference| evidence::matches(blocker, reference))
    })
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
