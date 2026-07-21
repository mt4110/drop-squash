use std::path::PathBuf;

pub(crate) mod status;
#[cfg(test)]
mod tests;

const DEFAULT_MANUAL: &str = "docs/manual-qa.md";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-distribution-rerun [manual-qa.md]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    crate::manual_qa_dirty_worktree::print_quickstart();
    println!("signed dmg runbook: docs/signed-dmg-runbook.md");
    for line in status::lines() {
        println!("{line}");
    }
    crate::manual_qa_ready_distribution::run(vec![path.display().to_string()])
}

pub(super) fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [] => Ok(PathBuf::from(DEFAULT_MANUAL)),
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}
