pub(super) fn lacks_required_evidence(label: &str, value: &str) -> bool {
    let Some(groups) = groups_for(label) else {
        return false;
    };
    let lower = value.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
}

fn groups_for(label: &str) -> Option<&'static [&'static [&'static str]]> {
    match label {
        "`codesign`" => Some(&[&["codesign"], &["developer id"]]),
        "`spctl`" => Some(&[&["spctl"], &["accepted"]]),
        "`stapler`" => Some(&[&["stapler", "staple"], &["stapled", "validate"]]),
        "Apple notary log" => Some(&[&["notary", "notarytool"], &["accepted"]]),
        "Gatekeeper clean-machine open" => {
            Some(&[&["gatekeeper"], &["opened", "opens"], &["clean", "fresh"]])
        }
        "`docs/release-blockers.md` status" => Some(&[&["verified"], &["all rows"]]),
        "Manual QA record" => Some(&[&["docs/manual-qa.md"], &["dropsquash.dmg", ".app"]]),
        "Lemon Squeezy sandbox purchase" => Some(&[&["test buyer"], &["order"]]),
        "Lemon Squeezy sandbox activation" => Some(&[&["pro"], &["raw key"], &["cache"]]),
        "Empty key activation" => Some(&[&["friendly"], &["raw key"], &["cache"]]),
        "Invalid license key handling" => Some(&[&["friendly"], &["raw key"], &["cache"]]),
        "Local license forget" => Some(&[&["cache"], &["trial", "locked"]]),
        "GitHub Release checksum" => Some(&[
            &["sha256sums", "sha-256"],
            &["attached"],
            &["dropsquash.dmg"],
        ]),
        "Homebrew tap PR" => Some(&[&["cask"], &["pr"]]),
        "Homebrew install result" => {
            Some(&[&["brew install"], &["--cask"], &["mt4110/tap/dropsquash"]])
        }
        _ => None,
    }
}
