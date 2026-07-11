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
        return check_signing_identity(env);
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
        check_team_id(env)?;
        return Ok(());
    }
    Err("notarization requires APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID".to_string())
}

fn check_api_key_path(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(path) = value(env, "APPLE_API_KEY_PATH") else {
        return Ok(());
    };
    let path = Path::new(path);
    if path.is_file() && path.extension().is_some_and(|value| value == "p8") {
        return Ok(());
    }
    Err("APPLE_API_KEY_PATH must point to a .p8 file".to_string())
}

fn check_signing_identity(env: &BTreeMap<String, String>) -> Result<(), String> {
    let identity = value(env, "APPLE_SIGNING_IDENTITY").unwrap_or_default();
    if identity.contains("Developer ID Application") {
        return Ok(());
    }
    Err("APPLE_SIGNING_IDENTITY must be a Developer ID Application identity".to_string())
}

fn check_team_id(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(team_id) = value(env, "APPLE_TEAM_ID") else {
        return Ok(());
    };
    if team_id.len() == 10 && team_id.chars().all(|value| value.is_ascii_alphanumeric()) {
        return Ok(());
    }
    Err("APPLE_TEAM_ID must be a 10-character Apple team id".to_string())
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
