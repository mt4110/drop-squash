use std::path::PathBuf;

use dropsquash_license::{license_key_fingerprint, LicenseCache};

#[cfg(test)]
mod tests;

const INSTANCE_ID: &str = "remote-device-1";
const INSTANCE_NAME: &str = "device-1";
const QA_KEY: &str = "LS-QA-NETWORK-FAILURE";
const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-seed-license-cache <license.json> [--expired]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (path, expired) = parse_args(args)?;
    cache(expired)
        .save_to_path(&path)
        .map_err(|error| error.to_string())?;
    println!(
        "manual QA seeded {} license cache: {}",
        if expired { "expired" } else { "valid" },
        path.display()
    );
    println!("instance_id: {INSTANCE_ID}");
    println!("fingerprint: {}", license_key_fingerprint(QA_KEY));
    println!(
        "license cache inspect command: sed -n '1,160p' \"{}\"",
        path.display()
    );
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, bool), String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok((PathBuf::from(path), false)),
        [path, flag] if flag == "--expired" => Ok((PathBuf::from(path), true)),
        [flag, path] if flag == "--expired" => Ok((PathBuf::from(path), true)),
        _ => Err(USAGE.to_string()),
    }
}

fn cache(expired: bool) -> LicenseCache {
    LicenseCache {
        instance_name: Some(INSTANCE_NAME.to_string()),
        instance_id: Some(INSTANCE_ID.to_string()),
        license_key_fingerprint: Some(license_key_fingerprint(QA_KEY)),
        validated_at_unix: Some(100),
        offline_grace_until_unix: Some(if expired { 101 } else { u64::MAX }),
        valid: true,
        ..LicenseCache::default()
    }
}
