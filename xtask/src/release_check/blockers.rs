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
    let invalid = invalid_status_rows(&text);
    if missing.is_empty() && invalid.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} has release blocker issues: {}{}",
        path.display(),
        join_prefix("missing ", missing),
        join_prefix(" invalid status ", invalid)
    ))
}

fn missing_release_blockers(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| !text.contains(&format!("| {blocker} |")))
        .collect()
}

fn invalid_status_rows(text: &str) -> Vec<&'static str> {
    REQUIRED_BLOCKERS
        .iter()
        .copied()
        .filter(|blocker| !text.lines().any(|line| has_allowed_status(line, blocker)))
        .collect()
}

fn has_allowed_status(line: &str, blocker: &str) -> bool {
    line.starts_with(&format!("| {blocker} | Blocked |"))
        || line.starts_with(&format!("| {blocker} | Verified |"))
}

fn join_prefix(prefix: &str, values: Vec<&str>) -> String {
    if values.is_empty() {
        return String::new();
    }
    format!("{prefix}{}", values.join(", "))
}

#[cfg(test)]
mod tests {
    use super::{invalid_status_rows, missing_release_blockers, REQUIRED_BLOCKERS};

    #[test]
    fn accepts_all_required_release_blockers() {
        let text = REQUIRED_BLOCKERS
            .iter()
            .map(|blocker| format!("| {blocker} | Blocked | Evidence required |\n"))
            .collect::<String>();

        assert!(missing_release_blockers(&text).is_empty());
        assert!(invalid_status_rows(&text).is_empty());
    }

    #[test]
    fn reports_missing_release_blocker() {
        let missing = missing_release_blockers("");

        assert!(missing.contains(&"Signed DMG"));
    }

    #[test]
    fn reports_missing_release_blocker_status() {
        let text = REQUIRED_BLOCKERS
            .iter()
            .map(|blocker| format!("| {blocker} | Evidence required |\n"))
            .collect::<String>();
        let invalid = invalid_status_rows(&text);

        assert!(invalid.contains(&"Signed DMG"));
    }
}
