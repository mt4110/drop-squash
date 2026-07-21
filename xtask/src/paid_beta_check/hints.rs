use super::manual_rows::PendingSummary;
use std::path::Path;

pub(super) fn license_browser_sign_in_checkpoint() -> String {
    "license browser sign-in checkpoint: if Lemon Squeezy still shows `Sign in to Lemon Squeezy` or `auth.lemonsqueezy.com/login`, stop and sign in before recording sandbox product setup, purchase, or activation".to_string()
}

pub(super) fn license_browser_sign_in_success() -> String {
    "license browser sign-in success: continue only after the Lemon Squeezy dashboard is open and sandbox mode is visible for the intended DropSquash product".to_string()
}

pub(super) fn license_activation_loop() -> String {
    "license activation loop: run sandbox quickstart 2 before the UI action, then sandbox quickstart 4 after Pro appears so the cache delta is recorded".to_string()
}

pub(super) fn local_proof(manual_path: &str, manual_rows: Option<&PendingSummary>) -> String {
    match manual_rows.and_then(|summary| summary.benchmark_csv.as_ref()) {
        Some(csv) if manual_path == "'docs/manual-qa.md'" => format!(
            "shortest packaged rerun entrypoint: cargo run -p xtask -- manual-qa-packaged-rerun (uses recorded benchmark CSV {})",
            csv.replace('\'', "'\\''")
        ),
        Some(csv) => format!(
            "after the fresh benchmark CSV exists: standard deterministic entrypoint `cargo run -p xtask -- manual-qa-ready-all {manual_path} '{}'`; packaged-only alternative `cargo run -p xtask -- manual-qa-ready-local-proof {manual_path} '{}'`; then use the emitted sample-link and installed-app helper commands before chooser or mounted-DMG checks",
            csv.replace('\'', "'\\''")
            , csv.replace('\'', "'\\''")
        ),
        None => "after the fresh benchmark CSV exists: run `manual-qa-ready-all` as the standard deterministic entrypoint or `manual-qa-ready-local-proof` as the packaged-only alternative, then use the emitted sample-link and installed-app helper commands before chooser or mounted-DMG checks".to_string(),
    }
}

pub(super) fn license_rerun(manual_path: &str) -> String {
    match manual_path {
        "'docs/manual-qa.md'" => {
            "shortest license rerun entrypoint: cargo run -p xtask -- manual-qa-license-rerun"
                .to_string()
        }
        _ => format!("cargo run -p xtask -- manual-qa-license-rerun {manual_path}"),
    }
}

pub(super) fn distribution_rerun(manual_path: &str) -> String {
    match manual_path {
        "'docs/manual-qa.md'" => {
            "shortest distribution rerun entrypoint: cargo run -p xtask -- manual-qa-distribution-rerun".to_string()
        }
        _ => format!("cargo run -p xtask -- manual-qa-distribution-rerun {manual_path}"),
    }
}

pub(super) fn distribution_handoff() -> String {
    "distribution snapshot handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)".to_string()
}

pub(super) fn isolated_prepare(path: &Path) -> Option<String> {
    (path == Path::new("docs/manual-qa.md")).then(|| {
        "cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash\" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared-<app-build>.md".to_string()
    })
}

pub(super) fn deterministic_recovery(path: &Path) -> Option<String> {
    (path == Path::new("docs/manual-qa.md")).then(|| {
        "preferred deterministic recovery: rerun `manual-qa-prepare --reset-trial`, `benchmark --release-set`, `benchmark-csv-check`, then `manual-qa-ready-all` before sandbox or signing reruns".to_string()
    })
}

pub(super) fn dirty_worktree_quickstart(git_status: Option<&str>) -> Vec<String> {
    crate::manual_qa_dirty_worktree::quickstart_lines(git_status)
}

pub(super) fn shell_path(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', "'\\''"))
}
