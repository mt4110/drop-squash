pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Privacy receipt sidecar" => &[
            &[".privacy.json"],
            &["uploaded_bytes"],
            &["metadata_policy"],
            &["file names"],
            &[
                "instead of absolute paths",
                "no absolute paths",
                "without absolute paths",
            ],
        ],
        "Reveal privacy receipt" => &[&["finder"], &[".privacy.json"], &["selected"]],
        _ => return None,
    };
    Some(groups)
}
