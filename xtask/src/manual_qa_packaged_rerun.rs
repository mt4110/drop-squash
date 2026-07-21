use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const DEFAULT_MANUAL: &str = "docs/manual-qa.md";
const PACKAGED_GUIDE: &str = "docs/manual-qa.md";
const LICENSE_RUNBOOK: &str = "docs/license-sandbox-runbook.md";
const SIGNED_DMG_RUNBOOK: &str = "docs/signed-dmg-runbook.md";
const PAID_BETA_CHECKLIST: &str = "docs/paid-beta-operator-checklist.md";
const DISTRIBUTION_HANDOFF: &str =
    "distribution snapshot handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)";
const PANEL_DIR: &str = "/tmp/dropsquash-qa-open-panel";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-packaged-rerun [manual-qa.md]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let csv = benchmark_csv(&text).ok_or_else(|| missing_csv(&path))?;
    for line in lines(&path, &csv, has_dmg_artifact(&text)) {
        println!("{line}");
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [] => Ok(PathBuf::from(DEFAULT_MANUAL)),
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn benchmark_csv(text: &str) -> Option<String> {
    table_rows(text)
        .into_iter()
        .find(|cells| cells.len() >= 3 && cells[0] == "Benchmark sample set")
        .and_then(|cells| crate::csv_evidence::existing_outside_repo_path(&cells[2]))
        .map(|path| path.display().to_string())
}

fn field_value(text: &str, label: &str) -> Option<String> {
    table_rows(text)
        .into_iter()
        .find_map(|cells| match cells.as_slice() {
            [found, value] if found == label => Some(value.clone()),
            _ => None,
        })
}

fn has_dmg_artifact(text: &str) -> bool {
    field_value(text, "App artifact").is_some_and(|artifact| artifact.ends_with(".dmg"))
}

fn lines(path: &Path, csv: &str, has_dmg_artifact: bool) -> Vec<String> {
    let mut lines = vec![
        format!("packaged manual QA guide: {PACKAGED_GUIDE}"),
        format!("license sandbox runbook: {LICENSE_RUNBOOK}"),
        format!("signed DMG runbook: {SIGNED_DMG_RUNBOOK}"),
        DISTRIBUTION_HANDOFF.to_string(),
        format!("paid beta operator checklist: {PAID_BETA_CHECKLIST}"),
        format!(
            "packaged rerun helper: cargo run -p xtask -- manual-qa-ready-local-proof '{}' '{}'",
            quote(path),
            quote(&PathBuf::from(csv))
        ),
        format!(
            "packaged sample-link helper: cargo run -p xtask -- manual-qa-link-samples '{}' {PANEL_DIR}",
            quote(&PathBuf::from(csv))
        ),
        crate::manual_qa_observation::packaged_visibility_reminder(
            "packaged observation reminder",
        ),
        format!(
            "packaged pending helper: cargo run -p xtask -- manual-qa-pending '{}' --section packaged-app",
            quote(path)
        ),
        format!(
            "packaged local-proof gate: cargo run -p xtask -- manual-qa-check '{}' --section local-proof",
            quote(path)
        ),
        format!(
            "packaged final gate: cargo run -p xtask -- manual-qa-check '{}'",
            quote(path)
        ),
        "paid beta blocker gate: cargo run -p xtask -- paid-beta-check".to_string(),
    ];
    if has_dmg_artifact {
        lines.extend([
            "packaged installed-app status: cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app".to_string(),
            "packaged installed-app stash: cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app".to_string(),
            "packaged installed-app restore: cargo run -p xtask -- manual-qa-installed-app restore /tmp/dropsquash-manual-qa-installed-app".to_string(),
            "packaged mounted dmg window probe: cargo run -p xtask -- manual-qa-window-probe".to_string(),
        ]);
    }
    lines
}

fn table_rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}

fn missing_csv(path: &Path) -> String {
    format!(
        "manual QA file is missing a checked Benchmark sample set CSV outside the repository; recover with `cargo run -p xtask -- manual-qa-prepare --reset-trial ...`, `cargo run -p xtask -- benchmark --release-set ...`, `cargo run -p xtask -- benchmark-csv-check ...`, then `cargo run -p xtask -- manual-qa-ready-all '{}' <results.csv>` for the standard paid-beta pass or `cargo run -p xtask -- manual-qa-ready-local-proof '{}' <results.csv>` for packaged-only proof",
        quote(path)
        , quote(path)
    )
}
