mod distribution;
mod macos_verification;

pub(super) struct Fields<'a> {
    pub(super) version: &'a str,
    pub(super) artifact_path: &'a str,
    pub(super) artifact_url: &'a str,
    pub(super) sha256: &'a str,
    pub(super) commit: &'a str,
}

pub(super) fn lines(fields: Fields<'_>) -> Vec<String> {
    let release_url = format!(
        "https://github.com/mt4110/drop-squash/releases/tag/v{}",
        fields.version
    );
    let mut lines = vec![
        "release notes prepared fields:".into(),
        "## Artifact".into(),
        format!("- Version: v{}", fields.version),
        "- Artifact: DropSquash.dmg".into(),
        format!("- Artifact URL: {}", fields.artifact_url),
        format!("- SHA-256: {}", fields.sha256),
        format!("- Git commit: {}", fields.commit),
    ];
    lines.extend(macos_verification::lines(
        fields.artifact_path,
        fields.artifact_url,
    ));
    lines.extend([
        "## Productization Evidence".into(),
        "- Public website URL: pending production deployment; replace with https://dropsquash.app/release-status after that page links release-status, privacy, pricing, terms, license, support, and download".into(),
        "- Pricing URL: pending final pricing; replace with https://dropsquash.app/pricing after draft price copy is removed".into(),
        "- Refund policy URL: pending final refund policy; replace with https://dropsquash.app/refund".into(),
        "- Live checkout URL: pending live checkout; replace with the tested https://store.lemonsqueezy.com/checkout/buy/<id> URL".into(),
    ]);
    lines.extend(distribution::lines(distribution::Fields {
        version: fields.version,
        artifact_path: fields.artifact_path,
        artifact_url: fields.artifact_url,
        sha256: fields.sha256,
        release_url: &release_url,
    }));
    lines
}

fn shell_arg(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-".contains(ch))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::{lines, Fields};

    #[test]
    fn generated_field_labels_exist_in_release_notes_template() {
        let template = std::fs::read_to_string("../docs/release-notes-template.md").unwrap();
        let generated = lines(Fields {
            version: "0.1.0",
            artifact_path: "/tmp/DropSquash.dmg",
            artifact_url:
                "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg",
            sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            commit: "abc1234",
        });
        let untracked = generated
            .iter()
            .filter_map(|line| field_label(line.as_str()))
            .filter(|label| *label != "SHA256SUMS line")
            .filter(|label| !template.contains(&format!("- {label}:")))
            .collect::<Vec<_>>();

        assert!(untracked.is_empty(), "{untracked:?}");
    }

    #[test]
    fn public_website_draft_keeps_public_web_proof_copy() {
        let template = std::fs::read_to_string("../docs/release-notes-template.md").unwrap();
        let generated = lines(Fields {
            version: "0.1.0",
            artifact_path: "/tmp/DropSquash.dmg",
            artifact_url:
                "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg",
            sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            commit: "abc1234",
        });
        let expected =
            "after that page links release-status, privacy, pricing, terms, license, support, and download";

        assert!(template.contains(expected));
        assert!(generated.iter().any(|line| line.contains(expected)));
    }

    fn field_label(line: &str) -> Option<&str> {
        line.strip_prefix("- ")?
            .split_once(':')
            .map(|(label, _)| label)
    }
}
