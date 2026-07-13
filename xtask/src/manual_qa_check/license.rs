mod cache;
mod identity;
mod requirements;

use identity::{fingerprint_evidence_ok, instance_id_evidence_ok};

pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    match label.trim() {
        "Sandbox product setup" => {
            require_all(label, result, requirements::PRODUCT_SETUP, missing);
            require_any(
                label,
                result,
                requirements::PRIVATE_STORE_ABSENCE,
                missing,
                "manual QA Sandbox product setup must say private store IDs were not recorded",
            );
        }
        "Sandbox purchase" => require_all(label, result, requirements::PURCHASE, missing),
        "Empty key activation" => {
            require_license_cache_evidence(label, result, requirements::EMPTY_KEY_CACHE, missing);
            require_action_state(label, result, requirements::EMPTY_KEY_ACTION, missing);
        }
        "Invalid key activation" => {
            require_license_cache_evidence(label, result, requirements::INVALID_KEY_CACHE, missing);
            require_action_state(label, result, requirements::ACTIVATING_ACTION, missing);
        }
        "Valid sandbox activation" => {
            require_license_cache_evidence(label, result, requirements::VALID_KEY_CACHE, missing);
            require_action_state(label, result, requirements::ACTIVATING_ACTION, missing);
        }
        "License network failure" => require_license_cache_evidence(
            label,
            result,
            requirements::NETWORK_FAILURE_CACHE,
            missing,
        ),
        "Expired license refresh" => {
            require_license_cache_evidence(label, result, requirements::EXPIRED_REFRESH, missing)
        }
        "Forget license on this Mac" => {
            require_any_state(result, missing);
            require_action_state(label, result, requirements::FORGET_ACTION, missing);
        }
        _ => {}
    }
}

fn require_all(label: &str, result: &str, needles: &[&str], missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if needles.iter().all(|needle| lower.contains(needle)) {
        return;
    }
    missing.push(format!("manual QA {label} needs concrete license evidence"));
}

fn require_any(
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

fn require_license_cache_evidence(
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
        && fingerprint_evidence_ok(label, result)
        && instance_id_evidence_ok(label, &lower)
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs license cache and raw-key evidence"
    ));
}

fn require_action_state(label: &str, result: &str, needles: &[&str], missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    if needles.iter().all(|needle| lower.contains(needle)) {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs license action state evidence"
    ));
}

fn require_any_state(result: &str, missing: &mut Vec<String>) {
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
