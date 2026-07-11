use super::blockers::row;
use std::path::Path;

const MANUAL_BLOCKERS: [(&str, &[&str]); 4] = [
    ("Packaged macOS manual QA", &["Choose recording conversion"]),
    ("Lemon Squeezy sandbox purchase", &["Sandbox purchase"]),
    ("Valid sandbox activation", &["Valid sandbox activation"]),
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
            Some(result) => result.trim().is_empty(),
            None => true,
        },
        None => true,
    }
}

#[cfg(test)]
mod tests;
