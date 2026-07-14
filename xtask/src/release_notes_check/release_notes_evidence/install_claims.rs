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

pub(super) fn validate(text: &str) -> Vec<String> {
    let lower = text.to_ascii_lowercase();
    DISALLOWED_PHRASES
        .iter()
        .filter(|phrase| lower.contains(**phrase))
        .map(|phrase| format!("release notes contain unsupported DMG cleanup claim: {phrase}"))
        .collect()
}
