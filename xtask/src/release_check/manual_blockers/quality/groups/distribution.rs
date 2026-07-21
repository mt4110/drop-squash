use super::Groups;

pub(super) fn for_check(check: &str) -> Option<Groups> {
    match check {
        "Gatekeeper open test" => Some(&[
            &["gatekeeper"],
            &["opened", "opens"],
            &["clean", "fresh"],
            &["public"],
            &["dropsquash.dmg"],
            &["signed"],
            &["notarized", "notarised"],
            &["stapled", "staple"],
            &["without warning", "no warning", "without gatekeeper warning"],
        ]),
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`" => {
            Some(&[
                &["csv"],
                &[".csv"],
                &["three", "3"],
                &["smaller"],
                &["outside repo", "outside repository"],
            ])
        }
        "Benchmark sample set" => Some(&[
            &["short"],
            &["medium"],
            &["large"],
            &["smaller"],
            &["backend"],
            &["csv"],
            &[".csv"],
            &["outside repo", "outside repository"],
            &["machine", "macbook", "mac "],
            &["macos", "os "],
        ]),
        "Benchmark regression threshold" => Some(&[
            &["20%", "20 percent"],
            &["regression"],
            &["sample"],
            &["same-machine", "same machine"],
            &["release candidate baseline"],
        ]),
        "`cargo run -p xtask -- manual-qa-check <manual-qa.md> --section local-proof`" => {
            Some(&[&["manual-qa-check"], &["local-proof"], &["passed", "passes"]])
        }
        "`cargo run -p xtask -- manual-qa-check`" => {
            Some(&[&["manual-qa-check"], &["passed", "passes"]])
        }
        _ => None,
    }
}
