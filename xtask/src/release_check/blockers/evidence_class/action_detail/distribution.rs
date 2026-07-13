pub(super) fn for_blocker(blocker: &str) -> Option<&'static [&'static str]> {
    match blocker {
        "Signed DMG" => Some(&[
            "public",
            "DropSquash.dmg",
            "Artifact URL",
            "`codesign`",
            "Developer ID",
        ]),
        "Notarized and stapled DMG" => Some(&[
            "public",
            "DropSquash.dmg",
            "Artifact URL",
            "`spctl`",
            "notary",
            "stapler",
        ]),
        "Published checksum" => Some(&[
            "public",
            "DropSquash.dmg",
            "Artifact URL",
            "lowercase SHA-256",
            "GitHub Release",
        ]),
        "Homebrew cask install" => Some(&[
            "brew install",
            "versioned",
            "DropSquash.dmg",
            "Artifact URL",
            "matching lowercase SHA-256",
            "auto_updates false",
            "zap",
        ]),
        _ => None,
    }
}
