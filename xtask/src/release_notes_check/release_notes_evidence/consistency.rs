use super::{checksum, value};

mod homebrew;

pub(super) fn validate(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let version = value::field("Version", text);
    require_version_in_url("Artifact URL", version, text, &mut errors);
    require_version_in_url("GitHub Release URL", version, text, &mut errors);
    require_artifact_name(text, &mut errors);
    require_same_origin("Public website URL", "Refund policy URL", text, &mut errors);
    checksum::validate(text, &mut errors);
    homebrew::validate(text, &mut errors);
    errors
}

fn require_version_in_url(
    label: &'static str,
    version: Option<&str>,
    text: &str,
    errors: &mut Vec<String>,
) {
    let (Some(version), Some(url)) = (version, value::field(label, text)) else {
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
        value::field("Artifact", text),
        value::field("Artifact URL", text),
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
        value::field(first_label, text),
        value::field(second_label, text),
    ) else {
        return;
    };
    if crate::url_origin::same(first, second) {
        return;
    }
    errors.push(format!(
        "{second_label} must use the same origin as {first_label}"
    ));
}
