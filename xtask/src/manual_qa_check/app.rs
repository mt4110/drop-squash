pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    let Some(requirement) = requirement_for(label.trim()) else {
        return;
    };
    let lower = result.to_ascii_lowercase();
    if requirement
        .groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        && checksum_evidence_ok(label, result)
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs concrete packaged-app evidence"
    ));
}

struct Requirement {
    groups: &'static [&'static [&'static str]],
}

fn checksum_evidence_ok(label: &str, result: &str) -> bool {
    if label != "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`" {
        return true;
    }
    result
        .split(|value: char| !value.is_ascii_hexdigit())
        .any(|part| part.len() == 64)
}

fn requirement_for(label: &str) -> Option<Requirement> {
    let groups: &[&[&str]] = match label {
        "Choose recording conversion" | "Drag-and-drop conversion" => {
            &[&[".squashed.mp4"], &["original"]]
        }
        "Privacy receipt sidecar" => &[
            &[".privacy.json"],
            &["uploaded_bytes"],
            &["metadata_policy"],
        ],
        "Reveal privacy receipt" => &[&["finder"], &[".privacy.json"]],
        "Duplicate output naming" => &[&["squashed-2", "numbered"]],
        "Cancellation" => &[&["ready"], &["trial", "history"]],
        "Multi-file queue" => &[&["three", "3"], &["one active", "sequential"]],
        "Queued job cancellation" => &[&["cancelled"], &["never starts", "never started"]],
        "Batch summary" => &[&["finished"], &["saved bytes"]],
        "Ask source policy" => &[&["ask", "choose"], &["trash", "keep"]],
        "Trash source policy" => &[&["trash"], &["verified", "smaller"]],
        "Failed conversion" => &[&["original"], &["trial count unchanged", "trial unchanged"]],
        "Larger output" => &[
            &["failure", "failed"],
            &["trial count unchanged", "trial unchanged"],
        ],
        "Reveal output" => &[&["finder"], &[".mp4"]],
        "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`" => {
            &[&["artifact-check"], &["dropsquash.dmg", ".dmg"]]
        }
        "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`" => {
            &[&["sha-256", "sha256"], &["dropsquash.dmg", ".dmg"]]
        }
        "`cargo run -p xtask -- macos-signing-check`" => &[&["macos-signing-check"]],
        "Codesign verification" => &[&["codesign"], &["developer id"]],
        "Notarization staple verification" => &[&["notary", "notarization"], &["staple", "spctl"]],
        "Gatekeeper open test" => &[&["gatekeeper"], &["opened", "opens"], &["clean", "fresh"]],
        _ => return None,
    };
    Some(Requirement { groups })
}
