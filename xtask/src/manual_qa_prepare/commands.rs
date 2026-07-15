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
    pending_section_command(path, "packaged-app")
}

pub(super) fn pending_local_proof_command(path: &Path) -> String {
    pending_section_command(path, "local-proof")
}

pub(super) fn pending_license_command(path: &Path) -> String {
    pending_section_command(path, "license")
}

pub(super) fn pending_benchmark_command(path: &Path) -> String {
    pending_section_command(path, "benchmark")
}

pub(super) fn pending_distribution_command(path: &Path) -> String {
    pending_section_command(path, "distribution")
}

pub(super) fn fill_release_gates_command(path: &Path) -> String {
    quoted_command(path, "manual-qa-fill-release-gates")
}

pub(super) fn fill_local_proof_command(path: &Path, csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-fill-local-proof '{}' '{}'",
        shell_single_quote(path),
        shell_single_quote(csv)
    )
}

pub(super) fn ready_local_proof_command(path: &Path, csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-ready-local-proof '{}' '{}'",
        shell_single_quote(path),
        shell_single_quote(csv)
    )
}

pub(super) fn ready_all_command(path: &Path, csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-ready-all '{}' '{}'",
        shell_single_quote(path),
        shell_single_quote(csv)
    )
}

pub(super) fn ready_license_command(path: &Path) -> String {
    quoted_command(path, "manual-qa-ready-license")
}

pub(super) fn ready_distribution_command(path: &Path) -> String {
    quoted_command(path, "manual-qa-ready-distribution")
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
    quoted_command(path, "manual-qa-clean-draft")
}

pub(super) fn fill_check_command(path: &Path) -> String {
    quoted_command(path, "manual-qa-fill-check")
}

fn pending_section_command(path: &Path, section: &str) -> String {
    format!(
        "cargo run -p xtask -- manual-qa-pending '{}' --section {section}",
        shell_single_quote(path)
    )
}

fn quoted_command(path: &Path, subcommand: &str) -> String {
    format!(
        "cargo run -p xtask -- {subcommand} '{}'",
        shell_single_quote(path)
    )
}

fn shell_single_quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}
