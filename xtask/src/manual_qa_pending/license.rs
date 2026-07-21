mod reminders;

use super::license_commands::{
    forget_command, forget_helper_command, launch_app_command, seed_cache_command, status_command,
};
use super::{fields, license_candidates};
const FRESH_BUILD: &str =
    "CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build";
const FRESH_APP: &str = "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app";
pub(super) fn extra_lines(text: &str, groups: &[(&str, Vec<(String, String)>)]) -> Vec<String> {
    let Some(rows) = groups
        .iter()
        .find(|(name, _)| *name == "License Sandbox")
        .map(|(_, rows)| rows)
    else {
        return Vec::new();
    };
    let mut lines = Vec::new();
    lines.push(format!("license fresh build command: {FRESH_BUILD}"));
    lines.push(format!("license fresh app artifact: {FRESH_APP}"));
    if let Some(path) = fields::field_value(text, "License cache path") {
        lines.push(format!("license cache path: {path}"));
        lines.push(format!(
            "license cache inspect command: sed -n '1,160p' \"{path}\""
        ));
        lines.push(format!(
            "license cache helper command: cargo run -p xtask -- manual-qa-license-cache '{}'",
            path.replace('\'', "'\\''")
        ));
        lines.push(format!(
            "license seed valid cache command: {}",
            seed_cache_command(path, false)
        ));
        lines.push(format!(
            "license seed expired cache command: {}",
            seed_cache_command(path, true)
        ));
        lines.push(format!(
            "license forget helper command: {}",
            forget_helper_command(text, path)
        ));
    }
    if let (Some(artifact), Some(config)) = (
        fields::field_value(text, "App artifact"),
        fields::field_value(text, "Config path"),
    ) {
        if artifact.ends_with(".dmg") {
            lines.push(
                "license launch note: App artifact is a DMG; prefer the fresh app launch commands below for sandbox activation loops and use the mounted DMG launch only when you intentionally need mounted-DMG behavior"
                    .to_string(),
            );
        }
        lines.push(format!(
            "license launch app command: {}",
            launch_app_command(artifact, config, None)
        ));
        lines.push(format!(
            "license network failure launch command: {}",
            launch_app_command(artifact, config, Some("http://127.0.0.1:9/v1/licenses"))
        ));
        lines.push(format!(
            "license fresh app launch command: {}",
            launch_app_command(FRESH_APP, config, None)
        ));
        lines.push(format!(
            "license fresh app network failure launch command: {}",
            launch_app_command(FRESH_APP, config, Some("http://127.0.0.1:9/v1/licenses"))
        ));
    }
    lines.push(format!(
        "license diagnostics command: {}",
        status_command(text)
    ));
    lines.push(format!("license forget command: {}", forget_command(text)));
    if reminders::needs_status_reminder(rows) {
        lines.push(format!(
            "license activation before-state command: {}",
            status_command(text)
        ));
        lines.push(format!(
            "license diagnostics after UI action: {}",
            status_command(text)
        ));
        lines.push(
            "license activation evidence reminder: failed activation rows should still show raw license key persisted: no, fingerprint missing, and instance_id missing"
                .to_string(),
        );
        lines.push(
            "license valid activation evidence reminder: successful activation should show license state: Pro plus fingerprint present 64-character lowercase hex and instance_id present"
                .to_string(),
        );
        lines.extend(license_candidates::lines().into_iter().map(str::to_string));
    }
    lines.extend(reminders::lines(rows));
    if reminders::has_forget_row(rows) {
        lines.push(format!(
            "license forget before-state command: {}",
            status_command(text)
        ));
        if let Some(path) = fields::field_value(text, "License cache path") {
            lines.push(format!(
                "license cache removed check: test ! -e \"{path}\" && echo removed || echo still-present"
            ));
        }
        lines.push(format!(
            "license forget after-state command: {}",
            status_command(text)
        ));
        lines.push(
            "license forget evidence reminder: record the Forgetting/disabled UI state, confirm cache removal, then record whether the app returned to trial or stayed locked"
                .to_string(),
        );
        lines.push(
            "license forget markdown row helper: run the forget helper above and copy its `manual QA markdown row:` output into docs/manual-qa.md"
                .to_string(),
        );
    }
    lines
}
