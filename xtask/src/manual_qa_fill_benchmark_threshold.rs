use std::path::PathBuf;

mod compare;
mod markdown;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-fill-benchmark-threshold <manual-qa.md> <results.csv> [baseline-results.csv]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (manual, current, baseline) = parse_args(args)?;
    crate::benchmark_csv_check::validate_path(&current)?;
    if let Some(path) = baseline.as_deref() {
        crate::benchmark_csv_check::validate_path(path)?;
    }
    let text = std::fs::read_to_string(&manual)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let filled = fill_rows(&text, &compare::result(&current, baseline.as_deref())?)?;
    std::fs::write(&manual, filled)
        .map_err(|error| format!("failed to write manual QA file: {error}"))?;
    println!("filled benchmark threshold row: {}", manual.display());
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

fn fill_rows(text: &str, result: &str) -> Result<String, String> {
    let mut found = false;
    let lines = text
        .lines()
        .map(|line| match markdown::label(line) {
            Some("Benchmark regression threshold") => {
                found = true;
                markdown::replace(line, result)
            }
            _ => line.to_string(),
        })
        .collect::<Vec<_>>();
    found
        .then(|| lines.join("\n"))
        .ok_or_else(|| "manual QA file is missing benchmark threshold row".to_string())
}
