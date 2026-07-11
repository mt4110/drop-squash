use super::blockers::row;
use std::path::Path;

mod quality;

const PACKAGED_MACOS_EVIDENCE: &[&str] = &[
    "App build",
    "App artifact",
    "macOS version",
    "Machine",
    "Input sample set",
    "Output folder",
    "Tester",
    "Date",
    "Choose recording conversion",
    "Drag-and-drop conversion",
    "Privacy receipt sidecar",
    "Reveal privacy receipt",
    "Duplicate output naming",
    "Cancellation",
    "Multi-file queue",
    "Queued job cancellation",
    "Batch summary",
    "Ask source policy",
    "Trash source policy",
    "Failed conversion",
    "Larger output",
    "Reveal output",
];

const MANUAL_BLOCKERS: [(&str, &[&str]); 7] = [
    ("Packaged macOS manual QA", PACKAGED_MACOS_EVIDENCE),
    ("Lemon Squeezy product setup", &["Sandbox product setup"]),
    ("Lemon Squeezy sandbox purchase", &["Sandbox purchase"]),
    ("Valid sandbox activation", &["Valid sandbox activation"]),
    ("Invalid license key handling", &["Invalid key activation"]),
    ("Local license forget", &["Forget license on this Mac"]),
    ("Gatekeeper clean-machine open", &["Gatekeeper open test"]),
];

pub(super) fn check(blockers_path: &Path, manual_path: &Path) -> Result<(), String> {
    let blockers = std::fs::read_to_string(blockers_path).map_err(|error| error.to_string())?;
    let manual = std::fs::read_to_string(manual_path).map_err(|error| error.to_string())?;
    let missing = missing_manual_verified_evidence(&blockers, &manual);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "verified release blockers need manual QA evidence: {}",
        missing.join(", ")
    ))
}

fn missing_manual_verified_evidence(blockers: &str, manual: &str) -> Vec<&'static str> {
    MANUAL_BLOCKERS
        .iter()
        .copied()
        .filter(|(blocker, checks)| {
            row::find(blockers, blocker)
                .filter(|line| row::has_status(line, blocker, "Verified"))
                .is_some_and(|_| checks.iter().any(|check| missing_result(manual, check)))
        })
        .map(|(blocker, _)| blocker)
        .collect()
}

fn missing_result(manual: &str, check: &str) -> bool {
    match manual
        .lines()
        .find(|line| line.starts_with('|') && line.contains(&format!("| {check} |")))
    {
        Some(line) => match line.trim_matches('|').split('|').next_back() {
            Some(result) => {
                unusable_result(result) || quality::lacks_required_evidence(check, result)
            }
            None => true,
        },
        None => true,
    }
}

fn unusable_result(result: &str) -> bool {
    matches!(
        result.trim().to_ascii_lowercase().as_str(),
        "" | "tbd"
            | "todo"
            | "n/a"
            | "na"
            | "none"
            | "blocked"
            | "skipped"
            | "pass"
            | "ok"
            | "done"
            | "works"
            | "verified"
            | "observed expected behavior"
    )
}

#[cfg(test)]
mod tests;
