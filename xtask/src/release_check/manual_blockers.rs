use super::blockers::row;
use std::path::Path;

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

const MANUAL_BLOCKERS: [(&str, &[&str]); 6] = [
    ("Packaged macOS manual QA", PACKAGED_MACOS_EVIDENCE),
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
                unusable_result(result) || lacks_required_license_evidence(check, result)
            }
            None => true,
        },
        None => true,
    }
}

fn lacks_required_license_evidence(check: &str, result: &str) -> bool {
    let lower = result.to_ascii_lowercase();
    let mentions_cache = lower.contains("cache") || lower.contains("license.json");
    match check {
        "Valid sandbox activation" => {
            !(mentions_cache && lower.contains("pro") && lower.contains("raw key"))
        }
        "Invalid key activation" => {
            !(mentions_cache && lower.contains("friendly") && lower.contains("raw key"))
        }
        "Forget license on this Mac" => {
            let mentions_state = lower.contains("trial") || lower.contains("locked");
            !(mentions_cache && mentions_state)
        }
        _ => false,
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
