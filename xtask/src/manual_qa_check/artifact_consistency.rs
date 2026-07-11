use std::path::Path;

const ARTIFACT_CHECK: &str = "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`";
const CHECKSUM: &str = "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`";

pub(super) fn validate(rows: &[(String, String)], missing: &mut Vec<String>) {
    let Some(name) = app_artifact_name(rows) else {
        return;
    };
    if !name.to_ascii_lowercase().ends_with(".dmg") {
        return;
    }
    for label in [ARTIFACT_CHECK, CHECKSUM] {
        if let Some(result) = value_for(rows, label) {
            require_same_artifact(label, result, &name, missing);
        }
    }
}

fn app_artifact_name(rows: &[(String, String)]) -> Option<String> {
    let value = value_for(rows, "App artifact")?;
    Path::new(value)
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

fn require_same_artifact(label: &str, result: &str, name: &str, missing: &mut Vec<String>) {
    if result
        .to_ascii_lowercase()
        .contains(&name.to_ascii_lowercase())
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} must reference App artifact {name}"
    ));
}

fn value_for<'a>(rows: &'a [(String, String)], label: &str) -> Option<&'a str> {
    rows.iter()
        .find(|(row_label, _)| row_label == label)
        .map(|(_, value)| value.as_str())
}
