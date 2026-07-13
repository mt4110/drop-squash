mod cache;
mod evidence;
mod identity;
mod requirements;

pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    match label.trim() {
        "Sandbox product setup" => {
            evidence::require_all(label, result, requirements::PRODUCT_SETUP, missing);
            evidence::require_any(
                label,
                result,
                requirements::PRIVATE_STORE_ABSENCE,
                missing,
                "manual QA Sandbox product setup must say private store IDs were not recorded",
            );
        }
        "Sandbox purchase" => {
            evidence::require_all(label, result, requirements::PURCHASE, missing);
            if !has_order_id(result) {
                missing.push("manual QA result needs concrete order id: Sandbox purchase".into());
            }
        }
        "Empty key activation" => {
            evidence::require_license_cache(label, result, requirements::EMPTY_KEY_CACHE, missing);
            evidence::require_action_state(label, result, requirements::EMPTY_KEY_ACTION, missing);
        }
        "Invalid key activation" => {
            evidence::require_license_cache(
                label,
                result,
                requirements::INVALID_KEY_CACHE,
                missing,
            );
            evidence::require_action_state(label, result, requirements::ACTIVATING_ACTION, missing);
        }
        "Valid sandbox activation" => {
            evidence::require_license_cache(label, result, requirements::VALID_KEY_CACHE, missing);
            evidence::require_action_state(label, result, requirements::ACTIVATING_ACTION, missing);
        }
        "License network failure" => evidence::require_license_cache(
            label,
            result,
            requirements::NETWORK_FAILURE_CACHE,
            missing,
        ),
        "Expired license refresh" => {
            evidence::require_license_cache(label, result, requirements::EXPIRED_REFRESH, missing)
        }
        "Forget license on this Mac" => {
            evidence::require_any_state(result, missing);
            evidence::require_action_state(label, result, requirements::FORGET_ACTION, missing);
        }
        _ => {}
    }
}

fn has_order_id(result: &str) -> bool {
    result
        .to_ascii_lowercase()
        .split(|value: char| !value.is_ascii_alphanumeric())
        .collect::<Vec<_>>()
        .windows(2)
        .any(|parts| parts[0] == "order" && parts[1].chars().any(|value| value.is_ascii_digit()))
}
