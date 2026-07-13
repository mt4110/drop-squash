pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Ask source policy" => &[
            &["ask", "choose"],
            &["trash"],
            &["keep"],
            &["original"],
            &["remained", "unchanged"],
        ],
        "Trash source policy" => &[
            &["trash"],
            &["original"],
            &["moved to trash", "moves to trash"],
            &["moving original", "moving"],
            &["disabled"],
            &["verified", "smaller"],
        ],
        _ => return None,
    };
    Some(groups)
}
