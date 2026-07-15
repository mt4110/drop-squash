use std::path::PathBuf;

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv> [baseline-results.csv]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let parsed = parse_args(args)?;
    crate::manual_qa_ready_local_proof::run(local_proof_args(&parsed))?;
    crate::manual_qa_ready_license::run(vec![display(&parsed.0)])?;
    crate::manual_qa_ready_distribution::run(vec![display(&parsed.0)])?;
    println!("all deterministic manual QA helpers completed: {}", parsed.0.display());
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

fn local_proof_args(parsed: &(PathBuf, PathBuf, Option<PathBuf>)) -> Vec<String> {
    let mut args = vec![display(&parsed.0), display(&parsed.1)];
    if let Some(path) = parsed.2.as_ref() {
        args.push(display(path));
    }
    args
}

fn display(path: &PathBuf) -> String {
    path.display().to_string()
}
