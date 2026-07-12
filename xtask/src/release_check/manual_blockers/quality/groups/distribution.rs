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
                &["three", "3"],
                &["smaller"],
                &["outside repo", "outside repository"],
            ])
        }
        "Benchmark sample set" => Some(&[
            &["short"],
            &["medium"],
            &["large"],
            &["machine", "macbook", "mac "],
            &["macos", "os "],
        ]),
        "Benchmark regression threshold" => Some(&[&["20%", "20 percent"]]),
        _ => None,
    }
}
