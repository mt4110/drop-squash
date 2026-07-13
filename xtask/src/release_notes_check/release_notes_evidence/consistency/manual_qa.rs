use super::value;

pub(super) fn validate(text: &str, errors: &mut Vec<String>) {
    let (Some(artifact_url), Some(record)) = (
        value::field("Artifact URL", text),
        value::field("Manual QA record", text),
    ) else {
        return;
    };
    if record.contains(artifact_url) {
        return;
    }
    errors.push("Manual QA record must include the Artifact URL".to_string());
}
