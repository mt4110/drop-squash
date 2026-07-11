pub(super) fn lacks_required_evidence(label: &str, value: &str) -> Option<bool> {
    let groups = match label {
        "Homebrew tap PR" => &[
            &["cask"][..],
            &["pr"],
            &["zap"],
            &["dropsquash.dmg"],
            &["auto_updates false"],
        ][..],
        "Homebrew install result" => &[
            &["brew install"][..],
            &["--cask"],
            &["mt4110/tap/dropsquash"],
            &["dropsquash.dmg"],
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
