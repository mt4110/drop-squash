pub(super) fn for_label(label: &str) -> Option<&'static str> {
    match label {
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`"
        | "Benchmark sample set" => Some(
            "note: if benchmark fails with `output is not smaller`, use that clip for the packaged-app Larger output row, verify the kept-original result under the current shipping profile and size, then rerun the release set with another real recording or the exact shipping size setting under test",
        ),
        "Sandbox product setup" => Some(
            "note: confirm the intended product is DropSquash, sandbox mode is active, license keys are enabled, and no private store IDs are pasted into the repository",
        ),
        "Disk image launch notice" => Some("note: requires launch from mounted DropSquash.dmg"),
        "Sandbox purchase" => Some(
            "note: record the sandbox test buyer order id or order number, but do not paste the key or private store IDs",
        ),
        "Valid sandbox activation" => Some(
            "note: record fingerprint and instance_id evidence, but do not paste the raw key",
        ),
        "License network failure" | "Expired license refresh" | "Forget license on this Mac" => {
            Some("note: check the cache outcome and observed app state without pasting the raw key")
        }
        "Failed conversion" => Some(
            "note: create a throwaway invalid .mp4 with `cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-manual-qa-invalid.mp4`, then use it outside the benchmark sample set",
        ),
        "`cargo run -p dropsquash -- license status`" => {
            Some("note: record diagnostic lines without pasting the sandbox key")
        }
        "`cargo run -p xtask -- macos-signing-check`" => {
            Some("note: requires release signing environment variables and secrets")
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::for_label;

    #[test]
    fn reports_note_for_disk_image_launch() {
        assert_eq!(
            for_label("Disk image launch notice"),
            Some("note: requires launch from mounted DropSquash.dmg")
        );
    }

    #[test]
    fn reports_note_for_sandbox_product_setup() {
        assert_eq!(
            for_label("Sandbox product setup"),
            Some(
                "note: confirm the intended product is DropSquash, sandbox mode is active, license keys are enabled, and no private store IDs are pasted into the repository"
            )
        );
    }

    #[test]
    fn reports_note_for_benchmark_not_smaller_failures() {
        assert_eq!(
            for_label("Benchmark sample set"),
            Some("note: if benchmark fails with `output is not smaller`, use that clip for the packaged-app Larger output row, verify the kept-original result under the current shipping profile and size, then rerun the release set with another real recording or the exact shipping size setting under test")
        );
    }

    #[test]
    fn reports_notes_for_license_rows() {
        assert_eq!(
            for_label("Sandbox purchase"),
            Some("note: record the sandbox test buyer order id or order number, but do not paste the key or private store IDs")
        );
        assert_eq!(
            for_label("Valid sandbox activation"),
            Some("note: record fingerprint and instance_id evidence, but do not paste the raw key")
        );
    }
}
