use std::path::Path;

const REQUIRED_SECTIONS: &[(&str, &[&str])] = &[
    (
        "build",
        &[
            "cargo fmt --all -- --check",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "cargo test --workspace",
            "nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
            "cargo run -p xtask -- manual-qa-prepare",
            "--input-sample-set",
            "cargo run -p xtask -- manual-qa-ready-all",
            "cargo run -p xtask -- manual-qa-ready-local-proof",
            "cargo run -p xtask -- manual-qa-license-rerun",
            "cargo run -p xtask -- manual-qa-distribution-rerun",
            "--section local-proof",
            "--section license",
            "--section distribution",
            "cargo run -p xtask -- manual-qa-prepare --restore-state",
            "cargo run -p xtask -- manual-qa-check",
            "cargo run -p xtask -- benchmark-csv-check",
            "/tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv",
            "cargo run -p xtask -- normalize-dmg target/release/bundle/dmg",
            "cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS",
            "cargo run -p dropsquash -- license status",
            "unsigned DMG as a QA artifact",
        ],
    ),
    (
        "security",
        &[
            "No signing secrets belong in the repository",
            "`ffmpeg`, `ffprobe`, shell",
            "do not upload media files",
            "do not enable telemetry by default",
            "cargo run -p xtask -- media-policy-check",
            "cargo run -p xtask -- privacy-policy-check",
            "cargo run -p xtask -- artifact-check path/to/DropSquash.dmg",
        ],
    ),
    (
        "macos",
        &[
            "first public beta target",
            "Developer ID Application",
            "notarization credentials",
            "signed, notarized, stapled, checked, and checksummed",
            "Gatekeeper no-warning evidence",
            "cargo run -p xtask -- macos-signing-check",
        ],
    ),
    (
        "store",
        &[
            "docs/release-blockers.md",
            "Lemon Squeezy product setup",
            "license keys enabled",
            "test purchase",
            "Live checkout URL",
            "refund policy",
            "Lemon Squeezy sandbox purchase",
            "Valid sandbox activation",
            "Empty key activation",
            "Invalid key activation",
            "License network failure",
            "Expired license refresh",
            "Local license forget",
        ],
    ),
    (
        "homebrew",
        &[
            "cargo run -p xtask -- homebrew-cask 0.1.0",
            "cargo run -p xtask -- homebrew-cask-check",
            "brew install --cask",
            "brew uninstall --cask",
            "auto_updates false",
            "zap",
        ],
    ),
    (
        "release notes",
        &[
            "cargo run -p xtask -- release-notes-check",
            "cargo run -p xtask -- github-release-plan",
            "cargo run -p xtask -- publish-check",
            "Benchmark sample set",
            "20% regression threshold",
            "Expired license refresh",
            "Homebrew evidence",
        ],
    ),
];

pub(super) fn check(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = missing_requirements(&text);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} is missing release checklist coverage: {}",
        path.display(),
        missing.join(", ")
    ))
}

fn missing_requirements(text: &str) -> Vec<String> {
    REQUIRED_SECTIONS
        .iter()
        .flat_map(|(section, needles)| missing_section_items(text, section, needles))
        .collect()
}

fn missing_section_items(text: &str, section: &str, needles: &[&str]) -> Vec<String> {
    needles
        .iter()
        .filter(|needle| !text.contains(**needle))
        .map(|needle| format!("{section}: {needle}"))
        .collect()
}

#[cfg(test)]
mod tests;
