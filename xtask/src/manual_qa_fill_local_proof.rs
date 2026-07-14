use std::path::PathBuf;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-fill-local-proof <manual-qa.md> <results.csv> [baseline-results.csv]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (manual, current, baseline) = parse_args(args)?;
    crate::manual_qa_release_gate_fill::run(vec![display(&manual)])?;
    crate::manual_qa_benchmark_fill::run(vec![display(&manual), display(&current)])?;
    let mut threshold_args = vec![display(&manual), display(&current)];
    if let Some(path) = baseline {
        threshold_args.push(display(&path));
    }
    crate::manual_qa_fill_benchmark_threshold::run(threshold_args)?;
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

fn display(path: &PathBuf) -> String {
    path.display().to_string()
}
