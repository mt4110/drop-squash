use super::value;

pub(super) fn validate(text: &str) -> Vec<String> {
    [
        validate_version(text),
        validate_artifact(text),
        validate_git_commit(text),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn validate_version(text: &str) -> Option<String> {
    let Some(value) = value::field("Version", text) else {
        return Some("Version must be present".to_string());
    };
    if is_semver(value.strip_prefix('v').unwrap_or(value)) {
        return None;
    }
    Some("Version must be a concrete semver version".to_string())
}

fn validate_artifact(text: &str) -> Option<String> {
    let Some(value) = value::field("Artifact", text) else {
        return Some("Artifact must be present".to_string());
    };
    if value == "DropSquash.dmg" {
        return None;
    }
    Some("Artifact must name the DropSquash DMG".to_string())
}

fn validate_git_commit(text: &str) -> Option<String> {
    let Some(value) = value::field("Git commit", text) else {
        return Some("Git commit must be present".to_string());
    };
    if (7..=40).contains(&value.len()) && value.chars().all(|value| value.is_ascii_hexdigit()) {
        return None;
    }
    Some("Git commit must be a concrete commit hash".to_string())
}

fn is_semver(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|value| value.is_ascii_digit()))
}
