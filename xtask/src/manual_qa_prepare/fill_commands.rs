use std::path::Path;

use super::{commands, options::Options};

pub(super) fn block(options: &Options, output_path: &Path) -> String {
    format!(
        "Prepared draft helper commands:\n\n```sh\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n```",
        commands::fill_release_gates_command(output_path),
        commands::fill_local_proof_command(
            output_path,
            &options.output_dir.join("benchmark-results.csv")
        ),
        commands::ready_local_proof_command(
            output_path,
            &options.output_dir.join("benchmark-results.csv")
        ),
        commands::ready_license_command(output_path),
        commands::ready_distribution_command(output_path),
        commands::fill_benchmark_command(
            output_path,
            &options.output_dir.join("benchmark-results.csv")
        ),
        commands::fill_benchmark_threshold_command(
            output_path,
            &options.output_dir.join("benchmark-results.csv")
        ),
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
