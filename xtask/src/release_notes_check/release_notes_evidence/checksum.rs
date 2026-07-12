use super::value;

pub(super) fn validate(text: &str, errors: &mut Vec<String>) {
    require_in_evidence(text, "SHA-256", "the SHA-256 digest", errors);
    require_in_evidence(text, "Artifact URL", "the Artifact URL", errors);
    require_in_evidence(text, "GitHub Release URL", "the GitHub Release URL", errors);
}

fn require_in_evidence(
    text: &str,
    field_label: &'static str,
    error_label: &'static str,
    errors: &mut Vec<String>,
) {
    let (Some(expected), Some(evidence)) = (
        value::field(field_label, text),
        value::field("GitHub Release checksum", text),
    ) else {
        return;
    };
    if evidence.contains(expected) {
        return;
    }
    errors.push(format!(
        "GitHub Release checksum must include {error_label}"
    ));
}
