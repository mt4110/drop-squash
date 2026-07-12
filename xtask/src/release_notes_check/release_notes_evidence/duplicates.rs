use super::fields;

pub(super) fn validate(text: &str) -> Vec<String> {
    release_note_labels()
        .into_iter()
        .filter(|label| field_count(label, text) > 1)
        .map(|label| format!("{label} must appear only once"))
        .collect()
}

fn release_note_labels() -> Vec<&'static str> {
    let mut labels = vec![
        "Version",
        "Artifact",
        "SHA-256",
        "Git commit",
        "Benchmark sample set",
        "Benchmark regression threshold",
    ];
    labels.extend(fields::URL.iter().map(|(label, _)| *label));
    labels.extend(fields::EVIDENCE);
    labels
}

fn field_count(label: &str, text: &str) -> usize {
    let prefix = format!("- {label}:");
    text.lines()
        .filter(|line| line.trim().starts_with(&prefix))
        .count()
}
