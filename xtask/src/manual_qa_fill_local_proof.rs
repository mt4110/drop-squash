use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-fill-local-proof <manual-qa.md> <results.csv> [baseline-results.csv]";
const LOCAL_PROOF_LABEL: &str =
    "`cargo run -p xtask -- manual-qa-check <manual-qa.md> --section local-proof`";
const LOCAL_PROOF_RESULT: &str = "manual-qa-check --section local-proof passed";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (manual, current, baseline) = parse_args(args)?;
    crate::manual_qa_release_gate_fill::run(vec![display(&manual)])?;
    crate::manual_qa_benchmark_fill::run(vec![display(&manual), display(&current)])?;
    let mut threshold_args = vec![display(&manual), display(&current)];
    if let Some(path) = baseline {
        threshold_args.push(display(&path));
    }
    crate::manual_qa_fill_benchmark_threshold::run(threshold_args)?;
    fill_local_proof_gate(&manual)?;
    println!("filled local proof rows: {}", manual.display());
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf, Option<PathBuf>), String> {
    match args.as_slice() {
        [manual, current] if manual != "--help" && manual != "-h" => {
            Ok((PathBuf::from(manual), PathBuf::from(current), None))
        }
        [manual, current, baseline] if manual != "--help" && manual != "-h" => Ok((
            PathBuf::from(manual),
            PathBuf::from(current),
            Some(PathBuf::from(baseline)),
        )),
        _ => Err(USAGE.to_string()),
    }
}

fn display(path: &Path) -> String {
    path.display().to_string()
}

fn fill_local_proof_gate(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let filled = fill_local_proof_row(&text)?;
    let missing = crate::manual_qa_check::check_text_section(&filled, Some("local-proof"));
    if !missing.is_empty() {
        return Err(missing.join("\n"));
    }
    std::fs::write(path, filled).map_err(|error| format!("failed to write manual QA file: {error}"))
}

fn fill_local_proof_row(text: &str) -> Result<String, String> {
    let mut found = false;
    let lines = text
        .lines()
        .map(|line| {
            let cells = line
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            if cells.first().copied() == Some(LOCAL_PROOF_LABEL) {
                found = true;
                return format!("| {} | {} | {} |", cells[0], cells[1], LOCAL_PROOF_RESULT);
            }
            line.to_string()
        })
        .collect::<Vec<_>>();
    found
        .then(|| lines.join("\n"))
        .ok_or_else(|| "manual QA file is missing the local-proof gate row".to_string())
}
