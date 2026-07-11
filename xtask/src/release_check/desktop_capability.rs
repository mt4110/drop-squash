use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

const REQUIRED_PERMISSIONS: [&str; 3] = [
    "core:event:default",
    "dialog:allow-open",
    "opener:allow-reveal-item-in-dir",
];
const DISALLOWED_PERMISSIONS: [&str; 6] = [
    "opener:allow-open-url",
    "opener:allow-default-urls",
    "shell:",
    "http:",
    "notification:",
    "clipboard:",
];

pub(super) fn check_default_capability(path: &Path) -> Result<(), String> {
    let permissions = read_permissions(path)?;
    reject_disallowed_permissions(&permissions)?;
    require_exact_permissions(&permissions)
}

fn read_permissions(path: &Path) -> Result<BTreeSet<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let value: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    let values = value
        .get("permissions")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{} is missing permissions array", path.display()))?;
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| "desktop capability permissions must be strings".to_string())
        })
        .collect()
}

fn require_exact_permissions(permissions: &BTreeSet<String>) -> Result<(), String> {
    let required = REQUIRED_PERMISSIONS
        .iter()
        .map(|value| value.to_string())
        .collect::<BTreeSet<_>>();
    if permissions == &required {
        return Ok(());
    }
    Err(format!(
        "desktop capability permissions changed; expected {:?}, got {:?}",
        REQUIRED_PERMISSIONS, permissions
    ))
}

fn reject_disallowed_permissions(permissions: &BTreeSet<String>) -> Result<(), String> {
    for permission in permissions {
        if DISALLOWED_PERMISSIONS
            .iter()
            .any(|prefix| permission.starts_with(prefix))
        {
            return Err(format!(
                "desktop capability contains disallowed permission: {permission}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
