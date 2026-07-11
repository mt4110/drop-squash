pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    match label.trim() {
        "Sandbox product setup" => require_all(
            label,
            result,
            &["dropsquash", "intended product", "license keys enabled"],
            missing,
        ),
        "Sandbox purchase" => require_all(
            label,
            result,
            &["intended product", "test buyer", "order"],
            missing,
        ),
        "Empty key activation" => {
            require_license_cache_evidence(label, result, &["friendly", "raw key"], missing)
        }
        "Invalid key activation" => {
            require_license_cache_evidence(label, result, &["friendly", "raw key"], missing)
        }
        "Valid sandbox activation" => {
            require_license_cache_evidence(label, result, &["pro", "raw key"], missing)
        }
        "License network failure" => require_license_cache_evidence(
            label,
            result,
            &[
                "friendly",
                "network",
                "existing",
                "valid",
                "preserved",
                "raw key",
            ],
            missing,
        ),
        "Forget license on this Mac" => require_any_state(result, missing),
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
        && raw_key_absent(&lower)
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs license cache and raw-key evidence"
    ));
}

fn raw_key_absent(value: &str) -> bool {
    value.contains("raw key absent")
        || value.contains("raw key is absent")
        || value.contains("no raw key")
        || value.contains("without raw key")
}

fn require_any_state(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    let mentions_cache = lower.contains("cache") || lower.contains("license.json");
    let mentions_removal = mentions_cache_removal(&lower);
    let mentions_state = lower.contains("trial") || lower.contains("locked");
    if mentions_cache && mentions_removal && mentions_state {
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
