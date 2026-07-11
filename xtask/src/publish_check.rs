use std::path::PathBuf;

const BLOCKERS: [&str; 10] = [
    "Packaged macOS manual QA",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Public website deployment",
    "Live checkout link",
    "Signed DMG",
    "Notarized and stapled DMG",
    "Gatekeeper clean-machine open",
    "Published checksum",
    "Homebrew cask install",
];

pub fn run(args: Vec<String>) -> Result<(), String> {
    let notes = args
        .first()
        .ok_or_else(|| "publish-check requires <release-notes.md>".to_string())?;
    crate::release_check::run()?;
    crate::release_notes_check::check_file(&PathBuf::from(notes))?;
    let blockers =
        std::fs::read_to_string("docs/release-blockers.md").map_err(|error| error.to_string())?;
    let unverified = unverified_blockers(&blockers);
    if unverified.is_empty() {
        println!("publish checks passed");
        return Ok(());
    }
    Err(format!(
        "release blockers must be Verified before publish: {}",
        unverified.join(", ")
    ))
}

fn unverified_blockers(text: &str) -> Vec<&'static str> {
    BLOCKERS
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
        cells.first() == Some(&blocker) && cells.get(1) == Some(&"Verified")
    })
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
