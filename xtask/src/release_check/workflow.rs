use std::path::Path;

const CI_WORKFLOW: &str = ".github/workflows/ci.yml";
const RELEASE_WORKFLOW: &str = ".github/workflows/release.yml";
const SECURITY_WORKFLOW: &str = ".github/workflows/security.yml";

const CI_WORKFLOW_GATES: [&str; 8] = [
    "cargo fmt --all -- --check",
    "cargo run -p xtask -- file-size-check",
    "cargo run -p xtask -- website-check",
    "cargo run -p xtask -- release-check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cachix/install-nix-action@v31",
    "nix flake check --no-build --all-systems",
];

const RELEASE_WORKFLOW_GATES: [&str; 16] = [
    "components: rustfmt, clippy",
    "cargo fmt --all -- --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo run -p xtask -- file-size-check",
    "cargo run -p xtask -- website-check",
    "cargo run -p xtask -- manual-qa-check",
    "cargo run -p xtask -- release-check",
    "pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
    "cargo run -p xtask -- artifact-check target/release/bundle/dmg/*.dmg",
    "dropsquash-unsigned-dmg",
    "cargo run -p xtask -- checksum target/release/bundle/dmg/*.dmg > SHA256SUMS",
    "actions/upload-artifact@v4",
    "dropsquash-unsigned-dmg-checksum",
    "cargo run -p xtask -- macos-signing-check",
    "Block unsigned Phase 0 release",
];

const SECURITY_WORKFLOW_GATES: [&str; 4] = [
    "cargo audit",
    "cargo deny check",
    "cargo run -p xtask -- media-policy-check",
    "cargo run -p xtask -- privacy-policy-check",
];

pub(super) fn check_all() -> Result<(), String> {
    check_workflow(CI_WORKFLOW, "CI", &CI_WORKFLOW_GATES)?;
    check_workflow(SECURITY_WORKFLOW, "security", &SECURITY_WORKFLOW_GATES)?;
    check_workflow(RELEASE_WORKFLOW, "release", &RELEASE_WORKFLOW_GATES)
}

fn check_workflow(path: &str, label: &str, gates: &[&'static str]) -> Result<(), String> {
    let text = std::fs::read_to_string(Path::new(path)).map_err(|error| error.to_string())?;
    let missing = missing_workflow_gates(&text, gates);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{path} is missing {label} gates: {}",
        missing.join(", ")
    ))
}

fn missing_workflow_gates(text: &str, gates: &[&'static str]) -> Vec<&'static str> {
    gates
        .iter()
        .copied()
        .filter(|gate| !text.contains(gate))
        .collect()
}

#[cfg(test)]
pub(super) fn missing_ci_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, &CI_WORKFLOW_GATES)
}

#[cfg(test)]
pub(super) fn missing_release_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, &RELEASE_WORKFLOW_GATES)
}

#[cfg(test)]
pub(super) fn missing_security_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, &SECURITY_WORKFLOW_GATES)
}
