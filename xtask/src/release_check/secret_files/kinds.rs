const SECRET_EXTENSIONS: [&str; 9] = [
    "cer",
    "cert",
    "crt",
    "key",
    "mobileprovision",
    "p12",
    "p8",
    "pem",
    "provisionprofile",
];

const LOCAL_EVIDENCE_EXTENSIONS: [&str; 19] = [
    "app",
    "appimage",
    "csv",
    "dmg",
    "dsym",
    "exe",
    "flatpak",
    "gz",
    "jsonl",
    "m4v",
    "mov",
    "mp4",
    "msi",
    "pkg",
    "tar",
    "tgz",
    "webm",
    "xcarchive",
    "zip",
];

pub(super) fn is_secret_file(name: &str, extension: Option<&str>) -> bool {
    name == ".env"
        || name == ".envrc"
        || name.starts_with(".env.")
        || has_extension(extension, &SECRET_EXTENSIONS)
}

pub(super) fn is_local_evidence_file(name: &str, extension: Option<&str>) -> bool {
    name == "SHA256SUMS" || has_extension(extension, &LOCAL_EVIDENCE_EXTENSIONS)
}

fn has_extension(extension: Option<&str>, denied: &[&str]) -> bool {
    extension.is_some_and(|value| {
        let lower = value.to_ascii_lowercase();
        denied.contains(&lower.as_str())
    })
}
