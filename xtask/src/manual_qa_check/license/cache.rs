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
    !has_raw_key_contradiction(value)
        && (value.contains("raw key absent")
            || value.contains("raw key is absent")
            || value.contains("no raw key")
            || value.contains("without raw key"))
}

fn has_raw_key_contradiction(value: &str) -> bool {
    [
        "raw key persisted",
        "raw key present",
        "raw key stored",
        "raw key written",
        "raw key saved",
        "persisted raw key",
        "stored raw key",
        "saved raw key",
    ]
    .iter()
    .any(|needle| value.contains(needle))
}

pub(super) fn observation_ok(label: &str, value: &str) -> bool {
    match label {
        "License network failure" => {
            value.contains("checked") || value.contains("inspected") || value.contains("confirmed")
        }
        "Expired license refresh" => {
            (value.contains("attempted") || value.contains("attempt"))
                && (value.contains("checked")
                    || value.contains("inspected")
                    || value.contains("confirmed"))
        }
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
