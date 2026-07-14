use std::path::Path;

use super::commands;

pub(super) fn fill_release_gates_line(path: &Path) -> String {
    format!(
        "manual QA Fill release gates command: {}",
        commands::fill_release_gates_command(path)
    )
}

pub(super) fn fill_benchmark_line(path: &Path, output_dir: &Path) -> String {
    format!(
        "manual QA Fill benchmark command: {}",
        commands::fill_benchmark_command(path, &output_dir.join("benchmark-results.csv"))
    )
}

pub(super) fn fill_benchmark_threshold_line(path: &Path, output_dir: &Path) -> String {
    format!(
        "manual QA Fill benchmark threshold command: {}",
        commands::fill_benchmark_threshold_command(path, &output_dir.join("benchmark-results.csv"))
    )
}

pub(super) fn clean_draft_line(path: &Path) -> String {
    format!(
        "manual QA Clean draft command: {}",
        commands::clean_draft_command(path)
    )
}

pub(super) fn fill_check_line(path: &Path) -> String {
    format!(
        "manual QA Fill check command: {}",
        commands::fill_check_command(path)
    )
}

pub(super) fn pending_line(path: &Path) -> String {
    format!(
        "manual QA Pending command: {}",
        commands::pending_command(path)
    )
}

pub(super) fn print_helper_commands(path: &Path, output_dir: &Path) {
    println!("{}", fill_release_gates_line(path));
    println!("{}", fill_benchmark_line(path, output_dir));
    println!("{}", fill_benchmark_threshold_line(path, output_dir));
    println!("{}", clean_draft_line(path));
    println!("{}", pending_line(path));
    println!("{}", fill_check_line(path));
}
