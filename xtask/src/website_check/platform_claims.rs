use std::path::Path;

const FORBIDDEN: [&str; 12] = [
    "Download for Windows",
    "Download for Linux",
    "Windows download",
    "Linux download",
    "Windows installer",
    "Linux installer",
    "Windows beta",
    "Linux beta",
    "Windows build is available",
    "Linux build is available",
    "Flatpak is available",
    "Available on Windows",
];

pub(super) fn check(path: &Path, text: &str, errors: &mut Vec<String>) {
    for phrase in FORBIDDEN {
        if text.contains(phrase) {
            errors.push(format!(
                "{} claims an unsupported platform is available: {phrase}",
                path.display()
            ));
        }
    }
}
