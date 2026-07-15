use std::path::Path;

use super::commands;

pub(super) fn block(output_path: &Path, csv_path: &Path) -> String {
    format!(
        "Prepared draft helper commands (run them from the same clean worktree that produced the App build above):\n\n```sh\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n```",
        commands::fill_release_gates_command(output_path),
        commands::fill_local_proof_command(output_path, csv_path),
        commands::ready_local_proof_command(output_path, csv_path),
        commands::ready_all_command(output_path, csv_path),
        commands::ready_license_command(output_path),
        commands::ready_distribution_command(output_path),
        commands::fill_benchmark_command(output_path, csv_path),
        commands::fill_benchmark_threshold_command(output_path, csv_path),
        commands::clean_draft_command(output_path),
        commands::pending_command(output_path),
        commands::pending_packaged_app_command(output_path),
        commands::pending_local_proof_command(output_path),
        commands::pending_license_command(output_path),
        commands::pending_benchmark_command(output_path),
        commands::pending_distribution_command(output_path),
        commands::fill_check_command(output_path)
    )
}
