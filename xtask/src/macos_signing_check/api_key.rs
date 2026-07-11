use std::collections::BTreeMap;
use std::path::Path;

use super::env::{present, value};

pub(super) fn check_identity(env: &BTreeMap<String, String>) -> Result<(), String> {
    if !present(env, "APPLE_API_KEY") {
        return Ok(());
    }
    let key = value(env, "APPLE_API_KEY").unwrap_or_default();
    if key.len() != 10 || !key.chars().all(|value| value.is_ascii_alphanumeric()) {
        return Err("APPLE_API_KEY must be a 10-character App Store Connect key id".to_string());
    }
    let issuer = value(env, "APPLE_API_ISSUER").unwrap_or_default();
    if is_uuid_like(issuer) {
        return Ok(());
    }
    Err("APPLE_API_ISSUER must be an App Store Connect issuer UUID".to_string())
}

pub(super) fn check_path(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(path) = value(env, "APPLE_API_KEY_PATH") else {
        return Ok(());
    };
    let path = Path::new(path);
    if path.is_file() && path.extension().is_some_and(|value| value == "p8") {
        return Ok(());
    }
    Err("APPLE_API_KEY_PATH must point to a .p8 file".to_string())
}

fn is_uuid_like(value: &str) -> bool {
    let parts = value.split('-').map(str::len).collect::<Vec<_>>();
    parts == [8, 4, 4, 4, 12]
        && value
            .chars()
            .all(|char| char.is_ascii_hexdigit() || char == '-')
}
