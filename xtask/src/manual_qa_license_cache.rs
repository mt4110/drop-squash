use dropsquash_license::LicenseCache;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-license-cache <license.json>";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let cache = LicenseCache::load_or_default(std::path::Path::new(&path))
        .map_err(|error| format!("failed to read license cache: {error}"))?;
    println!("manual QA license cache: {path}");
    for line in cache_summary_lines(&cache) {
        println!("{line}");
    }
    println!(
        "manual QA cache evidence: raw license key persisted: no, fingerprint {}, and instance_id {}",
        fingerprint_label(&cache),
        instance_id_label(&cache)
    );
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<String, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(path.clone()),
        _ => Err(USAGE.to_string()),
    }
}

pub(crate) fn cache_summary_lines(cache: &LicenseCache) -> [String; 4] {
    [
        "raw license key persisted: no".to_string(),
        format!("license cache fingerprint: {}", fingerprint_label(cache)),
        format!("license cache instance_id: {}", instance_id_label(cache)),
        format!("offline grace: {}", grace_label(cache)),
    ]
}

pub(crate) fn fingerprint_label(cache: &LicenseCache) -> &'static str {
    cache
        .license_key_fingerprint
        .as_deref()
        .filter(|value| value.len() == 64 && value.chars().all(is_lower_hex))
        .map(|_| "present 64-character lowercase hex")
        .unwrap_or("missing")
}

pub(crate) fn instance_id_label(cache: &LicenseCache) -> &'static str {
    cache
        .instance_id
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .map(|_| "present")
        .unwrap_or("missing")
}

pub(crate) fn grace_label(cache: &LicenseCache) -> String {
    match cache.offline_grace_until_unix {
        Some(until) => format!("recorded at unix {until}"),
        None => "absent".to_string(),
    }
}

fn is_lower_hex(char: char) -> bool {
    char.is_ascii_digit() || matches!(char, 'a'..='f')
}
