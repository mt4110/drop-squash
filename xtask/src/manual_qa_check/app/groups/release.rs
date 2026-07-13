pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Codesign verification" => &[
            &["codesign"],
            &["developer id"],
            &["public"],
            &["dropsquash.dmg"],
        ],
        "Notarization staple verification" => &[
            &["notary", "notarization"],
            &["stapler", "staple"],
            &["stapled", "validate"],
            &["spctl"],
            &["public"],
            &["dropsquash.dmg"],
        ],
        "Gatekeeper open test" => &[
            &["gatekeeper"],
            &["opened", "opens"],
            &["clean", "fresh"],
            &["public"],
            &["dropsquash.dmg"],
            &["signed"],
            &["notarized", "notarised"],
            &["stapled", "staple"],
            &[
                "without warning",
                "no warning",
                "without gatekeeper warning",
            ],
        ],
        _ => return None,
    };
    Some(groups)
}
