use std::path::Path;

const DISALLOWED_PHRASES: &[&str] = &[
    "download now",
    "download dropsquash",
    "download for macos",
    "download the beta",
    "buy now",
    "buy dropsquash",
    "purchase now",
    "start checkout",
    "checkout now",
    "get the beta",
    "available now",
];

pub(super) fn check(path: &Path, text: &str, errors: &mut Vec<String>) {
    let lower = text.to_ascii_lowercase();
    for phrase in DISALLOWED_PHRASES {
        if lower.contains(phrase) {
            errors.push(format!(
                "{} contains pre-release CTA copy: {phrase}",
                path.display()
            ));
        }
    }
}
