use std::path::Path;

mod complete;
mod distribution;
mod distribution_candidates;
mod distribution_commands;
mod fields;
mod gates;
mod license;
mod license_candidates;
mod license_commands;
mod markdown_rows;
mod packaged_app;
mod pending_rows;
mod phases;
pub(crate) mod ready;
mod row_notes;
mod run;
mod sample_hints;
pub(crate) mod section;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-pending <manual-qa.md> [--section packaged-app|license|benchmark|distribution|local-proof]";

#[cfg(test)]
mod tests;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    run::pending(args)
}

pub(crate) fn suggested_section(label: &str) -> &'static str {
    section::filter_for_label(label)
}

pub(crate) fn markdown_rows(path: &Path, filter: &str) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    Ok(markdown_rows::for_filter(&text, filter))
}
