use std::collections::BTreeMap;

pub(super) fn require_pair(
    env: &BTreeMap<String, String>,
    left: &str,
    right: &str,
    error: &str,
) -> Result<(), String> {
    if present(env, left) && present(env, right) {
        return Ok(());
    }
    Err(error.to_string())
}

pub(super) fn all_present(env: &BTreeMap<String, String>, keys: &[&str]) -> bool {
    keys.iter().all(|key| present(env, key))
}

pub(super) fn missing(env: &BTreeMap<String, String>, keys: &[&str]) -> Vec<String> {
    keys.iter()
        .filter(|key| !present(env, key))
        .map(|key| (*key).to_string())
        .collect()
}

pub(super) fn incomplete_error(
    env: &BTreeMap<String, String>,
    keys: &[&str],
    prefix: &str,
) -> Option<String> {
    let missing = missing(env, keys);
    (!missing.is_empty() && missing.len() < keys.len())
        .then(|| format!("{prefix}; missing {}", missing.join(", ")))
}

pub(super) fn is_base64_char(value: char) -> bool {
    value.is_ascii_alphanumeric() || matches!(value, '+' | '/' | '=')
}

pub(super) fn present(env: &BTreeMap<String, String>, key: &str) -> bool {
    value(env, key).is_some_and(|value| !value.trim().is_empty())
}

pub(super) fn require_keychain_password(env: &BTreeMap<String, String>) -> Result<(), String> {
    if present(env, "APPLE_KEYCHAIN_PASSWORD") {
        return Ok(());
    }
    Err(
        "CI macOS signing requires APPLE_KEYCHAIN_PASSWORD for temporary keychain import"
            .to_string(),
    )
}

pub(super) fn check_team_id(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(team_id) = value(env, "APPLE_TEAM_ID") else {
        return Ok(());
    };
    if team_id.len() == 10 && team_id.chars().all(|value| value.is_ascii_alphanumeric()) {
        return Ok(());
    }
    Err("APPLE_TEAM_ID must be a 10-character Apple team id".to_string())
}

pub(super) fn value<'a>(env: &'a BTreeMap<String, String>, key: &str) -> Option<&'a str> {
    env.get(key).map(String::as_str)
}
