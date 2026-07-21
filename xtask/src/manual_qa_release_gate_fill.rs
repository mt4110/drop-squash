use std::path::PathBuf;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-fill-release-gates <manual-qa.md>";
const MAIN_MANUAL_QA: &str = "docs/manual-qa.md";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let filled = fill_release_gate_rows(&text, &results(&path)?)?;
    std::fs::write(&path, filled)
        .map_err(|error| format!("failed to write manual QA file: {error}"))?;
    println!("filled release gate rows: {}", path.display());
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn results(path: &PathBuf) -> Result<Vec<(&'static str, &'static str)>, String> {
    if uses_main_manual_qa(path) {
        crate::release_check::run()?;
    }
    crate::file_size_check::run(vec![])?;
    crate::media_policy_check::run()?;
    crate::privacy_policy_check::run()?;
    crate::website_check::run(vec![])?;
    let mut rows = vec![
        (
            "`cargo run -p xtask -- file-size-check`",
            "file-size-check passed",
        ),
        (
            "`cargo run -p xtask -- media-policy-check`",
            "media-policy-check passed",
        ),
        (
            "`cargo run -p xtask -- privacy-policy-check`",
            "privacy-policy-check passed",
        ),
        (
            "`cargo run -p xtask -- website-check`",
            "website-check passed",
        ),
    ];
    if uses_main_manual_qa(path) {
        rows.insert(
            0,
            (
                "`cargo run -p xtask -- release-check`",
                "release-check passed",
            ),
        );
    }
    Ok(rows)
}

fn uses_main_manual_qa(path: &PathBuf) -> bool {
    path == &PathBuf::from(MAIN_MANUAL_QA)
}

fn fill_release_gate_rows(text: &str, results: &[(&str, &str)]) -> Result<String, String> {
    let mut missing = results.iter().map(|(label, _)| *label).collect::<Vec<_>>();
    let lines = text
        .lines()
        .map(|line| replace_row(line, results, &mut missing))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(lines.join("\n"))
    } else {
        Err(format!(
            "manual QA file is missing release gate rows: {}",
            missing.join(", ")
        ))
    }
}

fn replace_row(line: &str, results: &[(&str, &str)], missing: &mut Vec<&str>) -> String {
    let cells = cells(line);
    let Some(label) = cells.first().copied() else {
        return line.to_string();
    };
    let Some((_, result)) = results.iter().find(|(target, _)| *target == label) else {
        return line.to_string();
    };
    missing.retain(|target| *target != label);
    let expected = cells.get(1).copied().unwrap_or("Passes");
    format!("| {label} | {expected} | {result} |")
}

fn cells(line: &str) -> Vec<&str> {
    if !line.starts_with('|') || line.contains("---") {
        return Vec::new();
    }
    line.trim_matches('|').split('|').map(str::trim).collect()
}
