use std::path::Path;

const REQUIRED_BLOCKERS: [&str; 8] = [
    "Packaged macOS manual QA",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Signed DMG",
    "Notarized and stapled DMG",
    "Gatekeeper clean-machine open",
    "Published checksum",
    "Homebrew cask install",
];

pub(super) fn check_release_blockers(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = missing_release_blockers(&text);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} is missing release blockers: {}",
        path.display(),
        missing.join(", ")
    ))
}

fn missing_release_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| !text.contains(&format!("| {blocker} |")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{missing_release_blockers, REQUIRED_BLOCKERS};

    #[test]
    fn accepts_all_required_release_blockers() {
        let text = REQUIRED_BLOCKERS
            .iter()
            .map(|blocker| format!("| {blocker} | Evidence required |\n"))
            .collect::<String>();

        assert!(missing_release_blockers(&text).is_empty());
    }

    #[test]
    fn reports_missing_release_blocker() {
        let missing = missing_release_blockers("");

        assert!(missing.contains(&"Signed DMG"));
    }
}
