use std::path::Path;

use crate::manual_qa_observation::packaged_visibility_reminder;

use super::super::hints;
use super::super::manual_rows::PendingSummary;

pub(super) fn packaged(
    commands: &mut Vec<String>,
    pending: bool,
    has_local_pending: bool,
    manual_path: &str,
    manual_rows: Option<&PendingSummary>,
) {
    if !pending {
        return;
    }
    commands.push("packaged manual QA guide: docs/manual-qa.md".to_string());
    commands.push("proof map: Packaged macOS manual QA -> manual-qa-packaged-rerun -> docs/manual-qa.md -> record in docs/manual-qa.md".to_string());
    let command = if has_local_pending {
        format!("cargo run -p xtask -- manual-qa-pending {manual_path} --section local-proof")
    } else {
        format!("cargo run -p xtask -- manual-qa-check {manual_path} --section local-proof")
    };
    commands.push(command);
    commands.push(hints::local_proof(manual_path, manual_rows));
    commands.push(packaged_visibility_reminder(
        "packaged observation reminder",
    ));
    if has_local_pending {
        return;
    }
    commands.push("after the public DropSquash.dmg exists: rerun the packaged-app rows against that Artifact URL, then change `Packaged macOS manual QA` from `Blocked` to `Verified` in docs/release-blockers.md".to_string());
    commands.push("during that rerun: use the sample-link and installed-app helper commands emitted by manual-qa-ready-local-proof before chooser or mounted-DMG checks".to_string());
}

pub(super) fn license(commands: &mut Vec<String>, manual_qa_path: &Path, manual_path: &str) {
    commands.push("license sandbox runbook: docs/license-sandbox-runbook.md (see `Short Execution Memo` for the fast path)".to_string());
    commands.push("manual beta license issuance: docs/manual-beta-license-issuance.md".to_string());
    commands.push(hints::license_browser_sign_in_checkpoint());
    commands.push(hints::license_browser_sign_in_success());
    commands.push("proof map: Lemon Squeezy product setup / sandbox purchase / valid sandbox activation -> manual-qa-license-rerun -> docs/license-sandbox-runbook.md -> record in docs/manual-qa.md".to_string());
    if let Some(command) = hints::isolated_prepare(manual_qa_path) {
        commands.push(command);
    }
    commands.push(hints::license_rerun(manual_path));
    commands.push("after manual-qa-license-rerun: use `next sandbox markdown rows command`, then follow `next sandbox row candidates` and `sandbox quickstart 1..4`".to_string());
    commands.push(hints::license_activation_loop());
    commands.push(format!(
        "cargo run -p xtask -- manual-qa-pending {manual_path} --section license"
    ));
    commands.push("license sandbox step 1 -> fill Sandbox product setup".to_string());
    commands.push("license sandbox step 2 -> complete Sandbox purchase with a concrete test buyer order id or order number".to_string());
    commands.push(
        "license sandbox step 3 -> run Valid sandbox activation and inspect the cache".to_string(),
    );
}

pub(super) fn distribution(commands: &mut Vec<String>, manual_path: &str) {
    commands.push("signed DMG runbook: docs/signed-dmg-runbook.md (see `Short Execution Memo` for the fast path)".to_string());
    commands.push("distribution signing environment note: local macOS needs APPLE_SIGNING_IDENTITY or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD; notarization needs APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID; CI signing also needs APPLE_KEYCHAIN_PASSWORD and APPLE_CODESIGN_IDENTITY".to_string());
    commands.push("proof map: Signed DMG / notarized and stapled DMG / Gatekeeper clean-machine open -> manual-qa-distribution-rerun -> docs/signed-dmg-runbook.md -> record in Release notes and docs/manual-qa.md".to_string());
    commands.push(hints::distribution_handoff());
    commands.push(hints::distribution_rerun(manual_path));
    commands.push("after manual-qa-distribution-rerun: run `manual-qa-pending --section distribution | rg 'distribution .*markdown row:'`, then follow `distribution quickstart 1..7` before copying pending distribution markdown rows".to_string());
    commands.push(format!(
        "cargo run -p xtask -- manual-qa-pending {manual_path} --section distribution"
    ));
}
