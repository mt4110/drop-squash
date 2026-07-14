use dropsquash_license::LicenseCache;

pub(super) fn format_cache_diagnostics(cache: &LicenseCache, now: u64) -> Vec<String> {
    vec![
        "raw license key persisted: no".to_string(),
        format!("license cache identity: {}", identity_label(cache)),
        format!("license cache fingerprint: {}", fingerprint_label(cache)),
        format!("license cache instance_id: {}", instance_id_label(cache)),
        format!("offline grace: {}", grace_label(cache, now)),
    ]
}

fn identity_label(cache: &LicenseCache) -> &'static str {
    if has_hex_fingerprint(cache) && has_instance_id(cache) {
        "present"
    } else {
        "missing"
    }
}

fn has_hex_fingerprint(cache: &LicenseCache) -> bool {
    cache
        .license_key_fingerprint
        .as_deref()
        .is_some_and(|value| value.len() == 64 && value.chars().all(is_lower_hex))
}

fn has_instance_id(cache: &LicenseCache) -> bool {
    cache
        .instance_id
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
}

fn fingerprint_label(cache: &LicenseCache) -> &'static str {
    if has_hex_fingerprint(cache) {
        "present 64-character lowercase hex"
    } else {
        "missing"
    }
}

fn instance_id_label(cache: &LicenseCache) -> &'static str {
    if has_instance_id(cache) {
        "present"
    } else {
        "missing"
    }
}

fn is_lower_hex(char: char) -> bool {
    char.is_ascii_digit() || matches!(char, 'a'..='f')
}

fn grace_label(cache: &LicenseCache, now: u64) -> String {
    match cache.offline_grace_until_unix {
        Some(until) if until >= now => format!("active until unix {until}"),
        Some(until) => format!("expired at unix {until}"),
        None => "absent".to_string(),
    }
}
