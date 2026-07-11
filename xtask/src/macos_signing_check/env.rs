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

pub(super) fn present(env: &BTreeMap<String, String>, key: &str) -> bool {
    value(env, key).is_some_and(|value| !value.trim().is_empty())
}

pub(super) fn value<'a>(env: &'a BTreeMap<String, String>, key: &str) -> Option<&'a str> {
    env.get(key).map(String::as_str)
}
