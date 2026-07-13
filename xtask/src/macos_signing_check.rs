use std::collections::BTreeMap;

mod api_key;
mod env;
mod secret_value;
use env::{all_present, present, require_pair, value};

pub fn run() -> Result<(), String> {
    let env = std::env::vars().collect();
    check(&env)?;
    println!("macOS signing environment checks passed");
    Ok(())
}

fn check(env: &BTreeMap<String, String>) -> Result<(), String> {
    check_signing(env)?;
    check_certificate(env)?;
    check_notarization(env)?;
    secret_value::check(env)?;
    api_key::check_identity(env)?;
    api_key::check_path(env)?;
    Ok(())
}

fn check_signing(env: &BTreeMap<String, String>) -> Result<(), String> {
    if present(env, "GITHUB_ACTIONS") {
        return require_pair(
            env,
            "APPLE_CERTIFICATE",
            "APPLE_CERTIFICATE_PASSWORD",
            "CI macOS signing requires APPLE_CERTIFICATE with password",
        )
        .and_then(|()| require_keychain_password(env))
        .and_then(|()| check_ci_codesign_identity(env));
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

fn check_signing_identity(env: &BTreeMap<String, String>) -> Result<(), String> {
    let identity = value(env, "APPLE_SIGNING_IDENTITY").unwrap_or_default();
    if identity.contains("Developer ID Application") {
        return Ok(());
    }
    Err("APPLE_SIGNING_IDENTITY must be a Developer ID Application identity".to_string())
}

fn check_ci_codesign_identity(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(identity) = value(env, "APPLE_CODESIGN_IDENTITY") else {
        return Err("CI macOS signing requires APPLE_CODESIGN_IDENTITY".to_string());
    };
    if identity.contains("Developer ID Application") {
        return Ok(());
    }
    Err("APPLE_CODESIGN_IDENTITY must be a Developer ID Application identity".to_string())
}

fn check_certificate(env: &BTreeMap<String, String>) -> Result<(), String> {
    let Some(certificate) = value(env, "APPLE_CERTIFICATE") else {
        return Ok(());
    };
    let compact: String = certificate
        .chars()
        .filter(|value| !value.is_ascii_whitespace())
        .collect();
    if compact.len() >= 32 && compact.chars().all(is_base64_char) {
        return Ok(());
    }
    Err("APPLE_CERTIFICATE must be base64-encoded certificate data".to_string())
}

fn require_keychain_password(env: &BTreeMap<String, String>) -> Result<(), String> {
    if present(env, "APPLE_KEYCHAIN_PASSWORD") {
        return Ok(());
    }
    Err(
        "CI macOS signing requires APPLE_KEYCHAIN_PASSWORD for temporary keychain import"
            .to_string(),
    )
}

fn is_base64_char(value: char) -> bool {
    value.is_ascii_alphanumeric() || matches!(value, '+' | '/' | '=')
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

#[cfg(test)]
mod tests;
