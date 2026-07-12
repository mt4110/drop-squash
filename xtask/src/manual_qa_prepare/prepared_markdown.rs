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
    std::fs::write(path, format!("{}\n", rows.join("\n"))).map_err(|error| error.to_string())?;
    println!("manual QA Markdown output: {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests;
