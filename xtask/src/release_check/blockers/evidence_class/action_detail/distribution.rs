pub(super) fn for_blocker(blocker: &str) -> Option<&'static [&'static str]> {
    match blocker {
        "Signed DMG" => Some(&[
            "macos-signing-check",
            "public",
            "DropSquash.dmg",
            "Artifact URL",
            "`codesign`",
            "Developer ID",
        ]),
        "Notarized and stapled DMG" => Some(&[
            "macos-signing-check",
            "public",
            "DropSquash.dmg",
            "Artifact URL",
            "`spctl`",
            "notary",
            "stapled",
            "stapler",
        ]),
        "Published checksum" => Some(&[
            "SHA256SUMS",
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
            "brew uninstall",
            "removes it cleanly",
            "auto_updates false",
            "zap",
        ]),
        _ => None,
    }
}
