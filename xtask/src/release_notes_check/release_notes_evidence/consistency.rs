pub(super) fn validate(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let version = field_value("Version", text);
    require_version_in_url("Artifact URL", version, text, &mut errors);
    require_version_in_url("GitHub Release URL", version, text, &mut errors);
    require_artifact_name(text, &mut errors);
    errors
}

fn require_version_in_url(
    label: &'static str,
    version: Option<&str>,
    text: &str,
    errors: &mut Vec<String>,
) {
    let (Some(version), Some(url)) = (version, field_value(label, text)) else {
        return;
    };
    let version = version.strip_prefix('v').unwrap_or(version);
    if url.contains(&format!("/v{version}/")) || url.ends_with(&format!("/v{version}")) {
        return;
    }
    errors.push(format!("{label} must match Version"));
}

fn require_artifact_name(text: &str, errors: &mut Vec<String>) {
    let (Some(artifact), Some(url)) = (
        field_value("Artifact", text),
        field_value("Artifact URL", text),
    ) else {
        return;
    };
    if url.ends_with(&format!("/{artifact}")) {
        return;
    }
    errors.push("Artifact must match Artifact URL file name".to_string());
}

fn field_value<'a>(label: &str, text: &'a str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}
