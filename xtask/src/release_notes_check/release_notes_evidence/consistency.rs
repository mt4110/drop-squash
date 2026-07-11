pub(super) fn validate(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let version = field_value("Version", text);
    require_version_in_url("Artifact URL", version, text, &mut errors);
    require_version_in_url("GitHub Release URL", version, text, &mut errors);
    require_artifact_name(text, &mut errors);
    require_same_origin("Public website URL", "Refund policy URL", text, &mut errors);
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

fn require_same_origin(
    first_label: &'static str,
    second_label: &'static str,
    text: &str,
    errors: &mut Vec<String>,
) {
    let (Some(first), Some(second)) = (
        field_value(first_label, text),
        field_value(second_label, text),
    ) else {
        return;
    };
    if origin(first) == origin(second) {
        return;
    }
    errors.push(format!(
        "{second_label} must use the same origin as {first_label}"
    ));
}

fn origin(url: &str) -> Option<&str> {
    let without_scheme = url.strip_prefix("https://")?;
    Some(without_scheme.split('/').next().unwrap_or(without_scheme))
}

fn field_value<'a>(label: &str, text: &'a str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}
