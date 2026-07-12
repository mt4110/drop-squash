use super::Groups;

pub(super) fn for_label(label: &str) -> Option<Groups> {
    match label {
        "Manual QA record" => Some(&[&["docs/manual-qa.md"], &["dropsquash.dmg"]]),
        "Conversion safety evidence" => Some(&[
            &["cancellation", "cancelled"],
            &["failed conversion", "failure"],
            &["larger output"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ]),
        "Queue evidence" => Some(&[
            &["multi-file", "queue"],
            &["queued cancellation", "queued row"],
            &["batch summary"],
            &["finished"],
            &["saved bytes"],
            &["failed", "cancelled", "blocked"],
        ]),
        "Trash source policy" => Some(&[
            &["moving original", "moving"],
            &["disabled"],
            &["verified"],
            &["smaller"],
            &["trash"],
        ]),
        _ => None,
    }
}
