use std::collections::BTreeMap;

use super::manual_rows::PendingSummary;

pub(super) fn blocker_notes(
    required: &[String],
    statuses: &BTreeMap<String, &'static str>,
    manual_path: &std::path::Path,
    manual_rows: Option<&PendingSummary>,
) -> Vec<String> {
    let packaged_blocked = required.iter().any(|name| {
        name == "Packaged macOS manual QA"
            && statuses
                .get(name)
                .map_or(true, |status| *status != "Verified")
    });
    if !packaged_blocked || !manual_rows.is_some_and(|summary| summary.other.is_empty()) {
        return Vec::new();
    }
    let mut notes = vec![format!(
        "manual QA packaged-app note: local-proof rows are complete, but Packaged macOS manual QA stays blocked until {} is filled against the tested public DropSquash.dmg and manual-qa-check passes",
        manual_path.display()
    )];
    if let Some(csv) = manual_rows.and_then(|summary| summary.benchmark_csv.as_ref()) {
        notes.push(format!("manual QA packaged-app benchmark CSV: {csv}"));
    }
    if let Some(build) = manual_rows.and_then(|summary| summary.app_build.as_ref()) {
        if !crate::git_head_match::contains_current_short_head_after_git(build).unwrap_or(false) {
            notes.push(format!(
                "manual QA App build note: recorded App build `{build}` does not match the current HEAD; regenerate the prepared draft after rebuilding the app artifact before trusting packaged proof"
            ));
        }
    }
    if let Some(artifact) = manual_rows.and_then(|summary| summary.app_artifact.as_ref()) {
        if artifact.contains("/target/release/bundle/") {
            notes.push(format!(
                "manual QA App artifact note: recorded App artifact still points at a local build output `{artifact}`; rerun the packaged table against the tested public DropSquash.dmg before changing the blocker"
            ));
        }
    }
    notes
}
