pub(super) fn for_label(label: &str) -> Option<&'static str> {
    match label {
        "Disk image launch notice" => Some("Mounted DMG"),
        "Choose recording conversion" | "Drag-and-drop conversion"
        | "Privacy receipt sidecar" | "Reveal privacy receipt" | "Ask source policy"
        | "Trash source policy" | "Reveal output" => Some("Small Sample"),
        "Duplicate output naming" => Some("Duplicate Sample"),
        "Cancellation" | "Larger output" => Some("Large Sample"),
        "Multi-file queue" | "Queued job cancellation" | "Batch summary" => Some("Queue Sample"),
        "Failed conversion" => Some("Custom Failure Input"),
        "Sandbox product setup" | "Sandbox purchase" => Some("Setup"),
        "Empty key activation" | "Invalid key activation" => Some("Activation Safety"),
        "Valid sandbox activation" => Some("Valid Activation"),
        "License network failure" | "Expired license refresh"
        | "Forget license on this Mac" => Some("Failure Recovery"),
        "`cargo run -p dropsquash -- license status`" => Some("Local Diagnostics"),
        "`cargo run -p xtask -- manual-qa-check`" => Some("Final QA Gate"),
        "`cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md`" => {
            Some("Homebrew")
        }
        "`cargo run -p xtask -- macos-signing-check`" => Some("Signing Environment"),
        "Codesign verification" | "Notarization staple verification" => {
            Some("Signature Verification")
        }
        "Gatekeeper open test" => Some("Gatekeeper"),
        _ => None,
    }
}

pub(super) fn counts(rows: &[(String, String)]) -> Vec<(&'static str, usize)> {
    let mut counts = Vec::new();
    for (label, _) in rows {
        let Some(phase) = for_label(label) else {
            continue;
        };
        if let Some((_, count)) = counts.iter_mut().find(|(name, _)| *name == phase) {
            *count += 1;
        } else {
            counts.push((phase, 1));
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::{counts, for_label};

    #[test]
    fn maps_queue_rows_to_queue_phase() {
        assert_eq!(for_label("Batch summary"), Some("Queue Sample"));
    }

    #[test]
    fn counts_rows_per_phase_in_order() {
        let counts = counts(&[
            ("Choose recording conversion".to_string(), String::new()),
            ("Drag-and-drop conversion".to_string(), String::new()),
            ("Cancellation".to_string(), String::new()),
            ("Failed conversion".to_string(), String::new()),
        ]);

        assert_eq!(
            counts,
            vec![("Small Sample", 2), ("Large Sample", 1), ("Custom Failure Input", 1)]
        );
    }

    #[test]
    fn maps_license_rows_to_license_phases() {
        assert_eq!(for_label("Sandbox purchase"), Some("Setup"));
        assert_eq!(for_label("Valid sandbox activation"), Some("Valid Activation"));
    }

    #[test]
    fn maps_distribution_rows_to_distribution_phases() {
        assert_eq!(for_label("Codesign verification"), Some("Signature Verification"));
        assert_eq!(for_label("Gatekeeper open test"), Some("Gatekeeper"));
    }
}
