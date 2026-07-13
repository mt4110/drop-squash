pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Cancellation" => &[
            &["ready"],
            &["temp cleanup", "temporary file cleanup", "cleaned temp"],
            &["trial count unchanged", "trial unchanged", "no trial count"],
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
            &["waiting row", "queued row"],
            &["cancelled"],
            &["never starts", "never started"],
            &["trial count unchanged", "trial unchanged", "no trial count"],
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
        _ => return None,
    };
    Some(groups)
}
