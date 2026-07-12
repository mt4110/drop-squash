pub(super) fn lacks_required_evidence(label: &str, value: &str) -> bool {
    let Some(groups) = groups_for(label) else {
        return super::homebrew::lacks_required_evidence(label, value).unwrap_or(false);
    };
    let lower = value.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
}

fn groups_for(label: &str) -> Option<&'static [&'static [&'static str]]> {
    match label {
        "`codesign`" => Some(&[&["codesign"], &["developer id"], &["dropsquash.dmg"]]),
        "`spctl`" => Some(&[&["spctl"], &["accepted"], &["dropsquash.dmg"]]),
        "`stapler`" => Some(&[
            &["stapler", "staple"],
            &["stapled", "validate"],
            &["dropsquash.dmg"],
        ]),
        "Apple notary log" => Some(&[
            &["notary", "notarytool"],
            &["accepted"],
            &["dropsquash.dmg"],
        ]),
        "Gatekeeper clean-machine open" => Some(&[
            &["gatekeeper"],
            &["opened", "opens"],
            &["clean", "fresh"],
            &["signed"],
            &["notarized", "notarised"],
            &[
                "without warning",
                "no warning",
                "without gatekeeper warning",
            ],
        ]),
        "`docs/release-blockers.md` status" => {
            Some(&[&["docs/release-blockers.md"], &["verified"], &["all rows"]])
        }
        "Manual QA record" => Some(&[&["docs/manual-qa.md"], &["dropsquash.dmg", ".app"]]),
        "Conversion safety evidence" => Some(&[
            &["cancellation", "cancelled"],
            &["failed conversion", "failure"],
            &["larger output"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ]),
        "Queue evidence" => Some(&[
            &["multi-file", "queue"],
            &["queued cancellation", "queued row"],
            &["batch summary"],
            &["finished"],
            &["saved bytes"],
            &["failed", "cancelled", "blocked"],
        ]),
        "Trash source policy" => Some(&[
            &["moving original", "moving"],
            &["disabled"],
            &["verified"],
            &["smaller"],
            &["trash"],
        ]),
        "Lemon Squeezy product setup" => Some(&[
            &["dropsquash"],
            &["intended product"],
            &["license keys enabled"],
        ]),
        "Lemon Squeezy sandbox purchase" => {
            Some(&[&["intended product"], &["test buyer"], &["order"]])
        }
        "Valid sandbox activation" => Some(&[
            &["activating"],
            &["disabled"],
            &["pro"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
            &["cache"],
        ]),
        "Empty key activation" => Some(&[
            &["activate"],
            &["disabled"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
            &["cache"],
        ]),
        "Invalid license key handling" => Some(&[
            &["activating"],
            &["disabled"],
            &["friendly"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
            &["cache"],
        ]),
        "License network failure" => Some(&[
            &["friendly"],
            &["network"],
            &["existing"],
            &["valid"],
            &["preserved", "intact"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
            &["cache"],
        ]),
        "Local license forget" => Some(&[
            &["forgetting"],
            &["disabled"],
            &["cache"],
            &["removed", "cleared", "deleted", "clears"],
            &["trial", "locked"],
        ]),
        "GitHub Release checksum" => Some(&[
            &["sha256sums", "sha-256"],
            &["attached"],
            &["dropsquash.dmg"],
        ]),
        "Known limitations" => Some(&[&["macos"], &["windows", "linux", "platform"]]),
        "Support contact" => Some(&[&["support"], &["github issues", "@"]]),
        _ => None,
    }
}
