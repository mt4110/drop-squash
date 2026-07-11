use std::collections::BTreeMap;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let env = std::env::vars().collect();
    check(&env)?;
    println!("macOS signing environment checks passed");
    Ok(())
}

fn check(env: &BTreeMap<String, String>) -> Result<(), String> {
    check_signing(env)?;
    check_notarization(env)?;
    check_api_key_path(env)?;
    Ok(())
}

fn check_signing(env: &BTreeMap<String, String>) -> Result<(), String> {
    if present(env, "GITHUB_ACTIONS") {
        return require_pair(
            env,
            "APPLE_CERTIFICATE",
            "APPLE_CERTIFICATE_PASSWORD",
            "CI macOS signing requires APPLE_CERTIFICATE with password",
        );
    }
    if present(env, "APPLE_SIGNING_IDENTITY") {
        return Ok(());
    }
    require_pair(
        env,
        "APPLE_CERTIFICATE",
        "APPLE_CERTIFICATE_PASSWORD",
        "macOS signing requires APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with password",
    )
}

fn check_notarization(env: &BTreeMap<String, String>) -> Result<(), String> {
    let api_key = all_present(
        env,
        &["APPLE_API_KEY", "APPLE_API_ISSUER", "APPLE_API_KEY_PATH"],
    );
    let apple_id = all_present(env, &["APPLE_ID", "APPLE_PASSWORD", "APPLE_TEAM_ID"]);
    if api_key || apple_id {
        return Ok(());
    }
    Err("notarization requires APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID".to_string())
}

fn check_api_key_path(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(path) = value(env, "APPLE_API_KEY_PATH") else {
        return Ok(());
    };
    if Path::new(path).is_file() {
        return Ok(());
    }
    Err("APPLE_API_KEY_PATH must point to a .p8 file".to_string())
}

fn require_pair(
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

fn all_present(env: &BTreeMap<String, String>, keys: &[&str]) -> bool {
    keys.iter().all(|key| present(env, key))
}

fn present(env: &BTreeMap<String, String>, key: &str) -> bool {
    value(env, key).is_some_and(|value| !value.trim().is_empty())
}

fn value<'a>(env: &'a BTreeMap<String, String>, key: &str) -> Option<&'a str> {
    env.get(key).map(String::as_str)
}

#[cfg(test)]
mod tests;
