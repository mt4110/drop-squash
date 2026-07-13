use std::path::Path;

const FORBIDDEN: &[&str] = &[
    "download for windows",
    "download for linux",
    "download windows",
    "download linux",
    "download on windows",
    "download on linux",
    "windows download",
    "linux download",
    "windows installer",
    "linux installer",
    "windows beta",
    "linux beta",
    "windows available",
    "linux available",
    "windows build is available",
    "linux build is available",
    "flatpak is available",
    "flatpak download",
    "available on windows",
    "available on linux",
    "available for windows",
    "available for linux",
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
