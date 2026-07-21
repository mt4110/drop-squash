use super::{ready_all, APP_STATE_DIR, OUTPUT_DIR, QA_MARKDOWN, STATE_DIR};

pub(super) fn lines() -> Vec<String> {
    vec![
        format!(
            "preflight prepare markdown: cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"{APP_STATE_DIR}\" --state-dir {STATE_DIR} --output-dir {OUTPUT_DIR} --markdown-output {QA_MARKDOWN}"
        ),
        ready_all(),
        "preflight combined rerun helper: cargo run -p xtask -- manual-qa-paid-beta-rerun".to_string(),
        "preflight distribution handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)".to_string(),
        format!("preflight ready distribution: cargo run -p xtask -- manual-qa-distribution-rerun {QA_MARKDOWN}"),
        format!("preflight distribution markdown rows: cargo run -p xtask -- manual-qa-pending {QA_MARKDOWN} --section distribution | rg 'distribution .*markdown row:'"),
        "preflight paid beta gate: cargo run -p xtask -- paid-beta-check".to_string(),
        "preflight signing environment: CARGO_TARGET_DIR=/tmp/dsq-xtask-target cargo run -p xtask -- macos-signing-check".to_string(),
        "preflight signing env requirements: signing needs APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization also needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID".to_string(),
        "preflight distribution manual gate: cargo run -p xtask -- manual-qa-check".to_string(),
    ]
}
