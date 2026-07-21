pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    let groups: &[&[&str]] = match label {
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
        "`cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS`" => {
            &[&["sha-256", "sha256"], &["dropsquash.dmg", ".dmg"]]
        }
        "`cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md`" => {
            &[
                &["homebrew-cask-check"],
                &["passed", "passes"],
                &["dropsquash.rb"],
                &["release-notes.md"],
            ]
        }
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`" => {
            &[
                &["csv"],
                &[".csv"],
                &["three", "3"],
                &["smaller"],
                &["outside repo", "outside repository"],
            ]
        }
        "`cargo run -p xtask -- manual-qa-check <manual-qa.md> --section local-proof`" => {
            &[&["manual-qa-check"], &["local-proof"], &["passed", "passes"]]
        }
        "`cargo run -p xtask -- manual-qa-check`" => {
            &[&["manual-qa-check"], &["passed", "passes"]]
        }
        "`cargo run -p xtask -- macos-signing-check`" => {
            &[&["macos-signing-check"], &["passed", "passes"], &["release environment"]]
        }
        _ => return None,
    };
    Some(groups)
}
