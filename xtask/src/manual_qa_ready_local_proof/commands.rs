use std::path::Path;

use super::shell_single_quote;

pub(super) fn fresh_build_command() -> &'static str {
    "CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build"
}

pub(super) fn fresh_artifact_path() -> &'static str {
    "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app"
}

pub(super) fn sample_link_command(csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-link-samples '{}' /tmp/dropsquash-qa-open-panel",
        shell_single_quote(csv)
    )
}

pub(super) fn installed_app_status_command() -> &'static str {
    "cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app"
}

pub(super) fn installed_app_stash_command() -> &'static str {
    "cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app"
}

pub(super) fn installed_app_restore_command() -> &'static str {
    "cargo run -p xtask -- manual-qa-installed-app restore /tmp/dropsquash-manual-qa-installed-app"
}

pub(super) fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        match cells.as_slice() {
            [found, value] if *found == label => Some(*value),
            _ => None,
        }
    })
}

pub(super) fn packaged_app_pending_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}' --section packaged-app",
        shell_single_quote(path)
    )
}

pub(super) fn manual_check_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-check '{}'",
        shell_single_quote(path)
    )
}

pub(super) fn fresh_not_smaller_command(text: &str, csv: &Path) -> Result<Option<String>, String> {
    let Some(config) = field_value(text, "Config path") else {
        return Ok(None);
    };
    let rows = crate::benchmark_csv_check::read_rows(csv)?;
    let sample = rows[1]
        .get(2)
        .ok_or_else(|| "benchmark CSV row is missing output column: small".to_string())?;
    Ok(Some(format!(
        "cargo run -p xtask -- manual-qa-launch-app --settle-seconds 9 --open-file '{}' '{}' '{}'",
        sample,
        fresh_artifact_path(),
        config.replace('\'', "'\\''")
    )))
}
