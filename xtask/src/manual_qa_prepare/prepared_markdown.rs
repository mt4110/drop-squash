use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::{markdown, release_candidate};

pub(super) fn write(
    path: &Path,
    fields: &[markdown::Field],
    artifact: Option<&Path>,
) -> Result<(), String> {
    let mut rows = markdown::rows(fields);
    if let Some(artifact) = artifact {
        rows.extend(release_candidate::rows(artifact)?);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("failed to create manual QA Markdown output: {error}"))?;
    writeln!(file, "{}", markdown_text(&rows))
        .map_err(|error| format!("failed to write manual QA Markdown output: {error}"))?;
    println!("manual QA Markdown output: {}", path.display());
    Ok(())
}

fn markdown_text(rows: &[String]) -> String {
    format!(
        "{}\n\n{}",
        "Prepared manual QA draft only. Replace this file with concrete observations.",
        rows.join("\n")
    )
}

#[cfg(test)]
mod tests;
