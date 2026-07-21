mod reminders;

use super::distribution_candidates;
use super::distribution_commands::{
    artifact_check, checksum, codesign_verify, signing_plan, spctl, stapler,
};
use super::fields;
const FRESH_BUILD: &str =
    "CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build";
const FRESH_DMG_DIR: &str = "/tmp/dsq-build-target/release/bundle/dmg";
const FRESH_DMG: &str = "/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg";
const SIGNED_OUTPUT_DIR: &str = "/tmp/dropsquash-signed-release";
const SIGNED_DMG: &str = "/tmp/dropsquash-signed-release/DropSquash.dmg";

pub(super) fn extra_lines(text: &str, groups: &[(&str, Vec<(String, String)>)]) -> Vec<String> {
    if !groups
        .iter()
        .any(|(name, _)| *name == "Distribution And Signing")
    {
        return Vec::new();
    }
    let mut lines = Vec::new();
    lines.push(format!("distribution fresh build command: {FRESH_BUILD}"));
    lines.push(format!(
        "distribution fresh normalize command: cargo run -p xtask -- normalize-dmg {FRESH_DMG_DIR}"
    ));
    lines.push(format!("distribution fresh dmg artifact: {FRESH_DMG}"));
    lines.push(format!(
        "distribution fresh artifact check: {}",
        artifact_check(FRESH_DMG)
    ));
    lines.push(format!(
        "distribution fresh signing plan: {}",
        signing_plan(FRESH_DMG)
    ));
    lines.push(format!(
        "distribution fresh codesign verify plan: {}",
        codesign_verify(SIGNED_DMG)
    ));
    lines.push(format!(
        "distribution fresh stapler plan: {}",
        stapler(SIGNED_DMG)
    ));
    lines.push(format!(
        "distribution fresh spctl plan: {}",
        spctl(SIGNED_DMG)
    ));
    lines.push(format!(
        "distribution fresh checksum command: {}",
        checksum(SIGNED_DMG, &format!("{SIGNED_OUTPUT_DIR}/SHA256SUMS"))
    ));
    if let Some(artifact) = fields::field_value(text, "App artifact") {
        if artifact.contains("/target/release/bundle/dmg/") {
            lines.push(
                "distribution artifact note: App artifact is the local unsigned QA DMG; use these plan/check commands as preflight, then record the final rows against the signed public DropSquash.dmg and its Artifact URL"
                    .to_string(),
            );
        }
        lines.push(format!(
            "distribution artifact check: {}",
            artifact_check(artifact)
        ));
        lines.push(format!(
            "distribution signing plan: {}",
            signing_plan(artifact)
        ));
        lines.push(format!(
            "distribution codesign verify plan: {}",
            codesign_verify(artifact)
        ));
        lines.push(format!("distribution stapler plan: {}", stapler(artifact)));
        lines.push(format!("distribution spctl plan: {}", spctl(artifact)));
    }
    if let (Some(artifact), Some(output)) = (
        fields::field_value(text, "App artifact"),
        fields::field_value(text, "Output folder"),
    ) {
        lines.push(format!(
            "distribution checksum command: {}",
            checksum(artifact, &format!("{output}/SHA256SUMS"))
        ));
    }
    lines.push("distribution homebrew release notes path: replace path/to/release-notes.md with the public release notes markdown before recording that row".to_string());
    lines.push("distribution Gatekeeper note: record the exact public DropSquash.dmg Artifact URL in the Gatekeeper row once the signed artifact is published".to_string());
    lines.extend(reminders::lines(groups));
    lines.extend(
        distribution_candidates::lines()
            .into_iter()
            .map(str::to_string),
    );
    lines
}
