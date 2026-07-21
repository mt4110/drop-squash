pub(super) fn lines(groups: &[(&str, Vec<(String, String)>)]) -> Vec<String> {
    let mut lines = Vec::new();
    if needs_runbook(groups) {
        lines.push("distribution runbook: docs/signed-dmg-runbook.md".to_string());
    }
    if needs_next_steps(groups) {
        lines.push(
            "distribution next step 1: run macos-signing-check in the release environment"
                .to_string(),
        );
        lines.push(
            "distribution next step 2: sign the fresh DropSquash.dmg and verify codesign"
                .to_string(),
        );
        lines.push(
            "distribution next step 3: notarize, staple, then record the Gatekeeper open test against the public Artifact URL"
                .to_string(),
        );
    }
    if has_label(groups, "`cargo run -p xtask -- macos-signing-check`") {
        lines.push(
            "distribution signing reminder: load release signing secrets first, then record the row only after macos-signing-check passes"
                .to_string(),
        );
        lines.push(
            "distribution signing local options: APPLE_SIGNING_IDENTITY, or APPLE_CERTIFICATE with APPLE_CERTIFICATE_PASSWORD"
                .to_string(),
        );
        lines.push(
            "distribution notarization local options: APPLE_API_KEY/APPLE_API_ISSUER/APPLE_API_KEY_PATH, or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID"
                .to_string(),
        );
        lines.push(
            "distribution credential blocker note: if macos-signing-check fails here, treat it as release-credential setup work, not a product-code failure"
                .to_string(),
        );
        lines.push(
            "distribution isolated target hint: if xtask verification stalls in the everyday target tree, rerun signing helpers with CARGO_TARGET_DIR=/tmp/dsq-xtask-target"
                .to_string(),
        );
    }
    if has_label(groups, "Codesign verification") {
        lines.push(
            "distribution codesign reminder: verify the signed public DropSquash.dmg or mounted app with Developer ID evidence before filling the row"
                .to_string(),
        );
    }
    if has_label(groups, "Notarization staple verification") {
        lines.push(
            "distribution notarization reminder: wait for notary acceptance, then run stapler validate and spctl before recording the row"
                .to_string(),
        );
    }
    if has_label(groups, "Gatekeeper open test") {
        lines.push(
            "distribution Gatekeeper reminder: use the signed public DropSquash.dmg matching release notes and verify a clean open in a fresh macOS account"
                .to_string(),
        );
    }
    lines
}

fn needs_runbook(groups: &[(&str, Vec<(String, String)>)]) -> bool {
    needs_next_steps(groups) || has_label(groups, "Gatekeeper open test")
}

fn needs_next_steps(groups: &[(&str, Vec<(String, String)>)]) -> bool {
    has_label(groups, "`cargo run -p xtask -- macos-signing-check`")
        || has_label(groups, "Codesign verification")
        || has_label(groups, "Notarization staple verification")
        || has_label(groups, "Gatekeeper open test")
}

fn has_label(groups: &[(&str, Vec<(String, String)>)], expected: &str) -> bool {
    groups
        .iter()
        .flat_map(|(_, rows)| rows.iter())
        .any(|(label, _)| label == expected)
}

#[cfg(test)]
mod tests;
