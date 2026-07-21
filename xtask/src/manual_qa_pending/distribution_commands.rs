pub(super) fn artifact_check(artifact: &str) -> String {
    format!("cargo run -p xtask -- artifact-check '{artifact}'")
}

pub(super) fn checksum(artifact: &str, output: &str) -> String {
    format!("cargo run -p xtask -- checksum '{artifact}' --output '{output}'")
}

pub(super) fn signing_plan(artifact: &str) -> String {
    format!(
        "cargo run -p xtask -- macos-signing-plan '{artifact}' '/tmp/dropsquash-signed-release'"
    )
}

pub(super) fn codesign_verify(artifact: &str) -> String {
    format!("cargo run -p xtask -- macos-codesign-verify-plan '{artifact}'")
}

pub(super) fn stapler(artifact: &str) -> String {
    format!("cargo run -p xtask -- macos-stapler-plan '{artifact}'")
}

pub(super) fn spctl(artifact: &str) -> String {
    format!("cargo run -p xtask -- macos-spctl-plan '{artifact}'")
}
