pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Choose recording conversion" | "Drag-and-drop conversion" => {
            &[&[".squashed.mp4"], &["smaller"], &["original"]]
        }
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
        "Duplicate output naming" => &[&["second output"], &["squashed-2"], &[".mp4"]],
        "Cancellation" => &[
            &["ready"],
            &["trial"],
            &["history"],
            &["no new success", "no success"],
        ],
        "Multi-file queue" => &[
            &["three", "3"],
            &["one active", "1 active"],
            &["sequential"],
            &["completed", "finished"],
            &["unrelated failure", "unrelated failures"],
            &["did not block", "do not block"],
        ],
        "Queued job cancellation" => &[
            &["cancelled"],
            &["never starts", "never started"],
            &["trial"],
            &["history"],
            &["no new success", "no success"],
        ],
        "Batch summary" => &[
            &["finished"],
            &["saved bytes"],
            &["failed"],
            &["cancelled"],
            &["blocked"],
            &["trial lock", "license lock"],
        ],
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
        "Failed conversion" => &[
            &["friendly", "error"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ],
        "Larger output" => &[
            &["larger"],
            &["not smaller", "cannot be made smaller"],
            &["failure", "failed"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ],
        "Reveal output" => &[&["finder"], &[".squashed.mp4"], &["selected"]],
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
        "Codesign verification" => &[
            &["codesign"],
            &["developer id"],
            &["public"],
            &["dropsquash.dmg"],
        ],
        "Notarization staple verification" => &[
            &["notary", "notarization"],
            &["stapler", "staple"],
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
