pub(super) fn absence_inspection_ok(label: &str, value: &str) -> bool {
    if !matches!(label, "Empty key activation" | "Invalid key activation") {
        return true;
    }
    value.contains("checked") || value.contains("inspected")
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
