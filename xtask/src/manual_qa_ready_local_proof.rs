use std::path::PathBuf;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-ready-local-proof <manual-qa.md> <results.csv> [baseline-results.csv]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let parsed = parse_args(args)?;
    crate::manual_qa_fill_local_proof::run(run_args(&parsed))?;
    crate::manual_qa_clean_draft::run(vec![display(&parsed.0)])?;
    println!("prepared local proof draft is ready: {}", parsed.0.display());
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

fn run_args(parsed: &(PathBuf, PathBuf, Option<PathBuf>)) -> Vec<String> {
    let mut args = vec![display(&parsed.0), display(&parsed.1)];
    if let Some(path) = parsed.2.as_ref() {
        args.push(display(path));
    }
    args
}

fn display(path: &PathBuf) -> String {
    path.display().to_string()
}
