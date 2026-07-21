use std::collections::BTreeMap;
use std::path::Path;

use super::hints;
use super::manual_rows::PendingSummary;
mod scope;
mod sections;

pub(crate) fn suggested_commands(
    required: &[String],
    statuses: &BTreeMap<String, &'static str>,
    manual_qa_path: &Path,
    manual_rows: Option<&PendingSummary>,
    git_status: Option<&str>,
) -> Vec<String> {
    let pending = |name: &str| {
        statuses
            .get(name)
            .map_or(true, |status| *status != "Verified")
    };
    let manual_path = hints::shell_path(manual_qa_path);
    let mut commands = Vec::new();
    commands.push("paid beta operator checklist: docs/paid-beta-operator-checklist.md".to_string());
    let packaged_pending = scope::packaged(required, &pending);
    let license_pending = scope::license(required, &pending);
    let distribution_pending = scope::distribution(required, &pending);
    let has_local_pending = manual_rows.map_or(true, |summary| !summary.other.is_empty());
    if packaged_pending || license_pending || distribution_pending {
        commands.push(
            "combined paid beta rerun entrypoint: cargo run -p xtask -- manual-qa-paid-beta-rerun"
                .to_string(),
        );
        if distribution_pending {
            commands.push(
                "direct signing track: cargo run -p xtask -- productization-status --track \"Signing and distribution proof\""
                    .to_string(),
            );
        }
        if license_pending {
            commands.push(
                "direct license track: cargo run -p xtask -- productization-status --track \"License sandbox proof\""
                    .to_string(),
            );
        }
        commands.push(
            "after manual-qa-paid-beta-rerun: use `paid beta license markdown rows` and `paid beta distribution markdown rows` before the section gates".to_string(),
        );
        commands.extend(hints::dirty_worktree_quickstart(git_status));
    }
    if let Some(csv) = manual_rows
        .and_then(|summary| summary.benchmark_csv.as_ref())
        .filter(|_| license_pending || distribution_pending)
    {
        commands.push(format!(
            "preferred deterministic first pass: cargo run -p xtask -- manual-qa-ready-all {manual_path} '{}'",
            csv.replace('\'', "'\\''")
        ));
    } else if license_pending || distribution_pending {
        if let Some(recovery) = hints::deterministic_recovery(manual_qa_path) {
            commands.push(recovery);
        }
    }
    sections::packaged(
        &mut commands,
        packaged_pending,
        has_local_pending,
        &manual_path,
        manual_rows,
    );
    if license_pending {
        sections::license(&mut commands, manual_qa_path, &manual_path);
    }
    if distribution_pending {
        sections::distribution(&mut commands, &manual_path);
    }
    if !has_local_pending {
        commands.push(format!(
            "cargo run -p xtask -- manual-qa-check {manual_path}"
        ));
    }
    commands
}
