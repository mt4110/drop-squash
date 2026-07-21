use std::path::Path;

const LICENSE: &[&str] = &[
    "Sandbox product setup",
    "Sandbox purchase",
    "Valid sandbox activation",
];
const DISTRIBUTION: &[&str] = &[
    "`cargo run -p xtask -- manual-qa-check`",
    "`cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md`",
    "`cargo run -p xtask -- macos-signing-check`",
    "Codesign verification",
    "Notarization staple verification",
    "Gatekeeper open test",
];

pub(crate) struct PendingSummary {
    pub license: Vec<String>,
    pub distribution: Vec<String>,
    pub other: Vec<String>,
    pub benchmark_csv: Option<String>,
    pub app_build: Option<String>,
    pub app_artifact: Option<String>,
}

impl PendingSummary {
    pub(crate) fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.other.is_empty() {
            lines.push("manual QA local proof rows: none pending".into());
        } else {
            lines.push("manual QA local proof rows still empty:".into());
            lines.extend(self.other.iter().map(|label| format!("- {label}")));
        }
        if self.license.is_empty() {
            lines.push("manual QA license rows: none pending".into());
        } else {
            lines.push("manual QA license rows still empty:".into());
            lines.extend(self.license.iter().map(|label| format!("- {label}")));
        }
        if self.distribution.is_empty() {
            lines.push("manual QA distribution rows: none pending".into());
        } else {
            lines.push("manual QA distribution rows still empty:".into());
            lines.extend(self.distribution.iter().map(|label| format!("- {label}")));
            lines.push("manual QA distribution note: these pending rows include helper/preflight rows plus the three paid-beta blocker rows Signed DMG, Notarized and stapled DMG, and Gatekeeper clean-machine open".into());
        }
        lines
    }
}

pub(crate) fn pending_summary(path: &Path) -> Result<PendingSummary, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let pending = pending_labels(&text);
    Ok(PendingSummary {
        license: render_group(&pending, LICENSE),
        distribution: render_group(&pending, DISTRIBUTION),
        other: pending
            .into_iter()
            .filter(|label| {
                !LICENSE.contains(&label.as_str()) && !DISTRIBUTION.contains(&label.as_str())
            })
            .collect(),
        benchmark_csv: benchmark_csv(&text),
        app_build: field_value(&text, "App build"),
        app_artifact: field_value(&text, "App artifact"),
    })
}

fn render_group(pending: &[String], group: &[&str]) -> Vec<String> {
    group
        .iter()
        .filter(|label| pending.iter().any(|pending| pending == **label))
        .map(|label| (*label).to_string())
        .collect()
}

fn pending_labels(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>()
        })
        .filter(|cells| cells.len() >= 3 && cells[0] != "Check" && !cells[0].starts_with("---"))
        .filter(|cells| cells[2].is_empty())
        .map(|cells| cells[0].clone())
        .collect()
}

fn benchmark_csv(text: &str) -> Option<String> {
    markdown_rows(text)
        .into_iter()
        .find(|cells| cells.len() >= 3 && cells[0] == "Benchmark sample set")
        .and_then(|cells| crate::csv_evidence::existing_outside_repo_path(&cells[2]))
        .map(|path| path.display().to_string())
}

fn markdown_rows(text: &str) -> Vec<Vec<String>> {
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

fn field_value(text: &str, label: &str) -> Option<String> {
    markdown_rows(text)
        .into_iter()
        .find(|cells| cells.len() >= 2 && cells[0] == label)
        .map(|cells| cells[1].clone())
        .filter(|value| !value.is_empty())
}
