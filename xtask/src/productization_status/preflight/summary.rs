use crate::productization_status::{model::TrackStatus, scope::Scope};

pub(crate) fn summary(track: &TrackStatus, scope: &Scope, scoped: bool) -> Option<String> {
    let remaining = track
        .remaining
        .iter()
        .filter(|blocker| !scoped || !scope.is_deferred(blocker))
        .collect::<Vec<_>>();
    match track.name.as_str() {
        "Local packaged-app proof" if remaining.iter().any(|name| **name == "Benchmark release set") => Some(
            "primary command: cargo run -p xtask -- manual-qa-pending /tmp/dropsquash-manual-qa-prepared-<app-build>.md --section local-proof".to_string(),
        ),
        "Local packaged-app proof" => Some(
            "primary command: cargo run -p xtask -- manual-qa-packaged-rerun".to_string(),
        ),
        "License sandbox proof" => Some(
            "primary command: cargo run -p xtask -- manual-qa-license-rerun (or pass /tmp/dropsquash-manual-qa-prepared-<app-build>.md)".to_string(),
        ),
        "Public web proof" => Some(
            "primary command: cargo run -p xtask -- public-web-ready; public-web-rerun stays available as the operator memo, and publish-check remains the final public-release gate after production URLs, release notes, and blocker evidence exist".to_string(),
        ),
        "Signing and distribution proof" => Some(
            "primary command: cargo run -p xtask -- manual-qa-distribution-rerun (or pass /tmp/dropsquash-manual-qa-prepared-<app-build>.md); distribution snapshot handoff stays available via scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)".to_string(),
        ),
        _ => None,
    }
}
