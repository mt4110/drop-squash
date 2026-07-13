pub(super) fn lacks_required_evidence(label: &str, value: &str) -> Option<bool> {
    let groups = match label {
        "Homebrew tap PR" => &[
            &["cask"][..],
            &["pr"],
            &["public"],
            &["versioned"],
            &["zap"],
            &["dropsquash.dmg"],
            &["auto_updates false"],
        ][..],
        "Homebrew install result" => &[
            &["brew install"][..],
            &["--cask"],
            &["mt4110/tap/dropsquash"],
            &["versioned"],
            &["dropsquash.dmg"],
            &["sha-256", "sha256"],
            &["brew uninstall"],
            &["--cask"],
            &[
                "removed cleanly",
                "removed it cleanly",
                "removes it cleanly",
            ],
        ][..],
        _ => return None,
    };
    let lower = value.to_ascii_lowercase();
    Some(
        !groups
            .iter()
            .all(|group| group.iter().any(|needle| lower.contains(needle))),
    )
}
