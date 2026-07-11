pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    match label.trim() {
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
    if mentions_cache && needles.iter().all(|needle| lower.contains(needle)) {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs license cache and raw-key evidence"
    ));
}

fn require_any_state(result: &str, missing: &mut Vec<String>) {
    let lower = result.to_ascii_lowercase();
    let mentions_cache = lower.contains("cache") || lower.contains("license.json");
    let mentions_state = lower.contains("trial") || lower.contains("locked");
    if mentions_cache && mentions_state {
        return;
    }
    missing.push(
        "manual QA Forget license on this Mac needs cache and app-state evidence".to_string(),
    );
}
