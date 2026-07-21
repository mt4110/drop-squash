use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv> [baseline-results.csv]";
const PAID_BETA_CHECKLIST: &str = "docs/paid-beta-operator-checklist.md";
const PACKAGED_GUIDE: &str = "docs/manual-qa.md";
const LICENSE_RUNBOOK: &str = "docs/license-sandbox-runbook.md";
const SIGNED_DMG_RUNBOOK: &str = "docs/signed-dmg-runbook.md";
const DISTRIBUTION_HANDOFF: &str =
    "distribution snapshot handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)";
const ISOLATED_PREPARE: &str = "cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash\" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared.md";
const BENCHMARK_TEMPLATE: &str = "cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir {output_dir} --csv-output {csv_path}";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let parsed = parse_args(args)?;
    ensure_current_csv_exists(&parsed.1)?;
    crate::manual_qa_ready_local_proof::run(local_proof_args(&parsed))?;
    crate::manual_qa_license_rerun::run(vec![display(&parsed.0)])?;
    crate::manual_qa_distribution_rerun::run(vec![display(&parsed.0)])?;
    print_pending_paid_beta_rows(&parsed.0)?;
    println!(
        "all deterministic manual QA helpers completed: {}",
        parsed.0.display()
    );
    for line in next_lines(&parsed.0) {
        println!("{line}");
    }
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

fn display(path: &Path) -> String {
    path.display().to_string()
}

fn ensure_current_csv_exists(path: &Path) -> Result<(), String> {
    if path.is_file() {
        return Ok(());
    }
    Err(format!(
        "benchmark CSV does not exist: {}. rerun `{ISOLATED_PREPARE}`, then run `{}` and `cargo run -p xtask -- benchmark-csv-check {}` before using the fresh benchmark-results-<app-build>.csv path with `manual-qa-ready-all`",
        path.display(),
        benchmark_command(path),
        path.display()
    ))
}

fn benchmark_command(path: &Path) -> String {
    let output_dir = path
        .parent()
        .map(|parent| parent.display().to_string())
        .unwrap_or_else(|| "/tmp/dropsquash-manual-qa-output".to_string());
    BENCHMARK_TEMPLATE
        .replace("{output_dir}", &output_dir)
        .replace("{csv_path}", &path.display().to_string())
}

fn print_pending_paid_beta_rows(path: &Path) -> Result<(), String> {
    let license = crate::manual_qa_pending::markdown_rows(path, "license")?;
    let distribution = crate::manual_qa_pending::markdown_rows(path, "distribution")?;
    if !license.is_empty() {
        println!(
            "pending paid beta license markdown rows:\n{}",
            license.join("\n")
        );
    }
    if !distribution.is_empty() {
        println!(
            "pending paid beta distribution markdown rows:\n{}",
            distribution.join("\n")
        );
    }
    Ok(())
}

fn next_lines(path: &Path) -> Vec<String> {
    let manual = display(path);
    vec![
        format!("next operator checklist: {PAID_BETA_CHECKLIST}"),
        format!("packaged manual QA guide: {PACKAGED_GUIDE}"),
        format!("license sandbox runbook: {LICENSE_RUNBOOK}"),
        format!("signed DMG runbook: {SIGNED_DMG_RUNBOOK}"),
        DISTRIBUTION_HANDOFF.to_string(),
        "next paid beta manual QA helper: cargo run -p xtask -- manual-qa-paid-beta-rerun"
            .to_string(),
        format!(
            "next license section gate: cargo run -p xtask -- manual-qa-pending '{manual}' --section license"
        ),
        format!(
            "next distribution section gate: cargo run -p xtask -- manual-qa-pending '{manual}' --section distribution"
        ),
        format!("next final manual QA gate: cargo run -p xtask -- manual-qa-check '{manual}'"),
        "next paid beta check command: cargo run -p xtask -- paid-beta-check".to_string(),
    ]
}
