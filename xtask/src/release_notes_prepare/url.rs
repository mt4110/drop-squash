pub(super) fn validate(value: &str, version: &str) -> Result<(), String> {
    let prefix = "https://github.com/mt4110/drop-squash/releases/download/";
    let Some(path) = value.strip_prefix(prefix) else {
        return Err("Artifact URL must be the public GitHub Release DropSquash.dmg URL".into());
    };
    if valid_asset_path(path) && path.starts_with(&format!("v{version}/")) {
        return Ok(());
    }
    Err("Artifact URL must match the release version and DropSquash.dmg".into())
}

fn valid_asset_path(value: &str) -> bool {
    let parts = value.split('/').collect::<Vec<_>>();
    parts.len() == 2 && !parts[0].is_empty() && parts[1] == "DropSquash.dmg"
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn accepts_matching_versioned_artifact_url() {
        validate(
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg",
            "0.1.0",
        )
        .unwrap();
    }

    #[test]
    fn rejects_nested_artifact_url() {
        let error = validate(
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/nested/DropSquash.dmg",
            "0.1.0",
        )
        .unwrap_err();

        assert!(error.contains("Artifact URL"));
    }

    #[test]
    fn rejects_different_version_artifact_url() {
        let error = validate(
            "https://github.com/mt4110/drop-squash/releases/download/v0.2.0/DropSquash.dmg",
            "0.1.0",
        )
        .unwrap_err();

        assert!(error.contains("release version"));
    }
}
