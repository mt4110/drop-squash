const DEFAULT_MANUAL: &str = "docs/manual-qa.md";
const P1_REVIEW: &str = "docs/p1-review.md";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-p1-review [manual-qa.md]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let manual = match args.as_slice() {
        [] => DEFAULT_MANUAL.to_string(),
        [path] if path != "--help" && path != "-h" => path.clone(),
        _ => return Err(USAGE.to_string()),
    };
    for line in lines(&manual) {
        println!("{line}");
    }
    Ok(())
}

fn lines(manual: &str) -> Vec<String> {
    vec![
        format!("p1 review guide: {P1_REVIEW}"),
        "p1 baseline web test: pnpm --dir apps/desktop/web test".to_string(),
        "p1 baseline single-instance test: cargo test -p dropsquash-desktop single_instance -- --nocapture".to_string(),
        "p1 baseline file-size gate: cargo run -p xtask -- file-size-check".to_string(),
        "p1 baseline release gate: cargo run -p xtask -- release-check".to_string(),
        format!("p1 packaged rerun: cargo run -p xtask -- manual-qa-packaged-rerun '{manual}'"),
        "p1 mounted window probe: cargo run -p xtask -- manual-qa-window-probe".to_string(),
        "p1 review states: trial -> locked -> install notice -> queue + settings -> relaunch / single-instance".to_string(),
        "p1 verdict: ready for P2 alpha entry / needs more polish".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::lines;

    #[test]
    fn prints_p1_review_flow() {
        let lines = lines("docs/manual-qa.md");
        assert!(lines[0].contains("docs/p1-review.md"));
        assert!(lines
            .iter()
            .any(|line| line.contains("pnpm --dir apps/desktop/web test")));
        assert!(lines.iter().any(|line| line.contains("single_instance")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-packaged-rerun")));
        assert!(lines
            .iter()
            .any(|line| line.contains("manual-qa-window-probe")));
        assert!(lines.iter().any(|line| line.contains("needs more polish")));
    }
}
