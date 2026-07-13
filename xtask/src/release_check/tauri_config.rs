use std::path::Path;

use serde_json::Value;

const REQUIRED_PRODUCT_NAME: &str = "DropSquash";
const REQUIRED_IDENTIFIER: &str = "io.github.mt4110.dropsquash";
const REQUIRED_CONNECT_SRC: &str = "ipc: http://ipc.localhost";
const REQUIRED_TARGETS: [&str; 2] = ["app", "dmg"];

pub(super) fn check(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let config: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    let mut errors = Vec::new();
    require_string(
        &config,
        &["productName"],
        REQUIRED_PRODUCT_NAME,
        &mut errors,
    );
    require_string(&config, &["identifier"], REQUIRED_IDENTIFIER, &mut errors);
    require_string(
        &config,
        &["app", "security", "csp", "connect-src"],
        REQUIRED_CONNECT_SRC,
        &mut errors,
    );
    require_targets(&config, &mut errors);
    reject_updater_config(&config, &mut errors);
    if errors.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} release metadata is invalid: {}",
        path.display(),
        errors.join(", ")
    ))
}

fn require_string(config: &Value, path: &[&str], expected: &str, errors: &mut Vec<String>) {
    if value_at(config, path).and_then(Value::as_str) == Some(expected) {
        return;
    }
    errors.push(format!("{} must be {expected}", path.join(".")));
}

fn require_targets(config: &Value, errors: &mut Vec<String>) {
    let targets = value_at(config, &["bundle", "targets"]).and_then(Value::as_array);
    let Some(targets) = targets else {
        errors.push("bundle.targets must include app and dmg".to_string());
        return;
    };
    for required in REQUIRED_TARGETS {
        if !targets
            .iter()
            .any(|target| target.as_str() == Some(required))
        {
            errors.push(format!("bundle.targets must include {required}"));
        }
    }
}

fn reject_updater_config(value: &Value, errors: &mut Vec<String>) {
    if contains_updater(value) {
        errors.push("updater config must stay absent until signed updates are ready".to_string());
    }
}

fn contains_updater(value: &Value) -> bool {
    match value {
        Value::Object(object) => object.iter().any(|(key, value)| {
            key.to_ascii_lowercase().contains("updater") || contains_updater(value)
        }),
        Value::Array(values) => values.iter().any(contains_updater),
        Value::String(value) => value.to_ascii_lowercase().contains("updater"),
        _ => false,
    }
}

fn value_at<'a>(config: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter().try_fold(config, |value, key| value.get(key))
}

#[cfg(test)]
mod tests;
