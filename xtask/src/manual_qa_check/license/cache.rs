pub(super) fn absence_inspection_ok(label: &str, value: &str) -> bool {
    if !matches!(label, "Empty key activation" | "Invalid key activation") {
        return true;
    }
    value.contains("checked") || value.contains("inspected")
}

pub(super) fn cache_observation_ok(label: &str, value: &str) -> bool {
    if !matches!(label, "Valid sandbox activation") {
        return true;
    }
    value.contains("checked") || value.contains("inspected") || value.contains("confirmed")
}

pub(super) fn raw_key_absent(value: &str) -> bool {
    value.contains("raw key absent")
        || value.contains("raw key is absent")
        || value.contains("no raw key")
        || value.contains("without raw key")
}

pub(super) fn observation_ok(label: &str, value: &str) -> bool {
    match label {
        "License network failure" => {
            value.contains("checked") || value.contains("inspected") || value.contains("confirmed")
        }
        "Expired license refresh" => value.contains("attempted") || value.contains("attempt"),
        _ => true,
    }
}

pub(super) fn forget_observation_ok(value: &str) -> bool {
    let observed_cache =
        value.contains("checked") || value.contains("confirmed") || value.contains("inspected");
    let observed_state =
        value.contains("observed") || value.contains("confirmed") || value.contains("returned");
    observed_cache && observed_state
}
