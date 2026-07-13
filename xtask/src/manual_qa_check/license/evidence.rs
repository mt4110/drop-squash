use super::{cache, identity};

pub(super) fn require_all(label: &str, result: &str, needles: &[&str], missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if needles.iter().all(|needle| lower.contains(needle)) {
        return;
    }
    missing.push(format!("manual QA {label} needs concrete license evidence"));
}

pub(super) fn require_any(
    label: &str,
    result: &str,
    needles: &[&str],
    missing: &mut Vec<String>,
    message: &str,
) {
    let lower = result.to_ascii_lowercase();
    if needles.iter().any(|needle| lower.contains(needle)) {
        return;
    }
    missing.push(format!("{message}: {label}"));
}

pub(super) fn require_license_cache(
    label: &str,
    result: &str,
    needles: &[&str],
    missing: &mut Vec<String>,
) {
    let lower = result.to_ascii_lowercase();
    let mentions_cache = lower.contains("cache") || lower.contains("license.json");
    if mentions_cache
        && needles.iter().all(|needle| lower.contains(needle))
        && cache::raw_key_absent(&lower)
        && cache::absence_inspection_ok(label, &lower)
        && cache::cache_observation_ok(label, &lower)
        && cache::observation_ok(label, &lower)
        && identity::fingerprint_evidence_ok(label, result)
        && identity::instance_id_evidence_ok(label, &lower)
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs license cache and raw-key evidence"
    ));
}

pub(super) fn require_action_state(
    label: &str,
    result: &str,
    needles: &[&str],
    missing: &mut Vec<String>,
) {
    let lower = result.to_ascii_lowercase();
    if needles.iter().all(|needle| lower.contains(needle)) {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs license action state evidence"
    ));
}

pub(super) fn require_any_state(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    let mentions_cache = lower.contains("cache") || lower.contains("license.json");
    let mentions_removal = mentions_cache_removal(&lower);
    let mentions_state = lower.contains("trial") || lower.contains("locked");
    if mentions_cache && mentions_removal && mentions_state && cache::forget_observation_ok(&lower)
    {
        return;
    }
    missing.push(
        "manual QA Forget license on this Mac needs cache removal and app-state evidence"
            .to_string(),
    );
}

fn mentions_cache_removal(value: &str) -> bool {
    ["removed", "cleared", "deleted", "clears"]
        .iter()
        .any(|needle| value.contains(needle))
}
