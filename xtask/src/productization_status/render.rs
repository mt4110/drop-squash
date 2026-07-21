mod lines;
mod stats;
mod tracks;

use super::{model::Report, scope::Scope};

pub(super) fn text(report: &Report, track: Option<&str>, scope: &Scope) -> Result<String, String> {
    let scoped = track.is_none();
    let tracks = tracks::filtered(report, track, scope, scoped)?;
    let mut output = vec![
        "Productization status".to_string(),
        format!("release blockers: {}", stats::all(report)),
    ];
    if scoped {
        output.push(format!("paid beta now: {}", stats::scoped(report, scope)));
        output.push(format!(
            "deferred from paid-beta technical proof: {}",
            scope.deferred().join(", ")
        ));
        output.push(
            "these deferred items still remain required before production payment onboarding or a public paid beta"
                .to_string(),
        );
        output.push(
            "when paid-beta technical proof is complete, continue with: docs/public-beta-operator-checklist.md"
                .to_string(),
        );
        output.push(
            "deferred track detail: cargo run -p xtask -- productization-status --track \"Public web proof\""
                .to_string(),
        );
        if tracks.iter().any(|entry| !entry.remaining.is_empty()) {
            output.push("paid beta proof map: docs/paid-beta-readiness.md".to_string());
            output.push(
                "paid beta operator checklist: docs/paid-beta-operator-checklist.md".to_string(),
            );
            output.push(
                "paid beta manual QA helper: cargo run -p xtask -- manual-qa-paid-beta-rerun"
                    .to_string(),
            );
            output.push(
                "after paid beta manual QA helper: use `paid beta license markdown rows` and `paid beta distribution markdown rows` before section gates when only the remaining copy-ready rows are needed".to_string(),
            );
            output.push(
                "paid beta deterministic helper: if you have a fresh prepared manual-QA draft and checked benchmark CSV, run cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv> before section reruns".to_string(),
            );
            if tracks
                .iter()
                .any(|entry| entry.name == "Signing and distribution proof")
            {
                output.push(
                    "paid beta direct signing track: cargo run -p xtask -- productization-status --track \"Signing and distribution proof\""
                        .to_string(),
                );
            }
            if tracks
                .iter()
                .any(|entry| entry.name == "License sandbox proof")
            {
                output.push(
                    "paid beta direct license track: cargo run -p xtask -- productization-status --track \"License sandbox proof\""
                        .to_string(),
                );
            }
        }
    }
    output.extend(
        tracks
            .iter()
            .map(|entry| lines::summary(entry, scope, scoped)),
    );
    output.extend(
        tracks
            .iter()
            .filter_map(|entry| lines::remaining(entry, scope, scoped)),
    );
    output.extend(tracks.iter().filter_map(lines::record_target));
    output.extend(
        tracks
            .iter()
            .filter_map(|entry| lines::actions(report, entry, scope, scoped)),
    );
    output.extend(
        tracks
            .iter()
            .filter_map(|entry| lines::command(entry, scope, scoped)),
    );
    output.extend(lines::next_track(
        report,
        &tracks,
        track.is_some(),
        scope,
        scoped,
    ));
    Ok(output.join("\n"))
}
