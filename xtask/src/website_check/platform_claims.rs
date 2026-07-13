use std::path::Path;

const FORBIDDEN: [&str; 12] = [
    "download for windows",
    "download for linux",
    "windows download",
    "linux download",
    "windows installer",
    "linux installer",
    "windows beta",
    "linux beta",
    "windows build is available",
    "linux build is available",
    "flatpak is available",
    "available on windows",
];

pub(super) fn check(path: &Path, text: &str, errors: &mut Vec<String>) {
    let lower = text.to_ascii_lowercase();
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            errors.push(format!(
                "{} claims an unsupported platform is available: {phrase}",
                path.display()
            ));
        }
    }
}
