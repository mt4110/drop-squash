use std::path::PathBuf;

#[cfg(test)]
mod tests;

const DRAFT_MARKER: &str = "Prepared manual QA draft only.";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-ready-distribution <manual-qa.md>";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    crate::manual_qa_release_gate_fill::run(vec![display(&path)])?;
    if needs_cleaning(&path)? {
        crate::manual_qa_clean_draft::run(vec![display(&path)])?;
    }
    println!("distribution proof draft is ready: {}", path.display());
    println!(
        "next distribution pending command: {}",
        distribution_pending_command(&path)
    );
    println!("next manual QA check command: {}", manual_check_command(&path));
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn needs_cleaning(path: &PathBuf) -> Result<bool, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    Ok(text.contains(DRAFT_MARKER))
}

fn display(path: &PathBuf) -> String {
    path.display().to_string()
}

fn distribution_pending_command(path: &PathBuf) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}' --section distribution",
        shell_single_quote(path)
    )
}

fn manual_check_command(path: &PathBuf) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-check '{}'",
        shell_single_quote(path)
    )
}

fn shell_single_quote(path: &PathBuf) -> String {
    path.display().to_string().replace('\'', "'\\''")
}
