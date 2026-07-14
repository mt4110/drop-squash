pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Disk image launch notice" => &[
            &["disk image"],
            &["/volumes", "mounted"],
            &["applications"],
            &["warn", "notice"],
            &["move", "copy", "copied"],
            &["without replacing", "no replace", "did not replace"],
            &["finder"],
            &["post-copy", "copied app", "after copy"],
            &["open", "opened"],
            &["installed app", "applications copy"],
            &["eject", "ejected"],
            &["quit"],
            &["disk image copy", "mounted copy"],
            &[
                "does not delete",
                "did not delete",
                "no downloaded .dmg deletion",
            ],
        ],
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
