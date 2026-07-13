pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Choose recording conversion" | "Drag-and-drop conversion" => &[
            &[".squashed.mp4"],
            &["smaller"],
            &[
                "original remained",
                "original remains",
                "original unchanged",
                "preserved original",
            ],
        ],
        "Duplicate output naming" => &[&["second output"], &["squashed-2"], &[".mp4"]],
        "Failed conversion" => &[
            &["friendly", "error"],
            &[
                "original remained",
                "original remains",
                "original unchanged",
                "preserved original",
            ],
            &["trial count unchanged", "trial unchanged"],
        ],
        "Larger output" => &[
            &["larger"],
            &["not smaller", "cannot be made smaller"],
            &["failure", "failed"],
            &[
                "original remained",
                "original remains",
                "original unchanged",
                "preserved original",
            ],
            &["trial count unchanged", "trial unchanged"],
        ],
        "Reveal output" => &[&["finder"], &[".squashed.mp4"], &["selected"]],
        _ => return None,
    };
    Some(groups)
}
