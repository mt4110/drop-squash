pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
        "Choose recording conversion" | "Drag-and-drop conversion" => {
            &[&[".squashed.mp4"], &["original"]]
        }
        "Privacy receipt sidecar" => &[
            &[".privacy.json"],
            &["uploaded_bytes"],
            &["metadata_policy"],
        ],
        "Reveal privacy receipt" => &[&["finder"], &[".privacy.json"], &["selected"]],
        "Duplicate output naming" => &[&["squashed-2"], &[".mp4"]],
        "Cancellation" => &[
            &["ready"],
            &["trial"],
            &["history"],
            &["no new success", "no success"],
        ],
        "Multi-file queue" => &[&["three", "3"], &["one active", "sequential"]],
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
        ],
        "Ask source policy" => &[&["ask", "choose"], &["trash", "keep"]],
        "Trash source policy" => &[
            &["trash"],
            &["moving original", "moving"],
            &["disabled"],
            &["verified", "smaller"],
        ],
        "Failed conversion" => &[&["original"], &["trial count unchanged", "trial unchanged"]],
        "Larger output" => &[
            &["failure", "failed"],
            &["original"],
            &["trial count unchanged", "trial unchanged"],
        ],
        "Reveal output" => &[&["finder"], &[".mp4"], &["selected"]],
        "`cargo run -p xtask -- release-check`" => &[&["release-check"], &["passed", "passes"]],
        "`cargo run -p xtask -- file-size-check`" => {
            &[&["file-size-check"], &["passed", "passes"]]
        }
        "`cargo run -p xtask -- media-policy-check`" => {
            &[&["media-policy-check"], &["passed", "passes"]]
        }
        "`cargo run -p xtask -- privacy-policy-check`" => {
            &[&["privacy-policy-check"], &["passed", "passes"]]
        }
        "`cargo run -p xtask -- website-check`" => &[&["website-check"], &["passed", "passes"]],
        "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`" => {
            &[&["artifact-check"], &["passed", "passes"], &["dropsquash.dmg", ".dmg"]]
        }
        "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`" => {
            &[&["sha-256", "sha256"], &["dropsquash.dmg", ".dmg"]]
        }
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>`" => {
            &[
                &["csv"],
                &[".csv"],
                &["three", "3"],
                &["smaller"],
                &["outside repo", "outside repository"],
            ]
        }
        "`cargo run -p xtask -- manual-qa-check`" => {
            &[&["manual-qa-check"], &["passed", "passes"]]
        }
        "`cargo run -p xtask -- macos-signing-check`" => {
            &[&["macos-signing-check"], &["passed", "passes"], &["release environment"]]
        }
        "Codesign verification" => &[&["codesign"], &["developer id"]],
        "Notarization staple verification" => &[&["notary", "notarization"], &["staple", "spctl"]],
        "Gatekeeper open test" => &[
            &["gatekeeper"],
            &["opened", "opens"],
            &["clean", "fresh"],
            &["signed"],
            &["notarized", "notarised"],
            &["stapled", "staple"],
            &["without warning", "no warning", "without gatekeeper warning"],
        ],
        _ => return None,
    };
    Some(groups)
}
