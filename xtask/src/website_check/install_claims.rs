use std::path::Path;

pub(super) fn check(path: &Path, text: &str, errors: &mut Vec<String>) {
    for phrase in crate::dmg_cleanup_claims::matches(text) {
        errors.push(format!(
            "{} contains unsupported DMG cleanup claim: {phrase}",
            path.display()
        ));
    }
}
