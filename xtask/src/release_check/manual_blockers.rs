use super::blockers::row;
use std::path::Path;

mod benchmark;
mod field_quality;
mod mapping;
mod quality;
mod stale;

#[cfg(test)]
use mapping::PACKAGED_MACOS_EVIDENCE;

pub(super) fn check(blockers_path: &Path, manual_path: &Path) -> Result<(), String> {
    let blockers = std::fs::read_to_string(blockers_path).map_err(|error| error.to_string())?;
    let manual = std::fs::read_to_string(manual_path).map_err(|error| error.to_string())?;
    let missing = missing_manual_verified_evidence(&blockers, &manual);
    let stale = stale::manual_evidence_left_blocked(&blockers, &manual);
    if missing.is_empty() && stale.is_empty() {
        return Ok(());
    }
    Err(format!(
        "manual QA release blocker mismatch: missing verified evidence [{}] stale blocked [{}]",
        missing.join(", "),
        stale.join(", ")
    ))
}

fn missing_manual_verified_evidence(blockers: &str, manual: &str) -> Vec<&'static str> {
    mapping::MANUAL_BLOCKERS
        .iter()
        .copied()
        .filter(|(blocker, checks)| {
            row::find(blockers, blocker)
                .filter(|line| row::has_status(line, blocker, "Verified"))
                .is_some_and(|_| {
                    checks.iter().any(|check| missing_result(manual, check))
                        || lacks_blocker_evidence(blocker, manual)
                })
        })
        .map(|(blocker, _)| blocker)
        .collect()
}

fn lacks_blocker_evidence(blocker: &str, manual: &str) -> bool {
    match blocker {
        "Benchmark release set" => benchmark::lacks_matching_csv(manual),
        _ => false,
    }
}

fn missing_result(manual: &str, check: &str) -> bool {
    let matches = manual
        .lines()
        .filter(|line| matches_check(line, check))
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return true;
    }
    match result_cell(check, matches[0]) {
        Some(result) => {
            unusable_result(result)
                || field_quality::lacks_required_evidence(check, result)
                || quality::lacks_required_evidence(check, result)
        }
        None => true,
    }
}

fn matches_check(line: &str, check: &str) -> bool {
    line.starts_with('|') && line.trim_matches('|').split('|').next().map(str::trim) == Some(check)
}

fn result_cell<'a>(check: &str, line: &'a str) -> Option<&'a str> {
    let cells = line.trim_matches('|').split('|').collect::<Vec<_>>();
    let valid_cells = if mapping::is_metadata_field(check) {
        cells.len() == 2
    } else {
        matches!(cells.len(), 3..=4)
    };
    valid_cells.then(|| cells[cells.len() - 1])
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
