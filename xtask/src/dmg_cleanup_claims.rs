const DISALLOWED_PHRASES: &[&str] = &[
    "automatically deletes the downloaded dmg",
    "automatically deletes the downloaded .dmg",
    "automatically deletes dropsquash.dmg",
    "auto-deletes the downloaded dmg",
    "auto-deletes the downloaded .dmg",
    "auto-deletes dropsquash.dmg",
    "copy and paste automatically deletes the downloaded dmg",
    "copy and paste automatically deletes the downloaded .dmg",
    "copy/paste automatically deletes the downloaded dmg",
    "copy/paste automatically deletes the downloaded .dmg",
    "deletes the dmg after install",
    "deletes the .dmg after install",
    "drag to applications automatically deletes the downloaded dmg",
    "drag to applications automatically deletes the downloaded .dmg",
    "finder copy automatically deletes the downloaded dmg",
    "finder copy automatically deletes the downloaded .dmg",
];

pub(crate) fn matches(text: &str) -> Vec<&'static str> {
    let lower = text.to_ascii_lowercase();
    DISALLOWED_PHRASES
        .iter()
        .copied()
        .filter(|phrase| lower.contains(phrase))
        .collect()
}
