use std::path::Path;

const FORBIDDEN: &[&str] = &[
    "download for windows",
    "download for linux",
    "download the windows",
    "download the linux",
    "download windows",
    "download linux",
    "download on windows",
    "download on linux",
    "get windows",
    "get linux",
    "get the windows",
    "get the linux",
    "windows download",
    "linux download",
    "windows installer",
    "linux installer",
    "windows package",
    "linux package",
    "windows beta",
    "linux beta",
    "windows app available",
    "linux app available",
    "windows available",
    "linux available",
    "windows build is available",
    "linux build is available",
    "windows version available",
    "linux version available",
    "windows release available",
    "linux release available",
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
