use std::path::Path;

use super::options::Options;

pub(super) fn restore_command(options: &Options) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-prepare --restore-state --app-state-dir '{}' --state-dir '{}'",
        shell_single_quote(&options.app_state_dir),
        shell_single_quote(&options.state_dir)
    )
}

pub(super) fn open_dmg_command(path: &Path) -> String {
    format!("open -- '{}'", shell_single_quote(path))
}

pub(super) fn manual_check_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-check '{}'",
        shell_single_quote(path)
    )
}

pub(super) fn pending_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}'",
        shell_single_quote(path)
    )
}

pub(super) fn pending_packaged_app_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}' --section packaged-app",
        shell_single_quote(path)
    )
}

pub(super) fn fill_release_gates_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-fill-release-gates '{}'",
        shell_single_quote(path)
    )
}

pub(super) fn fill_benchmark_command(path: &Path, csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-fill-benchmark '{}' '{}'",
        shell_single_quote(path),
        shell_single_quote(csv)
    )
}

pub(super) fn fill_benchmark_threshold_command(path: &Path, csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-fill-benchmark-threshold '{}' '{}'",
        shell_single_quote(path),
        shell_single_quote(csv)
    )
}

pub(super) fn clean_draft_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-clean-draft '{}'",
        shell_single_quote(path)
    )
}

pub(super) fn fill_check_command(path: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-fill-check '{}'",
        shell_single_quote(path)
    )
}

fn shell_single_quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}
