use std::path::PathBuf;

use dropsquash_core::TRIAL_CONVERSION_LIMIT;
use dropsquash_history::{read_records, HistoryMetrics};

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-forget-license <license.json> [history.jsonl]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (path, history) = parse_args(args)?;
    forget_path(&path)?;
    println!("manual QA license cache forgotten: {}", path.display());
    println!(
        "manual QA license cache removed check: test ! -e \"{}\" && echo removed || echo still-present",
        path.display()
    );
    if let Some(history) = history {
        let state = post_forget_state(&history)?;
        println!("manual QA post-forget state: {state}");
        let candidate = row_candidate(state);
        println!("manual QA forget result hint: {candidate}");
        println!("manual QA forget row candidate: {candidate}");
        println!("manual QA markdown row: {}", markdown_row(&candidate));
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, Option<PathBuf>), String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok((PathBuf::from(path), None)),
        [path, history] => Ok((PathBuf::from(path), Some(PathBuf::from(history)))),
        _ => Err(USAGE.to_string()),
    }
}

fn forget_path(path: &std::path::Path) -> Result<(), String> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("failed to forget manual QA license cache: {error}")),
    }
}

fn post_forget_state(history: &std::path::Path) -> Result<&'static str, String> {
    let runtime = tokio::runtime::Runtime::new()
        .map_err(|error| format!("failed to create tokio runtime: {error}"))?;
    let records = runtime
        .block_on(read_records(history))
        .map_err(|error| format!("failed to read history for manual QA: {error}"))?;
    let metrics = HistoryMetrics::from_records(&records);
    Ok(
        if metrics.successful_conversion_count >= TRIAL_CONVERSION_LIMIT {
            "locked"
        } else {
            "trial"
        },
    )
}

fn row_candidate(state: &str) -> String {
    format!(
        "Forgetting state disabled action; confirmed license cache removed and observed app returned to {state} state"
    )
}

fn markdown_row(result: &str) -> String {
    format!(
        "| Forget license on this Mac | Forgetting state disables action; confirmed license cache removed; observed app returns to trial or locked state | {result} |"
    )
}
