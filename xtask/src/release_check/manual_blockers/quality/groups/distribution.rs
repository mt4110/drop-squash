use super::Groups;

pub(super) fn for_check(check: &str) -> Option<Groups> {
    match check {
        "Gatekeeper open test" => Some(&[
            &["gatekeeper"],
            &["opened", "opens"],
            &["clean", "fresh"],
            &["signed"],
            &["notarized", "notarised"],
            &["stapled", "staple"],
            &["without warning", "no warning", "without gatekeeper warning"],
        ]),
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>`" => {
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
            &["machine", "macbook", "mac "],
            &["macos", "os "],
        ]),
        "Benchmark regression threshold" => Some(&[&["20%", "20 percent"]]),
        "`cargo run -p xtask -- manual-qa-check`" => {
            Some(&[&["manual-qa-check"], &["passed", "passes"]])
        }
        _ => None,
    }
}
