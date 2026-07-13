use super::value;

const FIELDS: [&str; 5] = [
    "`codesign`",
    "`spctl`",
    "`stapler`",
    "Apple notary log",
    "Gatekeeper clean-machine open",
];

pub(super) fn validate(text: &str, errors: &mut Vec<String>) {
    let Some(artifact_url) = value::field("Artifact URL", text) else {
        return;
    };
    for field in FIELDS {
        require_artifact_url(field, artifact_url, text, errors);
    }
}

fn require_artifact_url(
    field: &'static str,
    artifact_url: &str,
    text: &str,
    errors: &mut Vec<String>,
) {
    let Some(evidence) = value::field(field, text) else {
        return;
    };
    if evidence.contains(artifact_url) {
        return;
    }
    errors.push(format!("{field} must include the Artifact URL"));
}
