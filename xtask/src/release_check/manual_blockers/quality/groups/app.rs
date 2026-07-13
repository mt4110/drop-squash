use super::Groups;

pub(super) fn for_check(check: &str) -> Option<Groups> {
    match check {
        "Choose recording conversion" | "Drag-and-drop conversion" => {
            Some(&[&[".squashed.mp4"], &["smaller"], &["original"]])
        }
        "Privacy receipt sidecar" => Some(&[
            &[".privacy.json"],
            &["uploaded_bytes"],
            &["metadata_policy"],
            &["file names"],
            &[
                "instead of absolute paths",
                "no absolute paths",
                "without absolute paths",
            ],
        ]),
        "Reveal privacy receipt" => Some(&[&["finder"], &[".privacy.json"], &["selected"]]),
        "Duplicate output naming" => Some(&[&["second output"], &["squashed-2"], &[".mp4"]]),
        "Cancellation" => Some(&[
            &["ready"],
            &["trial"],
            &["history"],
            &["no new success", "no success"],
        ]),
        "Multi-file queue" => Some(&[
            &["three", "3"],
            &["one active", "1 active"],
            &["sequential"],
            &["completed", "finished"],
            &["unrelated failure", "unrelated failures"],
            &["did not block", "do not block"],
        ]),
        "Queued job cancellation" => Some(&[
            &["cancelled"],
            &["never starts", "never started"],
            &["trial"],
            &["history"],
            &["no new success", "no success"],
        ]),
        "Batch summary" => Some(&[
            &["finished"],
            &["saved bytes"],
            &["failed"],
            &["cancelled"],
            &["blocked"],
            &["trial lock", "license lock"],
        ]),
        "Ask source policy" => Some(&[
            &["ask", "choose"],
            &["trash"],
            &["keep"],
            &["original"],
            &["remained", "unchanged"],
        ]),
        "Trash source policy" => Some(&[
            &["trash"],
            &["original"],
            &["moved to trash", "moves to trash"],
            &["moving original", "moving"],
            &["disabled"],
            &["verified", "smaller"],
        ]),
        "Failed conversion" => Some(&[
            &["friendly", "error"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ]),
        "Larger output" => Some(&[
            &["larger"],
            &["failure", "failed"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ]),
        "Reveal output" => Some(&[&["finder"], &[".squashed.mp4"], &["selected"]]),
        _ => None,
    }
}
