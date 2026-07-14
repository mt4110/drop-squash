use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::{
    benchmark, license_sandbox, markdown, options::Options, packaged_app, release_candidate,
    release_gate,
};

pub(super) fn write(
    path: &Path,
    fields: &[markdown::Field],
    artifact: Option<&Path>,
    options: &Options,
) -> Result<(), String> {
    let mut rows = markdown::rows(fields);
    rows.extend(packaged_app::rows());
    rows.extend(license_sandbox::rows());
    rows.extend(benchmark::rows());
    rows.extend(release_gate::rows());
    if let Some(artifact) = artifact {
        rows.extend(release_candidate::rows(artifact)?);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("failed to create manual QA Markdown output: {error}"))?;
    writeln!(file, "{}", markdown_text(&rows, artifact, options))
        .map_err(|error| format!("failed to write manual QA Markdown output: {error}"))?;
    println!("manual QA Markdown output: {}", path.display());
    Ok(())
}

fn markdown_text(rows: &[String], artifact: Option<&Path>, options: &Options) -> String {
    let mut sections = vec![
        "Prepared manual QA draft only. Replace this file with concrete observations.".to_string(),
        format!(
            "Benchmark commands:\n\n```sh\n{}\n{}\n```",
            benchmark::benchmark_command(options),
            benchmark::csv_check_command(options)
        ),
    ];
    if let Some(path) =
        artifact.filter(|path| path.extension().and_then(|value| value.to_str()) == Some("dmg"))
    {
        sections.push(format!(
            "Checksum command:\n\n```sh\n{}\n```",
            release_candidate::checksum_command(path, &options.output_dir)
        ));
    }
    sections.push(rows.join("\n"));
    sections.join("\n\n")
}

#[cfg(test)]
mod tests;
