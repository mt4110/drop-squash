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

fn shell_single_quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}
