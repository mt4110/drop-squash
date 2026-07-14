use std::path::Path;

const DISALLOWED_PHRASES: &[&str] = &[
    "automatically deletes the downloaded dmg",
    "automatically deletes the downloaded .dmg",
    "automatically deletes dropsquash.dmg",
    "auto-deletes the downloaded dmg",
    "auto-deletes the downloaded .dmg",
    "auto-deletes dropsquash.dmg",
    "deletes the dmg after install",
    "deletes the .dmg after install",
];

pub(super) fn check(path: &Path, text: &str, errors: &mut Vec<String>) {
    let lower = text.to_ascii_lowercase();
    for phrase in DISALLOWED_PHRASES {
        if lower.contains(phrase) {
            errors.push(format!(
                "{} contains unsupported DMG cleanup claim: {phrase}",
                path.display()
            ));
        }
    }
}
