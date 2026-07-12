use super::Groups;

pub(super) fn for_check(check: &str) -> Option<Groups> {
    match check {
        "Choose recording conversion" | "Drag-and-drop conversion" => {
            Some(&[&[".squashed.mp4"], &["original"]])
        }
        "Privacy receipt sidecar" => Some(&[
            &[".privacy.json"],
            &["uploaded_bytes"],
            &["metadata_policy"],
        ]),
        "Reveal privacy receipt" => Some(&[&["finder"], &[".privacy.json"], &["selected"]]),
        "Duplicate output naming" => Some(&[&["squashed-2"], &[".mp4"]]),
        "Cancellation" => Some(&[&["ready"], &["trial", "history"]]),
        "Multi-file queue" => Some(&[&["three", "3"], &["one active", "sequential"]]),
        "Queued job cancellation" => Some(&[&["cancelled"], &["never starts", "never started"]]),
        "Batch summary" => Some(&[
            &["finished"],
            &["saved bytes"],
            &["failed", "cancelled", "blocked"],
        ]),
        "Ask source policy" => Some(&[&["ask", "choose"], &["trash", "keep"]]),
        "Trash source policy" => Some(&[
            &["trash"],
            &["moving original", "moving"],
            &["disabled"],
            &["verified", "smaller"],
        ]),
        "Failed conversion" => {
            Some(&[&["original"], &["trial count unchanged", "trial unchanged"]])
        }
        "Larger output" => Some(&[
            &["failure", "failed"],
            &["trial count unchanged", "trial unchanged"],
        ]),
        "Reveal output" => Some(&[&["finder"], &[".mp4"], &["selected"]]),
        _ => None,
    }
}
