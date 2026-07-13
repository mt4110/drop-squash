pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Expired license refresh" => &[
            &["expired"],
            &["offline grace"],
            &["reconnect"],
            &["blocked"],
            &["before starting"],
            &["cache", "license.json"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
        ],
        _ => return None,
    };
    Some(groups)
}
