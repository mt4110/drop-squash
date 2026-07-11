pub(super) fn lacks_required_evidence(check: &str, result: &str) -> bool {
    let Some(groups) = groups_for(check) else {
        return false;
    };
    let lower = result.to_ascii_lowercase();
    !groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
}

fn groups_for(check: &str) -> Option<&'static [&'static [&'static str]]> {
    match check {
        "Choose recording conversion" | "Drag-and-drop conversion" => {
            Some(&[&[".squashed.mp4"], &["original"]])
        }
        "Privacy receipt sidecar" => Some(&[
            &[".privacy.json"],
            &["uploaded_bytes"],
            &["metadata_policy"],
        ]),
        "Reveal privacy receipt" => Some(&[&["finder"], &[".privacy.json"]]),
        "Duplicate output naming" => Some(&[&["squashed-2", "numbered"]]),
        "Cancellation" => Some(&[&["ready"], &["trial", "history"]]),
        "Multi-file queue" => Some(&[&["three", "3"], &["one active", "sequential"]]),
        "Queued job cancellation" => Some(&[&["cancelled"], &["never starts", "never started"]]),
        "Batch summary" => Some(&[&["finished"], &["saved bytes"]]),
        "Ask source policy" => Some(&[&["ask", "choose"], &["trash", "keep"]]),
        "Trash source policy" => Some(&[&["trash"], &["verified", "smaller"]]),
        "Failed conversion" => {
            Some(&[&["original"], &["trial count unchanged", "trial unchanged"]])
        }
        "Larger output" => Some(&[
            &["failure", "failed"],
            &["trial count unchanged", "trial unchanged"],
        ]),
        "Reveal output" => Some(&[&["finder"], &[".mp4"]]),
        "Valid sandbox activation" => Some(&[&["cache", "license.json"], &["pro"], &["raw key"]]),
        "Invalid key activation" => {
            Some(&[&["cache", "license.json"], &["friendly"], &["raw key"]])
        }
        "Forget license on this Mac" => Some(&[&["cache", "license.json"], &["trial", "locked"]]),
        "Gatekeeper open test" => {
            Some(&[&["gatekeeper"], &["opened", "opens"], &["clean", "fresh"]])
        }
        _ => None,
    }
}
