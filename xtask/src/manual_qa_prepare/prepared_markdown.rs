use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::{
    benchmark, commands, fill_commands, license_sandbox, markdown, options::Options, packaged_app,
    release_candidate, release_gate,
};

pub(super) fn write(
    path: &Path,
    fields: &[markdown::Field],
    artifact: Option<&Path>,
    options: &Options,
    benchmark_csv: &Path,
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
    writeln!(
        file,
        "{}",
        markdown_text(&rows, fields, artifact, options, path, benchmark_csv)
    )
    .map_err(|error| format!("failed to write manual QA Markdown output: {error}"))?;
    println!("manual QA Markdown output: {}", path.display());
    Ok(())
}

fn markdown_text(
    rows: &[String],
    fields: &[markdown::Field],
    artifact: Option<&Path>,
    options: &Options,
    output_path: &Path,
    benchmark_csv: &Path,
) -> String {
    let mut sections = vec![
        "Prepared manual QA draft only. Replace this file with concrete observations.".to_string(),
        format!(
            "Benchmark commands:\n\n```sh\n{}\n{}\n```",
            benchmark::benchmark_command(options, benchmark_csv),
            benchmark::csv_check_command(benchmark_csv)
        ),
        benchmark_context(fields, benchmark_csv),
        "Recommended proof flow: after `benchmark-csv-check` passes, run `manual-qa-ready-all` for one deterministic pass, or run `manual-qa-ready-local-proof` first and then use `manual-qa-ready-license` plus `manual-qa-ready-distribution` before their respective `manual-qa-pending --section ...` passes.".to_string(),
        fill_commands::block(output_path, benchmark_csv),
    ];
    if let Some(path) =
        artifact.filter(|path| path.extension().and_then(|value| value.to_str()) == Some("dmg"))
    {
        sections.push(format!(
            "Packaged app command:\n\n```sh\n{}\n```",
            commands::open_dmg_command(path)
        ));
        sections.push(format!(
            "Checksum command:\n\n```sh\n{}\n```",
            release_candidate::checksum_command(path, &options.output_dir)
        ));
    }
    if options.reset_trial {
        sections.push(format!(
            "Trial state restore command:\n\n```sh\n{}\n```",
            commands::restore_command(options)
        ));
    }
    sections.push(format!(
        "Manual QA check command after filling observations (run it from the same clean worktree that built the App build above):\n\n```sh\n{}\n```",
        commands::manual_check_command(output_path)
    ));
    sections.push(rows.join("\n"));
    sections.join("\n\n")
}

fn benchmark_context(fields: &[markdown::Field], csv: &Path) -> String {
    let macos = field_value(fields, "macOS version");
    let machine = field_value(fields, "Machine");
    format!(
        "Benchmark context to record:\n\n- CSV: {}\n- macOS: {}\n- Machine: {}\n- Result skeleton: {}",
        csv.display(),
        macos.unwrap_or("<record macOS version>"),
        machine.unwrap_or("<record machine>"),
        result_skeleton(machine, macos, &csv)
    )
}

fn result_skeleton(machine: Option<&str>, macos: Option<&str>, csv: &Path) -> String {
    format!(
        "Backend: apple-native. Samples: three short, medium, and large local recordings produced smaller outputs. short.mov duration <seconds>s <saved>% saved <throughput> MiB/s <ratio>x speed ratio; medium.mov duration <seconds>s <saved>% saved <throughput> MiB/s <ratio>x speed ratio; large.mov duration <seconds>s <saved>% saved <throughput> MiB/s <ratio>x speed ratio. Machine: {}. OS: {}. CSV saved outside repo: {}",
        machine.unwrap_or("<record machine>"),
        macos.unwrap_or("<record macOS version>"),
        csv.display()
    )
}

fn field_value<'a>(fields: &'a [markdown::Field], label: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(field, _)| *field == label)
        .map(|(_, value)| value.as_str())
}

#[cfg(test)]
mod tests;
