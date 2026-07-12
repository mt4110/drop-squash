use super::Groups;

pub(super) fn for_label(label: &str) -> Option<Groups> {
    match label {
        "`codesign`" => Some(&[
            &["codesign"],
            &["developer id"],
            &["public"],
            &["dropsquash.dmg"],
        ]),
        "`spctl`" => Some(&[&["spctl"], &["accepted"], &["public"], &["dropsquash.dmg"]]),
        "`stapler`" => Some(&[
            &["stapler", "staple"],
            &["stapled", "validate"],
            &["public"],
            &["dropsquash.dmg"],
        ]),
        "Apple notary log" => Some(&[
            &["notary", "notarytool"],
            &["accepted"],
            &["public"],
            &["dropsquash.dmg"],
        ]),
        "Gatekeeper clean-machine open" => Some(&[
            &["gatekeeper"],
            &["opened", "opens"],
            &["clean", "fresh"],
            &["signed"],
            &["notarized", "notarised"],
            &["stapled", "staple"],
            &[
                "without warning",
                "no warning",
                "without gatekeeper warning",
            ],
        ]),
        "`docs/release-blockers.md` status" => {
            Some(&[&["docs/release-blockers.md"], &["verified"], &["all rows"]])
        }
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
