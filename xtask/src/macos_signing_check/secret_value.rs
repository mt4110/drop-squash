use std::collections::BTreeMap;

use super::env::value;

const SECRET_KEYS: [&str; 4] = [
    "APPLE_CERTIFICATE_PASSWORD",
    "APPLE_PASSWORD",
    "APPLE_API_KEY",
    "APPLE_API_ISSUER",
];

pub(super) fn check(env: &BTreeMap<String, String>) -> Result<(), String> {
    for key in SECRET_KEYS {
        let Some(value) = value(env, key) else {
            continue;
        };
        if is_placeholder(value) {
            return Err(format!("{key} must not be a placeholder value"));
        }
    }
    Ok(())
}

fn is_placeholder(value: &str) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "app-password"
            | "changeme"
            | "example"
            | "placeholder"
            | "secret"
            | "test"
            | "todo"
            | "password"
    )
}

#[cfg(test)]
mod tests {
    use super::is_placeholder;

    #[test]
    fn detects_placeholder_values() {
        assert!(is_placeholder("app-password"));
        assert!(is_placeholder(" TODO "));
    }

    #[test]
    fn accepts_non_placeholder_values() {
        assert!(!is_placeholder("abcd-efgh-ijkl-mnop"));
    }
}
