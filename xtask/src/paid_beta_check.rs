use std::process::Command;

mod commands;
mod hints;
mod manual_rows;
mod notes;
mod parse;

const DEFERRED_SCOPE_NOTE: &str = "paid beta scope note: this check only covers the private/manual paid beta technical proof; public website deployment, pricing, refund, live checkout, published checksum, and Homebrew evidence remain required before production payment onboarding or a public paid beta";
const PUBLIC_WEB_HANDOFF: &str =
    "after paid-beta technical proof: start with `cargo run -p xtask -- public-web-ready`, use `cargo run -p xtask -- public-web-rerun` as the operator memo when needed, then run `cargo run -p xtask -- productization-status --track 'Public web proof'` and follow `docs/public-beta-operator-checklist.md` (`Short Execution Memo` for the fast path) before Stripe or Lemon Squeezy production onboarding";

pub fn run(args: Vec<String>) -> Result<(), String> {
    let (blockers_path, readiness_path, manual_qa_path) = parse::parse_args(args)?;
    let blockers = std::fs::read_to_string(&blockers_path)
        .map_err(|error| format!("failed to read {}: {error}", blockers_path.display()))?;
    let readiness = std::fs::read_to_string(&readiness_path)
        .map_err(|error| format!("failed to read {}: {error}", readiness_path.display()))?;
    let required = parse::required_blockers(&readiness)?;
    let statuses = parse::blocker_statuses(&blockers);
    let manual_rows = manual_rows::pending_summary(&manual_qa_path).ok();
    let git_status = git_status().ok();
    let notes = notes::blocker_notes(&required, &statuses, &manual_qa_path, manual_rows.as_ref());
    let remaining = required
        .iter()
        .filter(|name| {
            statuses
                .get(*name)
                .map_or(true, |status| *status != "Verified")
        })
        .map(|name| {
            format!(
                "- {name}: {}",
                statuses.get(name).copied().unwrap_or("Missing")
            )
        })
        .collect::<Vec<_>>();
    if remaining.is_empty() {
        println!(
            "paid beta blockers verified: {} required, {} verified\n{}\n{}",
            required.len(),
            required.len(),
            DEFERRED_SCOPE_NOTE,
            PUBLIC_WEB_HANDOFF,
        );
        return Ok(());
    }
    Err(format!(
        "paid beta blockers: {} required, {} verified, {} remaining\n{}\n{}\n{}\n{}\nnext commands:\n{}",
        required.len(),
        required.len() - remaining.len(),
        remaining.len(),
        remaining.join("\n"),
        render_supporting_output(manual_rows.as_ref(), &notes),
        DEFERRED_SCOPE_NOTE,
        PUBLIC_WEB_HANDOFF,
        commands::suggested_commands(
            &required,
            &statuses,
            &manual_qa_path,
            manual_rows.as_ref(),
            git_status.as_deref(),
        )
            .join("\n"),
    ))
}

fn git_status() -> Result<String, String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git status failed".to_string());
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn render_supporting_output(
    manual_rows: Option<&manual_rows::PendingSummary>,
    notes: &[String],
) -> String {
    let mut lines = manual_rows
        .map(|summary| summary.render())
        .unwrap_or_default();
    lines.extend(notes.iter().cloned());
    lines.join("\n")
}

#[cfg(test)]
mod tests;
